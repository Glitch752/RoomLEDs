package com.karlstav.cava

object CavaNative {
    fun init(
        bars: Int,
        sampleRate: Int,
        lowerCutoff: Int,
        higherCutoff: Int
    ): FloatArray =
        MyGLRenderer.InitCava(bars, sampleRate, lowerCutoff, higherCutoff)

    fun exec(
        audioIn: DoubleArray,
        numSamples: Int
    ): DoubleArray =
        MyGLRenderer.ExecCava(audioIn, numSamples)

    fun destroy() =
        MyGLRenderer.DestroyCava()
}