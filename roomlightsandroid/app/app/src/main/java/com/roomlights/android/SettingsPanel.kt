package com.roomlights.android

import android.annotation.SuppressLint
import android.content.Context
import android.graphics.Color
import android.widget.AdapterView
import android.widget.ArrayAdapter
import android.widget.CheckBox
import android.widget.LinearLayout
import android.widget.SeekBar
import android.widget.Spinner
import android.widget.TextView
import androidx.activity.ComponentActivity
import androidx.appcompat.widget.AppCompatEditText
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.lifecycleScope
import androidx.lifecycle.repeatOnLifecycle
import com.roomlights.android.recorder.RecordingType
import kotlinx.coroutines.launch

// Maybe there's a better way to do this, but whatever
private class HorizontalDivider(context: Context) : LinearLayout(context) {
    init {
        layoutParams = LayoutParams(
            LayoutParams.MATCH_PARENT,
            2
        )
        setBackgroundColor(Color.rgb(50, 50, 50))
    }
}

/**
 * A wrapper for a linear layout that contains settings UI elements
 */
@SuppressLint("ViewConstructor")
class SettingsPanel(context: ComponentActivity) : LinearLayout(context) {
    init {
        orientation = VERTICAL
        setPadding(16, 16, 16, 16)
        setBackgroundColor(Color.argb(128, 0, 0, 0))
        layoutParams = LayoutParams(
            LayoutParams.MATCH_PARENT,
            LayoutParams.WRAP_CONTENT
        )

        // Whether to show the visualizer
        val showVisualizer = CheckBox(context).apply {
            text = "Show Visualizer"
        }
        addView(showVisualizer)

        // Label for the seek bar
        addView(TextView(context).apply {
            text = "Visualizer Bar Count"
        })

        // The bar count for the visualizer, between 8 and MAX_VISUALIZER_BARS;
        // only active if visualizer is shown
        val visualizerBarCountSeekBar = SeekBar(context).apply {
            min = 8
            max = MAX_VISUALIZER_BARS
            progress = 64
        }
        addView(visualizerBarCountSeekBar)

        addView(HorizontalDivider(context))

        // The current recording type selection
        val recordingTypeSpinner = Spinner(context).apply {
            adapter = ArrayAdapter(
                context,
                android.R.layout.simple_spinner_dropdown_item,
                listOf(RecordingType.MICROPHONE, RecordingType.SYSTEM_AUDIO)
            )
        }
        addView(recordingTypeSpinner)

        addView(HorizontalDivider(context))

        // Whether UDP output is enabled
        val udpEnabled = CheckBox(context).apply {
            text = "Enable UDP Output"
        }
        addView(udpEnabled)

        // The UDP address input; only active if UDP is enabled
        val udpAddressInput = AppCompatEditText(context).apply {
            hint = "UDP Address"
            isSingleLine = true
        }
        addView(udpAddressInput)

        addView(HorizontalDivider(context))

        val app = (context.applicationContext as RoomLightsApplication)

        // Status text
        val statusText = TextView(context).apply {
            text = "Idle"
            setTextColor(Color.rgb(220, 220, 200))
        }
        addView(statusText)
        context.lifecycleScope.launch {
            context.repeatOnLifecycle(Lifecycle.State.STARTED) {
                app.udpSender.status.collect { status ->
                    statusText.text = status
                }
            }
        }

        context.lifecycleScope.launch {
            context.repeatOnLifecycle(Lifecycle.State.STARTED) {
                app.settings.showVisualizer.collect { show ->
                    showVisualizer.isChecked = show
                    visualizerBarCountSeekBar.isEnabled = show
                }
            }
        }
        context.lifecycleScope.launch {
            context.repeatOnLifecycle(Lifecycle.State.STARTED) {
                app.settings.visualizerBarCount.collect { count ->
                    visualizerBarCountSeekBar.progress = count
                }
            }
        }
        context.lifecycleScope.launch {
            context.repeatOnLifecycle(Lifecycle.State.STARTED) {
                app.settings.recordingType.collect { type ->
                    recordingTypeSpinner.setSelection(
                        when(type) {
                            RecordingType.MICROPHONE -> 0
                            RecordingType.SYSTEM_AUDIO -> 1
                        }
                    )
                }
            }
        }
        context.lifecycleScope.launch {
            context.repeatOnLifecycle(Lifecycle.State.STARTED) {
                app.settings.udpEnabled.collect { enabled ->
                    udpEnabled.isChecked = enabled
                    udpAddressInput.isEnabled = enabled
                }
            }
        }
        context.lifecycleScope.launch {
            context.repeatOnLifecycle(Lifecycle.State.STARTED) {
                app.settings.udpAddress.collect { address ->
                    udpAddressInput.setText(address)
                }
            }
        }

        showVisualizer.setOnCheckedChangeListener { _, isChecked ->
            app.settings.setShowVisualizer(isChecked)
        }
        visualizerBarCountSeekBar.setOnSeekBarChangeListener(object : SeekBar.OnSeekBarChangeListener {
            override fun onProgressChanged(seekBar: SeekBar?, progress: Int, fromUser: Boolean) {
                app.settings.setVisualizerBarCount(progress)
            }

            override fun onStartTrackingTouch(seekBar: SeekBar?) {
                // Do nothing
            }

            override fun onStopTrackingTouch(seekBar: SeekBar?) {
                // Do nothing
            }
        })
        recordingTypeSpinner.onItemSelectedListener = object : AdapterView.OnItemSelectedListener {
            override fun onItemSelected(
                parent: AdapterView<*>?,
                view: android.view.View?,
                position: Int,
                id: Long
            ) {
                val selectedType = when(position) {
                    0 -> RecordingType.MICROPHONE
                    1 -> RecordingType.SYSTEM_AUDIO
                    else -> RecordingType.MICROPHONE
                }
                app.settings.setRecordingType(selectedType)
            }

            override fun onNothingSelected(parent: AdapterView<*>?) {
                // Do nothing
            }
        }
        udpEnabled.setOnCheckedChangeListener { _, isChecked ->
            app.settings.setUdpEnabled(isChecked)
        }
        udpAddressInput.setOnFocusChangeListener { _, hasFocus ->
            if(!hasFocus) {
                app.settings.setUdpAddress(udpAddressInput.text.toString())
            }
        }
    }
}