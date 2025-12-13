package com.roomlights.android

import android.Manifest
import android.content.pm.PackageManager
import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import androidx.appcompat.app.AppCompatDelegate
import androidx.core.app.ActivityCompat
import androidx.preference.PreferenceManager

var numberOfBarsSet = 40
var colorPreset = "1"
var changeColor = true
var configReload = false

class MainActivity : AppCompatActivity() {
    private lateinit var visualizer: Visualizer
    private lateinit var recorder: Recorder

    private val RECORD_AUDIO_PERMISSION_REQUEST_CODE = 101
    private var RECORD_AUDIO_PERMISSION_DENIED = false

    public override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        recorder = Recorder(this)

        val preferences = PreferenceManager.getDefaultSharedPreferences(this)
        numberOfBarsSet = preferences.getInt("number_of_bars_preference_key", 40)
        colorPreset = preferences.getString("color_presets", "1").toString()

        AppCompatDelegate.setDefaultNightMode(AppCompatDelegate.MODE_NIGHT_FOLLOW_SYSTEM);

        visualizer = Visualizer(this, recorder.barsFlow)
        setContentView(visualizer)
    }

    override fun onResume() {
        super.onResume()

        if(!RECORD_AUDIO_PERMISSION_DENIED) {
            if(checkPermission(Manifest.permission.RECORD_AUDIO)) {
                recorder.start()
            } else {
                requestPermission(
                    Manifest.permission.RECORD_AUDIO,
                    RECORD_AUDIO_PERMISSION_REQUEST_CODE
                )
            }
        }
    }

    fun checkPermission(permission: String): Boolean {
        val check = ActivityCompat.checkSelfPermission(this, permission)
        return check == PackageManager.PERMISSION_GRANTED
    }

    override fun onPause() {
        super.onPause()
        recorder.stop()
    }

    private fun requestPermission(permission: String, requestCode: Int) {
        ActivityCompat.requestPermissions(this, arrayOf(permission), requestCode)
    }

    override fun onRequestPermissionsResult(requestCode: Int, permissions: Array<String>, grantResults: IntArray) {
        super.onRequestPermissionsResult(requestCode, permissions, grantResults)
        when(requestCode) {
            RECORD_AUDIO_PERMISSION_REQUEST_CODE -> {
                RECORD_AUDIO_PERMISSION_DENIED = !(
                    grantResults.isNotEmpty() &&
                    grantResults[0] == PackageManager.PERMISSION_GRANTED
                )
            }
        }
    }
}
