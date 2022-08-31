import axios from "axios";

const arrRandom = (arr: any[]) => {
  console.log(arr);
  if (arr.length === 0) return;
  return arr[Math.floor(Math.random() * arr.length)];
};

const srcToFile = async (src: string, fileName: string): Promise<File> => {
  const response = await axios.get(src, {
    responseType: "blob",
  });
  const mimeType = response.headers["content-type"];
  return new File([response.data], fileName, { type: mimeType });
};

export { arrRandom, srcToFile };
