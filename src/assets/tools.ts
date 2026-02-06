// import axios from "axios";

function arrRandom<T>(arr: T[]): T | undefined {
  if (arr.length === 0) return undefined
  return arr[Math.floor(Math.random() * arr.length)]
}

// const srcToFile = async (src: string, fileName: string): Promise<File> => {
//   const response = await axios.get(src, {
//     responseType: "blob",
//   });
//   const mimeType = response.headers["content-type"];
//   return new File([response.data], fileName, { type: mimeType });
// };

const cloneAudioBuffer = (fromAudioBuffer: AudioBuffer): AudioBuffer => {
  const audioContext = new (
    window.AudioContext ||
    (window as unknown as { webkitAudioContext: AudioContext }).webkitAudioContext
  )()
  const audioBuffer = audioContext.createBuffer(
    fromAudioBuffer.numberOfChannels,
    fromAudioBuffer.length,
    fromAudioBuffer.sampleRate,
  )

  console.log('audioBuffer', audioBuffer)
  console.log('fromAudioBuffer', fromAudioBuffer)

  for (let channelI = 0; channelI < audioBuffer.numberOfChannels; ++channelI) {
    const oldChannelData = fromAudioBuffer.getChannelData(channelI)
    const newChannelData = audioBuffer.getChannelData(channelI)
    newChannelData.set(oldChannelData)
  }
  return audioBuffer
}

const getAudioBuffer = async (srcPath: string): Promise<AudioBuffer> => {
  const audioContext = new (
    window.AudioContext ||
    (window as unknown as { webkitAudioContext: AudioContext }).webkitAudioContext
  )()
  const audioData = await new Promise<ArrayBuffer>((resolve, reject) => {
    const request = new XMLHttpRequest()
    request.open('GET', srcPath, true)
    request.responseType = 'arraybuffer'
    request.onload = () => resolve(request.response as ArrayBuffer)
    request.onerror = () => reject(new Error('Failed to load audio data'))
    request.send()
  })

  return new Promise((resolve, reject) => {
    void audioContext.decodeAudioData(
      audioData,
      (buffer) => {
        resolve(buffer)
      },
      (e) => {
        console.log('Error with decoding audio data', e)
        reject(e instanceof Error ? e : new Error('Failed to decode audio data'))
      },
    )
  })
}

export { arrRandom, cloneAudioBuffer, getAudioBuffer }
