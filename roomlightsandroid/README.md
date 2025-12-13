# Room lights android

An adroid app to replace `roomlightsctl` on android and provide native functionality.

Currently just a clone of [cavandroid](https://github.com/karlstav/cava/tree/master/cavandroid).

## Building

Make sure to clone with git submodules before building:

```
git clone --recurse-submodules
```

You may need to create a .env file and source it for a proper development environment on linux.
See `.env.example-linux`.

First, build fftw3-android:
```
cd fftw3-android
./build.sh
```