// import axios from "axios";

/* eslint-disable  @typescript-eslint/no-explicit-any */
const arrRandom = (arr: any[]) => {
  if (arr.length === 0) return
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
  const audioBuffer = new AudioBuffer({
    length: fromAudioBuffer.length,
    numberOfChannels: fromAudioBuffer.numberOfChannels,
    sampleRate: fromAudioBuffer.sampleRate,
  })

  for (let channelI = 0; channelI < audioBuffer.numberOfChannels; ++channelI) {
    const samples = fromAudioBuffer.getChannelData(channelI)
    audioBuffer.copyToChannel(samples, channelI)
  }
  return audioBuffer
}

const getAudioBuffer = async (srcPath: string): Promise<AudioBuffer> => {
  const audioContext = new (window.AudioContext || (window as any).webkitAudioContext)()

  const audioData = (await new Promise((resolve, reject) => {
    const request = new XMLHttpRequest()
    request.open('GET', srcPath, true)
    request.responseType = 'arraybuffer'
    request.onload = () => resolve(request.response as ArrayBuffer)
    request.onerror = (e) => reject(e)
    request.send()
  })) as ArrayBuffer

  const buffer = await audioContext.decodeAudioData(audioData)

  return buffer
}

export { arrRandom, cloneAudioBuffer, getAudioBuffer }
