type Effect = "Compressor" | "Reverb";

interface ReverbSettings {
  active: boolean;
  decay: number;
  preDelay: number;
  mix: number;
  ready: boolean;
}

interface CompressorSettings {
  active: boolean;
  attack: number;
  knee: number;
  ratio: number;
  release: number;
  threshold: number;
}

export { Effect, ReverbSettings, CompressorSettings };
