type Effect = "Compressor" | "Reverb";

interface ReverbSettingsInterface {
  decay: number;
  preDelay: number;
  mix: number;
  ready: boolean;
}

interface CompressorSettingsInterface {
  attack: number;
  knee: number;
  ratio: number;
  release: number;
  threshold: number;
}

type EffectSettings = ReverbSettingsInterface | CompressorSettingsInterface;

export { Effect, ReverbSettingsInterface, CompressorSettingsInterface, EffectSettings };
