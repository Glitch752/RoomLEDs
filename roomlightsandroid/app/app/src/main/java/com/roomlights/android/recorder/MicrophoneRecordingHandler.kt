package com.roomlights.android.recorder

import android.Manifest
import android.content.pm.PackageManager
import android.media.AudioFormat
import android.media.AudioRecord
import android.media.MediaRecorder
import android.util.Log
import androidx.annotation.RequiresPermission
import androidx.core.app.ActivityCompat
import com.roomlights.android.MainActivity
import com.roomlights.android.RoomLightsApplication

private const val AUDIO_SOURCE = MediaRecorder.AudioSource.MIC
private const val CHANNEL_CONFIG = AudioFormat.CHANNEL_IN_MONO
private const val AUDIO_FORMAT = AudioFormat.ENCODING_PCM_16BIT

private val BUFFER_SIZE_RECORDING = AudioRecord.getMinBufferSize(
    RECORDER_SAMPLE_RATE, CHANNEL_CONFIG, AUDIO_FORMAT
)

private const val LOG_TAG = "room_lights_microphone_recorder"

class MicrophoneRecordingHandler(app: RoomLightsApplication) : RecordingHandler(app) {
    private var audioRecord: AudioRecord? = null
    private var recordingThread: Thread? = null
    private var isRecording = false
    private var data = RecordingData(
        RecordingType.MICROPHONE,
        DoubleArray(BUFFER_SIZE_RECORDING * 2) /* Cava input samples */
    )

    private val audioBuffer = ByteArray(BUFFER_SIZE_RECORDING / 2)

    override fun checkPermissions(): Boolean {
        if(
            ActivityCompat.checkSelfPermission(app, Manifest.permission.RECORD_AUDIO) !=
            PackageManager.PERMISSION_GRANTED
        ) {
            // Request permission from user
            app.requestPermissionForHandler(Manifest.permission.RECORD_AUDIO)
            
            return false
        }

        return true
    }

    @RequiresPermission(Manifest.permission.RECORD_AUDIO)
    override fun startRecording(): RecordingData? {
        if(isRecording) return null
        isRecording = true

        audioRecord = AudioRecord(
            AUDIO_SOURCE,
            RECORDER_SAMPLE_RATE,
            CHANNEL_CONFIG,
            AUDIO_FORMAT,
            BUFFER_SIZE_RECORDING
        )
        if(audioRecord?.state != AudioRecord.STATE_INITIALIZED) {
            Log.e(LOG_TAG, "Error initializing AudioRecord")
            return null
        }
        audioRecord?.startRecording()

        recordingThread = Thread { writeAudioDataToBuffer() }
        recordingThread?.start()

        return data
    }

    private fun writeAudioDataToBuffer() {
        while(isRecording) {
            val read = audioRecord?.read(audioBuffer, 0, audioBuffer.size) ?: 0
            data.lock.lock()
            for(i in 0 until read step 2) {
                if(i > audioBuffer.size - 2) break
                val lowByte = audioBuffer[i].toInt()
                val highByte = audioBuffer[i + 1].toInt()
                val pcmValue = (highByte shl 8) or (lowByte and 0xFF)
                data.cavaInputSamplesL[data.newSampleCount] = pcmValue.toDouble() / 32767.0
                data.newSampleCount++
                if(data.newSampleCount > data.cavaInputSamplesL.size - 1) data.newSampleCount = 0
            }
            data.lock.unlock()
        }
    }

    override fun stopRecording() {
        isRecording = false
        try {
            recordingThread?.join()
        } catch(_: InterruptedException) {
            // Ignore
        }

        audioRecord?.stop()
        audioRecord?.release()
        audioRecord = null
    }

    override fun release() {
        // No resources to release
    }
}