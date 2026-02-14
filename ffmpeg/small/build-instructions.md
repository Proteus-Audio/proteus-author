# Slim FFMPEG Build

This build is optimized for the single command that the authoring app uses.

```sh
git clone https://git.ffmpeg.org/ffmpeg.git
cd ffmpeg
make distclean 2>/dev/null || true

./configure \
 --prefix="$PWD/build" \
 --disable-everything \
 --enable-ffmpeg \
 --enable-protocol=file \
 --enable-muxer=matroska \
 --enable-encoder=libvorbis \
 --enable-libvorbis \
 --enable-demuxer=wav \
 --enable-decoder=pcm_s16le \
 --enable-decoder=pcm_s24le \
 --enable-decoder=pcm_s32le \
 --enable-decoder=pcm_f32le \
 --enable-demuxer=mp3 \
 --enable-decoder=mp3 \
 --enable-parser=mp3 \
 --enable-swresample \
 --enable-avfilter \
 --enable-filter=aresample \
 --enable-filter=aformat \
 --enable-filter=anull \
 --disable-doc \
 --disable-ffplay \
 --disable-ffprobe \
 --disable-avdevice \
 --disable-swscale \
 --disable-network

make -j
make install
strip build/bin/ffmpeg
```
