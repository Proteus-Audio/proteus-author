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
  const audioContext = new (window.AudioContext || (window as any).webkitAudioContext)()
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
  const audioContext = new (window.AudioContext || (window as any).webkitAudioContext)()
  const audioData = (await new Promise((resolve, reject) => {
    const request = new XMLHttpRequest()
    request.open('GET', srcPath, true)
    request.responseType = 'arraybuffer'
    request.onload = () => resolve(request.response as ArrayBuffer)
    request.onerror = (e) => reject(e)
    request.send()
  })) as ArrayBuffer

  return new Promise((resolve, reject) => {
    audioContext.decodeAudioData(
      audioData,
      (buffer) => {
        resolve(buffer)
      },
      (e) => {
        console.log('Error with decoding audio data', e)
        reject(e)
      },
    )
  })
}

export { arrRandom, cloneAudioBuffer, getAudioBuffer }
