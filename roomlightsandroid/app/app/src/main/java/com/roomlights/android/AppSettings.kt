package com.roomlights.android

import android.content.Context
import android.content.SharedPreferences
import android.util.Log
import androidx.core.content.edit
import com.roomlights.android.recorder.RecordingType
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow

class AppSettings(context: Context) {
    private val prefs = context.getSharedPreferences("settings", Context.MODE_PRIVATE)

    private val _showVisualizer = MutableStateFlow(prefs.getBoolean("show_visualizer", true))
    val showVisualizer: StateFlow<Boolean> = _showVisualizer
    fun setShowVisualizer(show: Boolean) { prefs.edit { putBoolean("show_visualizer", show) } }

    private val _visualizerBarCount = MutableStateFlow(prefs.getInt("visualizer_bar_count", 64))
    val visualizerBarCount: StateFlow<Int> = _visualizerBarCount
    fun setVisualizerBarCount(count: Int) { prefs.edit { putInt("visualizer_bar_count", count) } }

    private val _recordingType = MutableStateFlow(
        if(prefs.getBoolean("record_system_audio", false)) {
            RecordingType.SYSTEM_AUDIO
        } else {
            RecordingType.MICROPHONE
        }
    )
    val recordingType: StateFlow<RecordingType> = _recordingType
    fun setRecordingType(type: RecordingType) {
        prefs.edit {
            putBoolean("record_system_audio", type == RecordingType.SYSTEM_AUDIO)
        }
    }

    private val _udpEnabled = MutableStateFlow(prefs.getBoolean("udp_enabled", false))
    val udpEnabled: StateFlow<Boolean> = _udpEnabled
    fun setUdpEnabled(enabled: Boolean) { prefs.edit { putBoolean("udp_enabled", enabled) } }

    private val _udpAddress = MutableStateFlow(prefs.getString("udp_address", "") ?: "")
    val udpAddress: StateFlow<String> = _udpAddress
    fun setUdpAddress(address: String) { prefs.edit { putString("udp_address", address) } }

    private val listener = SharedPreferences.OnSharedPreferenceChangeListener { _, key ->
        when(key) {
            "show_visualizer" -> _showVisualizer.value = prefs.getBoolean("show_visualizer", true)
            "visualizer_bar_count" -> _visualizerBarCount.value = prefs.getInt("visualizer_bar_count", 64)

            "record_system_audio" -> {
                _recordingType.value = if(prefs.getBoolean("record_system_audio", false)) {
                    RecordingType.SYSTEM_AUDIO
                } else {
                    RecordingType.MICROPHONE
                }
            }

            "udp_enabled" -> _udpEnabled.value = prefs.getBoolean("udp_enabled", false)
            "udp_address" -> _udpAddress.value = prefs.getString("udp_address", "") ?: ""
        }
    }

    init {
        prefs.registerOnSharedPreferenceChangeListener(listener)
    }
}
