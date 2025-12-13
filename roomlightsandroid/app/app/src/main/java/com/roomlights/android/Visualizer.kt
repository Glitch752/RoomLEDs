package com.roomlights.android

import android.annotation.SuppressLint
import java.io.InputStream
import java.nio.ByteBuffer
import java.nio.ByteOrder
import java.nio.FloatBuffer
import java.nio.charset.StandardCharsets
import javax.microedition.khronos.egl.EGLConfig
import javax.microedition.khronos.opengles.GL10
import android.opengl.GLES30
import android.content.Context
import android.opengl.GLSurfaceView
import android.util.Log
import kotlinx.coroutines.flow.StateFlow

private const val FLOAT_SIZE_BYTES = 4 // size of Float
private const val VERTICES_DATA_STRIDE_BYTES = 5 * FLOAT_SIZE_BYTES

private const val UNKNOWN_PROGRAM = -1
private const val UNKNOWN_ATTRIBUTE = -1

private const val MAX_VISUALIZER_BARS = 256

private const val LOG_TAG = "room_lights_visualizer"

@SuppressLint("ViewConstructor")
class Visualizer(
    context: Context,
    barsFlow: StateFlow<FloatArray>
) : GLSurfaceView(context) {
    private val renderer: VisualizerRenderer = VisualizerRenderer(context, barsFlow)

    init {
        if(!isInEditMode) {
            setEGLContextClientVersion(3)
            setRenderer(renderer)
        }
    }
}

class VisualizerRenderer(
    private val context: Context,
    private val barsFlow: StateFlow<FloatArray>
) : GLSurfaceView.Renderer {
    private var visualizedBars: Int
    private val visualizerBarData = FloatArray(MAX_VISUALIZER_BARS)

    private var screenWidth: Int = 0
    private var screenHeight: Int = 0
    
    // Shader buffers
    private val vertexBuffer: FloatBuffer
    private val barBuffer: FloatBuffer
    
    // Shader uniforms and attributes
    private var uniformBars = UNKNOWN_ATTRIBUTE
    private var uniformBarsCount = UNKNOWN_ATTRIBUTE
    private var uniformResolution = UNKNOWN_ATTRIBUTE
    private var inPositionHandle = UNKNOWN_ATTRIBUTE

    init {
        program = UNKNOWN_PROGRAM
        
        val mVerticesData = floatArrayOf(
        // X, Y, Z, U, V
        -1.0f, -1.0f, 0f, 0f, 0f,
        1.0f, -1.0f, 0f, 1f, 0f,
        -1.0f,  1.0f, 0f, 0f, 1f,
        1.0f,  1.0f, 0f, 1f, 1f )
        
        vertexBuffer = ByteBuffer.allocateDirect(mVerticesData.count() * FLOAT_SIZE_BYTES).order(ByteOrder.nativeOrder()).asFloatBuffer()
        vertexBuffer.put(mVerticesData).position(0)
        
        barBuffer = ByteBuffer.allocateDirect(MAX_VISUALIZER_BARS * FLOAT_SIZE_BYTES)
            .order(ByteOrder.nativeOrder()).asFloatBuffer()
        
        visualizedBars = numberOfBarsSet
    }
    
    override fun onSurfaceCreated(unused: GL10, config: EGLConfig) {
        // Set the background frame color
        val resource = context.resources
        
        val vertexShaderSource = getFileText(resource.openRawResource(R.raw.pass_through))
        val fragmentShaderSource = getFileText(resource.openRawResource(R.raw.bar_spectrum))
        
        createProgram(vertexShaderSource, fragmentShaderSource)
        checkGlError("createProgram")
        uniformBars = GLES30.glGetUniformLocation(program, "bars")
        uniformBarsCount = GLES30.glGetUniformLocation(program, "bars_count")
        uniformResolution = GLES30.glGetUniformLocation(program, "u_resolution")
        checkGlError("glGetUniformLocation")
        
        GLES30.glUseProgram(program)
        checkGlError("glUseProgram")
        
        inPositionHandle = GLES30.glGetAttribLocation(program, "position")
        GLES30.glVertexAttribPointer(inPositionHandle, 3, GLES30.GL_FLOAT, false, VERTICES_DATA_STRIDE_BYTES, vertexBuffer)
        checkGlError("glVertexAttribPointer maPosition")
        GLES30.glEnableVertexAttribArray(inPositionHandle)
        checkGlError("glVertexAttribEnable maPosition")
        
        GLES30.glUniform1i(uniformBarsCount, visualizedBars)
    }
    
    override fun onDrawFrame(unused: GL10) {
        // Redraw background color
        GLES30.glClearColor(0.0f, 0.0f, 0.0f, 0.0f)
        GLES30.glClear(GLES30.GL_COLOR_BUFFER_BIT)
        
        // Get bars from flow
        val bars = barsFlow.value

        // Because the visualized bars isn't the same as the bars from cava,
        // do a naive downsample
        val step = bars.size.toFloat() / visualizedBars.toFloat()
        for(i in 0 until visualizedBars) {
            val index = (i * step).toInt()
            visualizerBarData[i] = bars.getOrNull(index) ?: 0f
        }
        
        // Draw
        barBuffer.put(visualizerBarData).position(0)
        
        GLES30.glUniform4fv(uniformBars, visualizedBars, barBuffer)
        checkGlError("glGetUniform4fv uniform_bars")
        
        GLES30.glDrawArrays(GLES30.GL_TRIANGLE_STRIP, 0, 4)
        checkGlError("glDrawArrays")
        
        if(configReload) {
            visualizedBars = numberOfBarsSet

            GLES30.glUniform1i(uniformBarsCount, visualizedBars)
            for(i in 0 until visualizedBars) {
                visualizerBarData[i] = 0.0.toFloat()
            }
            
            configReload = false
        }
    }
    
    override fun onSurfaceChanged(unused: GL10, width: Int, height: Int) {
        GLES30.glViewport(0, 0, width, height)
        GLES30.glUniform3f(uniformResolution, width.toFloat(), height.toFloat(), 0.0f)
        checkGlError("glUniform3f uniform_u_resolution")
        screenWidth = width
        screenHeight = height
    }
    
    private fun floatColor(color: Int): Float {
        return color.toFloat() / 255.toFloat()
    }
}

