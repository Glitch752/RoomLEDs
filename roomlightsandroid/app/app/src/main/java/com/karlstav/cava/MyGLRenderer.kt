package com.karlstav.cava;

// Why does cavacore call it this...
public class MyGLRenderer {
    companion object {
        init {
            System.loadLibrary("cavacore")
        }
        
        @JvmStatic
        public external fun InitCava(
            bars: Int,
            sampleRate: Int,
            lowerCutoff: Int,
            higherCutoff: Int
        ): FloatArray

        @JvmStatic
        public external fun ExecCava(
            audioIn: DoubleArray,
            numSamples: Int
        ): DoubleArray
        
        @JvmStatic
        public external fun DestroyCava()
    }
}