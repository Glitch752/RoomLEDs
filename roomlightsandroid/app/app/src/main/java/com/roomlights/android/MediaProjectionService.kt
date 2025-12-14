package com.roomlights.android

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.Service
import android.content.Intent
import android.content.pm.ServiceInfo
import android.os.Build
import android.os.IBinder
import android.util.Log
import androidx.core.app.NotificationCompat
import com.roomlights.android.RoomLightsApplication

class MediaProjectionService : Service() {
    override fun onBind(intent: Intent?): IBinder? = null
    
    init {
        Log.e("rladebug", "SERVICE CLASS LOADED")
    }

    override fun onCreate() {
        super.onCreate()
        Log.d("rladebug", "onCreate() called")
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        Log.e("rladebug", "MediaProjectionService started")

        startForeground(
            100,
            createNotification(),
            ServiceInfo.FOREGROUND_SERVICE_TYPE_MEDIA_PROJECTION
        )

        (application as RoomLightsApplication).mediaProjectionServiceReady = true

        return START_STICKY
    }

    private fun createNotification(): Notification {
        val channelId = "media_projection"

        val channel = NotificationChannel(
            channelId,
            "Audio Capture",
            NotificationManager.IMPORTANCE_LOW
        )
        getSystemService(NotificationManager::class.java)
            .createNotificationChannel(channel)
        
        return Notification.Builder(this, channelId)
            .setContentTitle("Test")
            .setSmallIcon(android.R.drawable.ic_media_play)
            .build()
    }
}
