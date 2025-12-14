#include <jni.h>
#include <stdlib.h>
#include <math.h>
#include "cavacore.h"

struct cava_plan *plan;
double *cava_in;
double *cava_out;

// cava_init, initialize visualization, takes the following parameters:

// number_of_bars, number of wanted bars per channel

// rate, sample rate of input signal

// channels, number of interleaved channels in input

// autosens, toggle automatic sensitivity adjustment 1 = on, 0 = off
// on, gives a dynamically adjusted output signal from 0 to 1
// the output is continuously adjusted to use the entire range
// off, will pass the raw values from cava directly to the output
// the max values will then be dependent on the input

// noise_reduction, adjust noise reduction filters. 0 - 1, recommended 0.77
// the raw visualization is very noisy, this factor adjusts the integral
// and gravity filters inside cavacore to keep the signal smooth
// 1 will be very slow and smooth, 0 will be fast but noisy.

// low_cut_off, high_cut_off cut off frequencies for visualization in Hz
// recommended: 50, 10000

// returns a cava_plan to be used by cava_execute. If cava_plan.status is 0 all is OK.
// If cava_plan.status is -1, cava_init was called with an illegal parameter, see error string in
// cava_plan.error_message
JNIEXPORT void JNICALL Java_com_karlstav_cava_CavaNativeJNI_InitCava(
    JNIEnv *env,
    jobject thiz,

    jint bars_per_channel,
    jint sampleRate,
    jint channels,
    jboolean autosens,
    jfloat noise_reduction,
    jint lower_cut_off,
    jint higher_cut_off
) {
    plan = cava_init(
        bars_per_channel, // Number of bars
        sampleRate, // Rate
        channels, // Channels (only 1 or 2 allowed)
        autosens, // Autosens
        noise_reduction, // Noise reduction
        lower_cut_off, // Low cut off
        higher_cut_off // High cut off
    );
    cava_in = (double *)malloc(plan->FFTbassbufferSize * sizeof(double));
    cava_out = (double *)malloc(plan->number_of_bars * plan->audio_channels * sizeof(double));
}

// cava_execute, executes visualization

// cava_in, input buffer can be any size. internal buffers in cavacore is
// 4096 * number of channels at 44100 samples rate, if new_samples is greater
// then samples will be discarded. However it is recommended to use less
// new samples per execution as this determines your framerate.
// 512 samples at 44100 sample rate mono, gives about 86 frames per second.

// new_samples, the number of samples in cava_in to be processed per execution
// in case of async reading of data this number is allowed to vary from execution to execution

// cava_out, output buffer. Size must be number of bars * number of channels. Bars will
// be sorted from lowest to highest frequency. If stereo input channels are configured
// then all left channel bars will be first then the right.

// plan, the cava_plan struct returned from cava_init

// cava_execute assumes cava_in samples to be interleaved if more than one channel
// only up to two channels are supported.
JNIEXPORT jdoubleArray JNICALL Java_com_karlstav_cava_CavaNativeJNI_ExecCava(
    JNIEnv *env,
    jobject thiz,
    
    jdoubleArray cava_input,
    jint new_samples
) {
    jdoubleArray cavaReturn = (*env)->NewDoubleArray(env, plan->number_of_bars * plan->audio_channels);

    cava_in = (*env)->GetDoubleArrayElements(env, cava_input, NULL);

    cava_execute(cava_in, new_samples, cava_out, plan);

    (*env)->SetDoubleArrayRegion(env, cavaReturn, 0, plan->number_of_bars * plan->audio_channels, cava_out);
    (*env)->ReleaseDoubleArrayElements(env, cava_input, cava_in, JNI_ABORT);

    return cavaReturn;
}

// cava_destroy, destroys the plan, frees up memory
JNIEXPORT void JNICALL Java_com_karlstav_cava_CavaNativeJNI_DestroyCava(
    JNIEnv *env, 
    jobject thiz
) {
    cava_destroy(plan);
}