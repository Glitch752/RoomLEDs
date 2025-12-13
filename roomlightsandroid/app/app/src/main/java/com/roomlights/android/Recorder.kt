package com.roomlights.android;

import android.Manifest
import android.content.Context
import android.content.pm.PackageManager
import android.media.AudioFormat
import android.media.AudioRecord
import android.media.MediaRecorder
import android.util.Log
import androidx.core.app.ActivityCompat
import com.karlstav.cava.CavaNative
import java.util.concurrent.locks.ReentrantLock
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow

private const val FRAMERATE: Int = 80
private const val TOTAL_PIXELS: Int = 812
private const val BLOCK_SIZE: Int = 4
// The total number of visualizer blocks, which equates
// to the number of Cava bars
private const val TOTAL_BLOCKS: Int = TOTAL_PIXELS / BLOCK_SIZE + 1

private const val LOW_CUTOFF_FREQUENCY = 50
private const val HIGH_CUTOFF_FREQUENCY = 10000

private const val RECORDER_SAMPLE_RATE = 44100
private const val AUDIO_SOURCE = MediaRecorder.AudioSource.MIC
private const val CHANNEL_CONFIG = AudioFormat.CHANNEL_IN_MONO
private const val AUDIO_FORMAT = AudioFormat.ENCODING_PCM_16BIT

private val BUFFER_SIZE_RECORDING = AudioRecord.getMinBufferSize(
    RECORDER_SAMPLE_RATE, CHANNEL_CONFIG, AUDIO_FORMAT
)

private const val LOG_TAG = "room_lights_recorder"

/**
 * Manages recording input data for the visualizer, either from
 * the device microphone or system audio depending on preferences.  
 * 
 */
public class Recorder(private val context: Context) {
    private var audioRecord: AudioRecord? = null
    private var recordingThread: Thread? = null
    private var cavaThread: Thread? = null
    private var isRecording = false

    private val lock = ReentrantLock()
    val audioBuffer = ByteArray(BUFFER_SIZE_RECORDING / 2)
    val cavaInputSamples = DoubleArray(BUFFER_SIZE_RECORDING * 2)
    var newSampleCount = 0

    private val _barsFlow = MutableStateFlow(FloatArray(TOTAL_BLOCKS) { 0f })
    val barsFlow: StateFlow<FloatArray> get() = _barsFlow

    fun start() {
        if(isRecording) return

        if(
            ActivityCompat.checkSelfPermission(context, Manifest.permission.RECORD_AUDIO) !=
            PackageManager.PERMISSION_GRANTED
        ) {
            Log.e(LOG_TAG, "Recorder start called before microphone permission granted")
            return
        }

        audioRecord = AudioRecord(
            AUDIO_SOURCE,
            RECORDER_SAMPLE_RATE,
            CHANNEL_CONFIG,
            AUDIO_FORMAT,
            BUFFER_SIZE_RECORDING
        )
        if(audioRecord?.state != AudioRecord.STATE_INITIALIZED) {
            Log.e(LOG_TAG, "Error initializing AudioRecord")
            return
        }
        audioRecord?.startRecording()
        isRecording = true
        
        recordingThread = Thread { writeAudioDataToBuffer() }
        recordingThread?.start()

        CavaNative.init(
            TOTAL_BLOCKS,
            FRAMERATE,
            LOW_CUTOFF_FREQUENCY,
            HIGH_CUTOFF_FREQUENCY
        )

        cavaThread = Thread { runCavaLoop() }
        cavaThread?.start()
    }

    fun stop() {
        isRecording = false
        try {
            recordingThread?.join()
            cavaThread?.join()
        } catch(e: InterruptedException) {
            // Ignore
        }

        audioRecord?.stop()
        audioRecord?.release()
        audioRecord = null

        CavaNative.destroy()
    }

    private fun writeAudioDataToBuffer() {
        while(isRecording) {
            val read = audioRecord?.read(audioBuffer, 0, audioBuffer.size) ?: 0
            lock.lock()
            for(i in 0 until read step 2) {
                if (i > audioBuffer.size - 2) break
                val lowByte = audioBuffer[i].toInt()
                val highByte = audioBuffer[i + 1].toInt()
                val pcmValue = (highByte shl 8) or (lowByte and 0xFF)
                cavaInputSamples[newSampleCount] = pcmValue.toDouble() / 32767.0
                newSampleCount++
                if (newSampleCount > cavaInputSamples.size - 1) newSampleCount = 0
            }
            lock.unlock()
        }
    }

    private fun runCavaLoop() {
        val bars = FloatArray(TOTAL_BLOCKS)
        val frameTimeMs = 1000L / FRAMERATE
        while(isRecording) {
            val start = System.currentTimeMillis()
            
            lock.lock()
            val cavaOut = CavaNative.exec(cavaInputSamples, newSampleCount)
            lock.unlock()

            for(i in 0 until TOTAL_BLOCKS) {
                bars[i] = cavaOut.getOrNull(i)?.toFloat() ?: 0f
            }

            _barsFlow.value = bars.copyOf()
            newSampleCount = 0

            val elapsed = System.currentTimeMillis() - start
            val sleep = frameTimeMs - elapsed
            if(sleep > 0) Thread.sleep(sleep)

            // TODO: reload cava if applicable
        }
    }
}