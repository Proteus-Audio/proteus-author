You can get a much smaller ffmpeg by building a mux‑only binary tailored to
your exact command. Your current command doesn’t specify -c, so ffmpeg will
choose default codecs and may force in encoders/decoders you don’t need. The
easiest win is to make the command “copy‑only,” then build a mux‑only ffmpeg.

What your command does

- It concatenates multiple input files into a single Matroska audio container
  (.mka), attaches a JSON file, and sets stream titles.
- There’s no explicit codec setting, so ffmpeg will re‑encode by default.

Easy win: make it stream‑copy
If your input files are already in a codec that Matroska can store (most audio
codecs are), you can avoid re‑encoding:

# Add this to the command in export_prot:

-c copy

That turns your workflow into mux‑only, which drastically reduces the ffmpeg
build.

Minimal ffmpeg build for mux‑only .mka
This assumes you only need to read files from disk and mux them into Matroska,
no decoding/encoding.

git clone https://git.ffmpeg.org/ffmpeg.git
cd ffmpeg

./configure \
 --prefix="$PWD/build" \
 --disable-everything \
 --enable-ffmpeg \
 --enable-protocol=file \
 --enable-muxer=matroska \
 --enable-demuxer=wav \
 --enable-demuxer=flac \
 --enable-demuxer=mp3 \
 --enable-demuxer=ogg \
 --enable-demuxer=mov \
 --enable-demuxer=matroska \
 --enable-parser=flac \
 --enable-parser=opus \
 --enable-parser=vorbis \
 --enable-parser=mp3 \
 --disable-debug \
 --disable-doc \
 --disable-ffplay \
 --disable-ffprobe \
 --disable-avdevice \
 --disable-postproc \
 --disable-swscale \
 --disable-network \
 --extra-cflags="-Os" \
 --extra-ldflags="-Wl,-dead_strip"

make -j
make install
strip build/bin/ffmpeg

Notes:

- Keep only the demuxers/parsers you actually use. The list above is a guess;
  remove what you don’t need.
- If you only ever use a single input format (e.g., WAV), keep only that
  demuxer and parser.

If you must re‑encode
Pick one target codec and enable only its encoder plus any resampler you need.
Example: encode everything to FLAC in .mka:

./configure \
 --prefix="$PWD/build" \
 --disable-everything \
 --enable-ffmpeg \
 --enable-protocol=file \
 --enable-muxer=matroska \
 --enable-encoder=flac \
 --enable-decoder=pcm_s16le \
 --enable-demuxer=wav \
 --enable-parser=flac \
 --enable-swresample \
 --disable-debug \
 --disable-doc \
 --disable-ffplay \
 --disable-ffprobe \
 --disable-avdevice \
 --disable-postproc \
 --disable-swscale \
 --disable-network \
 --extra-cflags="-Os" \
 --extra-ldflags="-Wl,-dead_strip"

Then change your command to:

-c:a flac

What I need from you to tailor it

- What audio input formats do you actually export from? (wav, flac, mp3, aac,
  m4a, ogg, opus, etc)
- Do you want to preserve the original codecs (-c copy) or re‑encode to a
  single codec?

If you answer those, I can give you an exact configure line and adjust the
export_prot command accordingly.

---

The failure cascaded from ./configure rejecting --disable-postproc. Because
configure failed, ffbuild/config.mak wasn’t generated, so make and strip
couldn’t find anything.

Here’s a corrected, minimal, mux‑only build that avoids that option and should
work on current ffmpeg:

git clone https://git.ffmpeg.org/ffmpeg.git
cd ffmpeg

# Clean if you already tried once

make distclean 2>/dev/null || true

./configure \
 --prefix="$PWD/build" \
 --disable-everything \
 --enable-ffmpeg \
 --enable-protocol=file \
 --enable-muxer=matroska \
 --enable-demuxer=wav \
 --enable-demuxer=flac \
 --enable-demuxer=mp3 \
 --enable-demuxer=ogg \
 --enable-demuxer=mov \
 --enable-demuxer=matroska \
 --enable-parser=flac \
 --enable-parser=opus \
 --enable-parser=vorbis \
 --enable-parser=mp3 \
 --disable-debug \
 --disable-doc \
 --disable-ffplay \
 --disable-ffprobe \
 --disable-avdevice \
 --disable-swscale \
 --disable-network \
 --extra-cflags="-Os" \
 --extra-ldflags="-Wl,-dead_strip"

