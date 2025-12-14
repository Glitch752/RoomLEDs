package com.roomlights.android.recorder

import android.util.Log
import com.karlstav.cava.CavaNative
import com.roomlights.android.RoomLightsApplication
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch

const val RECORDER_SAMPLE_RATE = 44100

private const val FRAMERATE: Int = 80
private const val TOTAL_PIXELS: Int = 812
private const val BLOCK_SIZE: Int = 4
// The total number of visualizer blocks, which equates
// to the number of Cava bars
private const val TOTAL_BLOCKS: Int = TOTAL_PIXELS / BLOCK_SIZE + 1

private const val CHANNELS: Int = 2
private const val BLOCKS_PER_CHANNEL: Int = TOTAL_BLOCKS / CHANNELS

private const val LOW_CUTOFF_FREQUENCY = 50
private const val HIGH_CUTOFF_FREQUENCY = 10000
private const val NOISE_REDUCTION = 0.7f

private const val LOG_TAG = "room_lights_recorder"

/**
 * Manages recording input data for the visualizer, either from
 * the device microphone or system audio depending on preferences.  
 */
class Recorder(
    private val app: RoomLightsApplication
) {
    private var cavaThread: Thread? = null
    private var isRecording = false

    private var activeHandler: RecordingHandler? = null
    private var currentData: RecordingData? = null

    private val _barsFlow = MutableStateFlow(FloatArray(TOTAL_BLOCKS))
    val barsFlow: StateFlow<FloatArray> get() = _barsFlow

    init {
        app.appScope.launch {
            app.settings.recordingType.collect { recordingType ->
                if(isRecording) {
                    stop()
                }

                activeHandler?.release()

                activeHandler = when(recordingType) {
                    RecordingType.MICROPHONE -> MicrophoneRecordingHandler(app)
                    RecordingType.SYSTEM_AUDIO -> SystemAudioRecordingHandler(app)
                }

                start()
            }
        }
    }

    fun start() {
        if(isRecording) return

        if(!(activeHandler?.checkPermissions() ?: false)) {
            return
        }

        isRecording = true

        CavaNative.init(
            BLOCKS_PER_CHANNEL,
            RECORDER_SAMPLE_RATE,
            CHANNELS,
            true,
            NOISE_REDUCTION,
            LOW_CUTOFF_FREQUENCY,
            HIGH_CUTOFF_FREQUENCY
        )

        currentData = activeHandler?.startRecording()

        cavaThread = Thread { runCavaLoop() }
        cavaThread?.start()
    }

    fun stop() {
        if(!isRecording) return
        isRecording = false

        try {
            cavaThread?.join()
        } catch (e: InterruptedException) {
            Log.e(LOG_TAG, "Error stopping Cava thread: ${e.message}")
        }
        cavaThread = null

        activeHandler?.stopRecording()
        currentData = null

        CavaNative.destroy()
    }

    private fun runCavaLoop() {
        val bars = FloatArray(TOTAL_BLOCKS)
        val frameTimeMs = 1000L / FRAMERATE
        while(isRecording) {
            val start = System.currentTimeMillis()

            val data = currentData ?: break

            data.lock.lock()
            val cavaOut = CavaNative.exec(data.getInterleavedCavaBuffer(CHANNELS), data.newSampleCount)
            data.newSampleCount = 0
            data.lock.unlock()

            if(CHANNELS == 2) {
                // If using 2 channels, reverse the second half of the output
                for(i in 0 until BLOCKS_PER_CHANNEL) {
                    bars[i] = cavaOut.getOrNull(i)?.toFloat() ?: 0f
                    bars[i + BLOCKS_PER_CHANNEL] =
                        cavaOut.getOrNull(TOTAL_BLOCKS - 1 - i)?.toFloat() ?: 0f
                }
            } else {
                for(i in 0 until TOTAL_BLOCKS) {
                    bars[i] = cavaOut.getOrNull(i)?.toFloat() ?: 0f
                }
            }

            _barsFlow.value = bars.copyOf()

            val elapsed = System.currentTimeMillis() - start
            val sleep = frameTimeMs - elapsed
            if(sleep > 0) Thread.sleep(sleep)

            // We would reload cava if applicable here, but the LED count is hardcoded for now
        }
    }
}