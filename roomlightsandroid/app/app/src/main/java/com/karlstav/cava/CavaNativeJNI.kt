package com.karlstav.cava;

public class CavaNativeJNI {
    companion object {
        init {
            System.loadLibrary("cavacore-jni")
        }
        
        @JvmStatic
        public external fun InitCava(
            barsPerChannel: Int,
            sampleRate: Int,
            channels: Int,
            autosens: Boolean,
            noiseReduction: Float,
            lowerCutoff: Int,
            higherCutoff: Int
        )

        @JvmStatic
        public external fun ExecCava(
            audioIn: DoubleArray,
            numSamples: Int
        ): DoubleArray
        
        @JvmStatic
        public external fun DestroyCava()
    }
}