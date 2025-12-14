package com.roomlights.android;

import android.app.Application
import android.content.Intent
import android.media.projection.MediaProjection
import android.util.Log
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import com.roomlights.android.recorder.Recorder
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.flow.MutableSharedFlow
import kotlinx.coroutines.flow.MutableStateFlow

class RoomLightsApplication : Application() {
    /**
     * An application-wide coroutine scope
     */
    val appScope = CoroutineScope(
        SupervisorJob() + Dispatchers.Default
    )

    lateinit var recorder: Recorder
    lateinit var settings: AppSettings
    lateinit var udpSender: UdpSender

    var mediaProjectionServiceReady: Boolean = false

    // This should probably be a MutableSharedFlow, but I couldn't get it to work... whatever!
    var permissionRequest: MutableStateFlow<String?> = MutableStateFlow(null)
    fun requestPermissionForHandler(permission: String) {
        if(permissionRequest.tryEmit(permission)) {
            Log.e("rladebug", "permission request emitted: $permission")
        } else {
            Log.e("rladebug", "permission request failed: $permission")
        }
    }

    var mediaProjection: MediaProjection? = null
    fun hasMediaProjection(): Boolean {
        return mediaProjection != null
    }

    var activityRequests: MutableStateFlow<Intent?> = MutableStateFlow(null)
    fun startActivityForResult(intent: Intent) {
        if(activityRequests.tryEmit(intent)) {
            Log.e("rladebug", "activity request emitted: $intent")
        } else {
            Log.e("rladebug", "activity request failed: $intent")
        }
    }

    var stopServiceRequest: MutableStateFlow<Boolean> = MutableStateFlow(false)
    fun requestStopMediaProjectionService() {
        if(stopServiceRequest.tryEmit(true)) {
            Log.e("rladebug", "stop service request emitted")
        } else {
            Log.e("rladebug", "stop service request failed")
        }
    }

    override fun onCreate() {
        super.onCreate()

        settings = AppSettings(this)
        recorder = Recorder(this)
        udpSender = UdpSender(this, recorder.barsFlow, settings.udpEnabled, settings.udpAddress)
        
        udpSender.start()
    }

    private var initialized = false
    fun initializeIfNeeded() {
        if(initialized) return
        initialized = true

        recorder.start()
    }
}