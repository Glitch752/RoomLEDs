package com.karlstav.cava

object CavaNative {
    /**
     * Initializes visualization.
     *
     * @param barsPerChannel Number of wanted bars per channel.
     * @param sampleRate Sample rate of input signal.
     * @param channels Number of interleaved channels in input.
     * @param autosens Toggle automatic sensitivity adjustment. `true` = on, `false` = off.
     *   - On: gives a dynamically adjusted output signal from 0 to 1. The output is continuously adjusted to use the entire range.
     *   - Off: passes the raw values from cava directly to the output. The max values will then be dependent on the input.
     * @param noiseReduction Adjust noise reduction filters. 0 - 1, recommended 0.77.
     *   - The raw visualization is very noisy, this factor adjusts the integral and gravity filters inside cavacore to keep the signal smooth.
     *   - 1 will be very slow and smooth, 0 will be fast but noisy.
     * @param lowerCutoff Low cutoff frequency for visualization in Hz. Recommended: 50.
     * @param higherCutoff High cutoff frequency for visualization in Hz. Recommended: 10000.
     * @return A cava_plan to be used by cava_execute. If cava_plan.status is 0 all is OK.
     *   If cava_plan.status is -1, cava_init was called with an illegal parameter, see error string in cava_plan.error_message.
     */
    fun init(
        barsPerChannel: Int,
        sampleRate: Int,
        channels: Int,
        autosens: Boolean,
        noiseReduction: Float,
        lowerCutoff: Int,
        higherCutoff: Int
    ) = CavaNativeJNI.InitCava(
            barsPerChannel,
            sampleRate,
            channels,
            autosens,
            noiseReduction,
            lowerCutoff,
            higherCutoff
        )

    /**
     * Executes visualization.
     *
     * @param audioIn Input buffer. Can be any size. Internal buffers in cavacore are
     * 4096 * number of channels at 44100 sample rate. If numSamples is greater,
     * samples will be discarded. It is recommended to use fewer samples per execution
     * as this determines your framerate. For example, 512 samples at 44100 sample rate mono
     * gives about 86 frames per second.
     * @param numSamples The number of samples in audioIn to be processed per execution.
     * In case of async reading of data, this number is allowed to vary from execution to execution.
     * @return Output buffer. Size is number of bars * number of channels. Bars are sorted from
     * lowest to highest frequency. If stereo input channels are configured, all left channel bars
     * will be first, then the right.
     *
     * Notes:
     * - Only up to two channels are supported.
     * - Assumes audioIn samples to be interleaved if more than one channel.
     */
    fun exec(
        audioIn: DoubleArray,
        numSamples: Int
    ): DoubleArray = CavaNativeJNI.ExecCava(audioIn, numSamples)

    /**
     * Destroys visualization and frees allocated memory.
     */
    fun destroy() = CavaNativeJNI.DestroyCava()
}