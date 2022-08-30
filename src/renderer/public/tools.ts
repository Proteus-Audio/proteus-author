const arrRandom = (arr: any[]) => {
    console.log(arr);
    if (arr.length === 0) return;
    return arr[Math.floor(Math.random() * arr.length)];
  };
  
  export default { arrRandom };
  