private var program = UNKNOWN_PROGRAM
private fun getFileText(inputStream: InputStream):String {
    val bytes = inputStream.readBytes() // See below
    val text = String(bytes, StandardCharsets.UTF_8)
    inputStream.close()
    return text
}
private fun createProgram(vertexSource: String, fragmentSource: String): Boolean {
    if(program != UNKNOWN_PROGRAM) {
        // Delete program
        GLES30.glDeleteProgram(program)
        checkGlError("glAttachShader: delete program")
        program = UNKNOWN_PROGRAM
    }
    
    // Load vertex shader
    val vertexShader = loadShader(GLES30.GL_VERTEX_SHADER, vertexSource)
    if(vertexShader == UNKNOWN_PROGRAM) {
        return false
    }
    
    // Load fragment shader
    val pixelShader = loadShader(GLES30.GL_FRAGMENT_SHADER, fragmentSource)
    if(pixelShader == UNKNOWN_PROGRAM) {
        return false
    }
    
    program = GLES30.glCreateProgram()
    if(program != UNKNOWN_PROGRAM) {
        GLES30.glAttachShader(program, vertexShader)
        checkGlError("glAttachShader: vertex")
        GLES30.glAttachShader(program, pixelShader)
        checkGlError("glAttachShader: pixel")
        return linkProgram()
    }
    
    return true
}

private fun linkProgram(): Boolean {
    if(program == UNKNOWN_PROGRAM) return false
    
    GLES30.glLinkProgram(program)
    val linkStatus = IntArray(1)
    GLES30.glGetProgramiv(program, GLES30.GL_LINK_STATUS, linkStatus, 0)
    
    if(linkStatus[0] != GLES30.GL_TRUE) {
        Log.e(LOG_TAG, "Could not link program: ")
        Log.e(LOG_TAG, GLES30.glGetProgramInfoLog(program))
        GLES30.glDeleteProgram(program)
        program = UNKNOWN_PROGRAM
        return false
    }
    
    return true
}

private fun loadShader(shaderType: Int, source: String): Int {
    var shader = GLES30.glCreateShader(shaderType)
    
    if(shader != UNKNOWN_PROGRAM) {
        GLES30.glShaderSource(shader, source)
        GLES30.glCompileShader(shader)
        val compiled = IntArray(1)
        GLES30.glGetShaderiv(shader, GLES30.GL_COMPILE_STATUS, compiled, 0)
        
        if(compiled[0] == UNKNOWN_PROGRAM) {
            Log.e(LOG_TAG, "Could not compile shader $shaderType:")
            Log.e(LOG_TAG, GLES30.glGetShaderInfoLog(shader))
            GLES30.glDeleteShader(shader)
            shader = UNKNOWN_PROGRAM
        }
    }
    
    return shader
}

private fun checkGlError(op: String) {
    var error: Int
    
    while(GLES30.glGetError().also { error = it } != GLES30.GL_NO_ERROR) {
        Log.e(LOG_TAG, "$op: glError $error")
        throw RuntimeException("$op: glError $error")
    }
}