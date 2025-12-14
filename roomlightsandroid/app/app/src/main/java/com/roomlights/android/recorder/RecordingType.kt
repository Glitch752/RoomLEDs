package com.roomlights.android.recorder

import java.util.concurrent.locks.ReentrantLock
import com.roomlights.android.RoomLightsApplication

enum class RecordingType {
    MICROPHONE,
    SYSTEM_AUDIO;

    fun displayName(): String {
        return when(this) {
            MICROPHONE -> "Microphone"
            SYSTEM_AUDIO -> "System Audio"
        }
    }
}

class RecordingData(
    val recordingType: RecordingType,
    var cavaInputSamplesL: DoubleArray,
    var cavaInputSamplesR: DoubleArray? = null,
    var newSampleCount: Int = 0,
    var lock: ReentrantLock = ReentrantLock()
) {
    fun getInterleavedCavaBuffer(channels: Int): DoubleArray {
        if(channels == 1) {
            return cavaInputSamplesL
        }

        if(cavaInputSamplesR == null) {
            val interleaved = DoubleArray(newSampleCount * 2)
            for(i in 0 until newSampleCount) {
                interleaved[i * 2] = cavaInputSamplesL[i]
                interleaved[i * 2 + 1] = cavaInputSamplesL[i]
            }
            return interleaved
        }

        val interleaved = DoubleArray(newSampleCount * 2)
        for(i in 0 until newSampleCount) {
            interleaved[i * 2] = cavaInputSamplesL[i]
            interleaved[i * 2 + 1] = cavaInputSamplesR!![i]
        }
        return interleaved
    }
}

abstract class RecordingHandler(protected val app: RoomLightsApplication) {
    abstract fun checkPermissions(): Boolean
    abstract fun startRecording(): RecordingData?
    abstract fun stopRecording()
    abstract fun release()
}