use log::warn;
use proteus_lib::container::play_settings::EffectSettings;
use proteus_lib::dsp::effects::{normalize_legacy_effect_aliases, AudioEffect};

pub fn decode_effects(raw_effects: &[EffectSettings]) -> Vec<AudioEffect> {
    let mut decoded = Vec::with_capacity(raw_effects.len());

    for effect in raw_effects {
        match serde_json::from_value::<AudioEffect>(effect.clone()) {
            Ok(effect) => decoded.push(effect),
            Err(err) => warn!("failed to parse effect entry: {}", err),
        }
    }

    normalize_legacy_effect_aliases(decoded)
}