make -j
make install
strip build/bin/ffmpeg

If you still get “Unknown option”, run:

./configure --help | rg "disable-(postproc|swscale|avdevice)"

and I’ll adjust for whatever your ffmpeg version supports.

Also, for mux‑only, update your command in export_prot to force stream copy:

-c copy

That lets you remove all encoders/decoders entirely from the build, which is
what makes it small.

To tailor the exact minimal --enable-\* list, tell me:

1. What input audio formats do you actually export from?
2. Are you OK with -c copy, or do you need to re‑encode to a fixed codec?

---

1. Install deps (macOS)

brew install libogg libvorbis

2. Minimal ffmpeg build (Vorbis encoder, WAV+MP3 inputs)

git clone https://git.ffmpeg.org/ffmpeg.git
cd ffmpeg
make distclean 2>/dev/null || true

./configure \
 --prefix="$PWD/build" \
 --disable-everything \
 --enable-ffmpeg \
 --enable-protocol=file \
 --enable-muxer=ogg \
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
 --disable-doc \
 --disable-ffplay \
 --disable-ffprobe \
 --disable-avdevice \
 --disable-swscale \
 --disable-network

make -j
make install
strip build/bin/ffmpeg

3. Use it in your command

-c:a libvorbis

If you still want .mka output (Matroska) instead of .ogg, swap the muxer line
to:

--enable-muxer=matroska

or enable both:

--enable-muxer=ogg --enable-muxer=matroska

———

Extending later (add only what you need)

Add these when you’re ready:

- FLAC input
  - --enable-demuxer=flac --enable-decoder=flac --enable-parser=flac
- AAC input (raw .aac)
  - --enable-demuxer=aac --enable-decoder=aac --enable-parser=aac
- AAC in .m4a / .mp4
  - --enable-demuxer=mov --enable-decoder=aac

If you tell me your actual input file types (extensions and how they’re
produced), I can tighten this further and remove extra PCM decoders.

One question to confirm: do you want the output container to be .ogg now, or
are you still writing .mka with Vorbis inside?

---

You have two fixes:

Option A: Add Matroska muxer to the build (recommended)
Rebuild with Matroska enabled:

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
 --disable-doc \
 --disable-ffplay \
 --disable-ffprobe \
 --disable-avdevice \
 --disable-swscale \
 --disable-network

If you also want .ogg output, keep both:

--enable-muxer=ogg --enable-muxer=matroska

Option B: Force the output format
If you can’t rebuild right now, add -f ogg and output .ogg instead of .mka.
But .mka specifically requires the Matroska muxer, so this isn’t compatible
with your current file extension/format.

———

Given your app writes .mka, you want Option A.
If you want, I can help you lock the muxer and codec explicitly in export_prot
so this never varies by ffmpeg build.

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
 --disable-doc \
 --disable-ffplay \
 --disable-ffprobe \
 --disable-avdevice \
 --disable-swscale \
 --disable-network

make -j
make install
strip build/bin/ffmpeg

---

The key error is this line:

'aresample' filter not present, cannot convert formats.

Your inputs are pcm_s24le (s32), but libvorbis requires fltp. ffmpeg normally
auto‑inserts aresample/aformat filters to convert sample format. Because you
built with --disable-everything and didn’t enable libavfilter or the aresample
filter, ffmpeg can’t do the conversion and the encoder fails.

Fix: enable avfilter + aresample (and aformat to be safe). Example:

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

That should unblock your command.

Why it fails now

- libvorbis needs fltp samples
- your WAV inputs are s32
- conversion requires aresample filter
- filter wasn’t built

If you want to keep the build even tighter, we can verify which filters get
auto‑inserted by ffmpeg and keep only those.

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
