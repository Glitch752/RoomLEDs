package com.roomlights.android

import android.app.Activity
import android.content.Context
import android.content.Intent
import android.content.pm.PackageManager
import android.graphics.Color
import android.media.projection.MediaProjectionManager
import android.os.Bundle
import android.util.Log
import android.view.Gravity
import android.widget.FrameLayout
import androidx.appcompat.app.AppCompatActivity
import androidx.appcompat.app.AppCompatDelegate
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import androidx.core.view.isVisible
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.lifecycleScope
import androidx.lifecycle.repeatOnLifecycle
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import kotlinx.coroutines.withTimeout

class MainActivity : AppCompatActivity() {
    val app by lazy {
        application as RoomLightsApplication
    }

    private lateinit var visualizer: Visualizer

    public override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        val root = FrameLayout(this).apply {
            setBackgroundColor(Color.rgb(20, 20, 30))
        }
        setContentView(root)

        AppCompatDelegate.setDefaultNightMode(AppCompatDelegate.MODE_NIGHT_FOLLOW_SYSTEM)

        // Hide the action bar
        supportActionBar?.hide()

        visualizer = Visualizer(this, app.recorder.barsFlow)
        root.addView(
            visualizer,
            FrameLayout.LayoutParams(
                FrameLayout.LayoutParams.MATCH_PARENT,
                FrameLayout.LayoutParams.MATCH_PARENT
            )
        )

        root.addView(
            SettingsPanel(this),
            FrameLayout.LayoutParams(
                FrameLayout.LayoutParams.WRAP_CONTENT,
                FrameLayout.LayoutParams.WRAP_CONTENT,
                Gravity.TOP or Gravity.START
            )
        )

        lifecycleScope.launch {
            repeatOnLifecycle(Lifecycle.State.STARTED) {
                app.settings.showVisualizer.collect { enabled ->
                    visualizer.isVisible = enabled
                }
            }
        }
        lifecycleScope.launch {
            repeatOnLifecycle(Lifecycle.State.STARTED) {
                app.settings.visualizerBarCount.collect { count ->
                    visualizer.setBarCount(count)
                }
            }
        }

        launchApplicationActivityHandlers()

        app.initializeIfNeeded()
    }

    private fun launchApplicationActivityHandlers() {
        Log.e("rladebug", "launching application activity handlers")
        
        lifecycleScope.launch {
            repeatOnLifecycle(Lifecycle.State.STARTED) {
                app.permissionRequest.collect { permission ->
                    if(permission != null) {
                        Log.e("rladebug", "requesting permission in activity: $permission")
                        ActivityCompat.requestPermissions(
                            this@MainActivity,
                            arrayOf(permission),
                            1
                        )
                    }
                }
            }
        }
        lifecycleScope.launch {
            repeatOnLifecycle(Lifecycle.State.STARTED) {
                app.activityRequests.collect { intent ->
                    if(intent != null) {
                        Log.e("rladebug", "starting activity for result: $intent")
                        startActivityForResult(intent, 1)
                    }
                }
            }
        }
        lifecycleScope.launch {
            repeatOnLifecycle(Lifecycle.State.STARTED) {
                app.stopServiceRequest.collect { stopRequested ->
                    if(stopRequested) {
                        Log.e("rladebug", "stopping media projection service as requested")
                        val intent = Intent(this@MainActivity, MediaProjectionService::class.java)
                        stopService(intent)
                        
                        app.mediaProjectionServiceReady = false
                    }
                }
            }
        }
    }

    override fun onRequestPermissionsResult(requestCode: Int, permissions: Array<String>, grantResults: IntArray) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults)
        if(requestCode == 1) {
            if((grantResults.isNotEmpty() && grantResults[0] == PackageManager.PERMISSION_GRANTED)) {
                app.recorder.start()
            }
        }
    }

    override fun onActivityResult(
        requestCode: Int,
        resultCode: Int,
        data: Intent?
    ) {
        super.onActivityResult(requestCode, resultCode, data)
        if(requestCode == 1 && resultCode == Activity.RESULT_OK && data != null) {
            val intent = Intent(this, MediaProjectionService::class.java)
            startForegroundService(intent)
            Log.e("rladebug", "started foreground service for media projection")

            lifecycleScope.launch {
                Log.e("rladebug", "waiting for media projection service to be ready")

                withTimeout(3_000) {
                    while(!app.mediaProjectionServiceReady) {
                        delay(50)
                    }
                }

                Log.e("rladebug", "media projection service is ready")

                val mgr = getSystemService(Context.MEDIA_PROJECTION_SERVICE) as MediaProjectionManager
                app.mediaProjection = mgr.getMediaProjection(resultCode, data)

                Log.e("rladebug", "got media projection, starting recorder")

                app.recorder.start()
            }
        }
    }
}
