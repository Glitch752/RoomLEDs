package com.roomlights.android.recorder

import android.Manifest
import android.content.Context
import android.content.pm.PackageManager
import android.media.AudioAttributes
import android.media.AudioFormat
import android.media.AudioPlaybackCaptureConfiguration
import android.media.AudioRecord
import android.media.projection.MediaProjection
import android.media.projection.MediaProjectionManager
import android.os.Build
import android.util.Log
import androidx.annotation.RequiresPermission
import androidx.core.content.ContextCompat
import com.roomlights.android.RoomLightsApplication
import kotlin.math.min

class SystemAudioRecordingHandler(
    app: RoomLightsApplication
) : RecordingHandler(app) {

    private var audioRecord: AudioRecord? = null
    private var recordingThread: Thread? = null
    private var isRunning = false

    private lateinit var recordingData: RecordingData

    private val sampleRate = 44100
    private val channelConfig = AudioFormat.CHANNEL_IN_STEREO
    private val audioFormat = AudioFormat.ENCODING_PCM_FLOAT

    private var mediaProjection: MediaProjection?
        get() = app.mediaProjection
        set(value) {
            app.mediaProjection = value
        }
    
    override fun checkPermissions(): Boolean {
        // RECORD_AUDIO runtime permission
        if(ContextCompat.checkSelfPermission(
                app,
                Manifest.permission.RECORD_AUDIO
            ) != PackageManager.PERMISSION_GRANTED
        ) {
            app.requestPermissionForHandler(Manifest.permission.RECORD_AUDIO);
            return false
        }

        // MediaProjection permission
        if(!app.hasMediaProjection()) {
            val mgr = app.getSystemService(Context.MEDIA_PROJECTION_SERVICE) as MediaProjectionManager
            app.startActivityForResult(mgr.createScreenCaptureIntent())
            return false
        }

        return true
    }

    @RequiresPermission(Manifest.permission.RECORD_AUDIO)
    override fun startRecording(): RecordingData? {
        if(!checkPermissions()) return null
        if(mediaProjection == null) return null

        val config = AudioPlaybackCaptureConfiguration.Builder(mediaProjection!!)
            .addMatchingUsage(AudioAttributes.USAGE_MEDIA)
            .addMatchingUsage(AudioAttributes.USAGE_GAME)
            .addMatchingUsage(AudioAttributes.USAGE_NOTIFICATION)
            .build()

        val minBufferSize = AudioRecord.getMinBufferSize(
            sampleRate,
            channelConfig,
            audioFormat
        )

        audioRecord = AudioRecord.Builder()
            .setAudioFormat(
                AudioFormat.Builder()
                    .setSampleRate(sampleRate)
                    .setEncoding(audioFormat)
                    .setChannelMask(channelConfig)
                    .build()
            )
            .setBufferSizeInBytes(minBufferSize * 2)
            .setAudioPlaybackCaptureConfig(config)
            .build()

        if (audioRecord?.state != AudioRecord.STATE_INITIALIZED) {
            audioRecord?.release()
            return null
        }

        recordingData = RecordingData(
            recordingType = RecordingType.SYSTEM_AUDIO,
            cavaInputSamplesL = DoubleArray(2048),
            cavaInputSamplesR = DoubleArray(2048)
        )

        isRunning = true
        audioRecord?.startRecording()

        startReaderThread(minBufferSize)

        return recordingData
    }

    private fun startReaderThread(bufferSize: Int) {
        recordingThread = Thread {
            val floatBuffer = FloatArray(bufferSize)

            while (isRunning) {
                val read = audioRecord?.read(
                    floatBuffer,
                    0,
                    floatBuffer.size,
                    AudioRecord.READ_BLOCKING
                ) ?: 0

                if (read <= 0) continue

                recordingData.lock.lock()
                try {
                    val frames = read / 2
                    val max = min(
                        frames,
                        recordingData.cavaInputSamplesL.size
                    )

                    for(i in 0 until max) {
                        recordingData.cavaInputSamplesL[i] =
                            floatBuffer[i * 2].toDouble()

                        recordingData.cavaInputSamplesR?.set(
                            i,
                            floatBuffer[i * 2 + 1].toDouble()
                        )
                    }

                    recordingData.newSampleCount = max
                } finally {
                    recordingData.lock.unlock()
                }
            }
        }

        recordingThread?.name = "PlaybackCaptureReader"
        recordingThread?.priority = Thread.MAX_PRIORITY
        recordingThread?.start()
    }

    override fun stopRecording() {
        isRunning = false
        recordingThread?.join(300)
        recordingThread = null

        audioRecord?.stop()
    }

    override fun release() {
        stopRecording()
        audioRecord?.release()
        audioRecord = null

        // mediaProjection?.stop()
        // app.requestStopMediaProjectionService()
    }
}
