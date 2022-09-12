/* eslint-disable  @typescript-eslint/no-explicit-any */
const arrRandom = (arr: any[]) => {
  if (arr.length === 0) return
  return arr[Math.floor(Math.random() * arr.length)]
}

export default { arrRandom }
