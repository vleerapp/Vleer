use std::sync::{Arc, RwLock};

pub const DEFAULT_TARGET_LUFS: f32 = -14.0;

const REFERENCE_PEAK: f32 = -1.0;

pub struct NormalizationState {
    pub enabled: bool,
    pub target_lufs: f32,
    pub gain: f32,
}

impl Default for NormalizationState {
    fn default() -> Self {
        Self {
            enabled: false,
            target_lufs: DEFAULT_TARGET_LUFS,
            gain: 1.0,
        }
    }
}

impl NormalizationState {
    pub fn new(enabled: bool, target_lufs: f32) -> Self {
        Self {
            enabled,
            target_lufs,
            gain: 1.0,
        }
    }

    pub fn set_track_loudness(&mut self, track_lufs: f32) {
        if !self.enabled {
            self.gain = 1.0;
            return;
        }

        let gain_db = self.target_lufs - track_lufs;

        let gain_db = gain_db.min(12.0);

        self.gain = 10.0f32.powf(gain_db / 20.0);
    }

    pub fn set_replaygain(&mut self, gain_db: f32, peak: Option<f32>) {
        if !self.enabled {
            self.gain = 1.0;
            return;
        }

        let mut linear_gain = 10.0f32.powf(gain_db / 20.0);

        if let Some(peak) = peak {
            if peak > 0.0 {
                let max_gain = 1.0 / peak;
                linear_gain = linear_gain.min(max_gain);
            }
        }

        self.gain = linear_gain;
    }

    pub fn reset(&mut self) {
        self.gain = 1.0;
    }
}

pub struct NormalizationSource<S> {
    inner: S,
    state: Arc<RwLock<NormalizationState>>,
}

impl<S: rodio::Source<Item = f32>> NormalizationSource<S> {
    pub fn new(inner: S, state: Arc<RwLock<NormalizationState>>) -> Self {
        Self { inner, state }
    }
}

impl<S: rodio::Source<Item = f32>> Iterator for NormalizationSource<S> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = self.inner.next()?;
        let state = self.state.read().unwrap();

        if state.enabled {
            let amplified = sample * state.gain;
            Some(soft_clip(amplified))
        } else {
            Some(sample)
        }
    }
}

impl<S: rodio::Source<Item = f32>> rodio::Source for NormalizationSource<S> {
    fn current_span_len(&self) -> Option<usize> {
        self.inner.current_span_len()
    }

    fn channels(&self) -> u16 {
        self.inner.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.inner.sample_rate()
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.inner.total_duration()
    }
}

fn soft_clip(sample: f32) -> f32 {
    if sample.abs() <= 1.0 {
        sample
    } else {
        sample.signum() * (1.0 - (-sample.abs() + 1.0).exp().recip())
    }
}

pub fn rms_to_approximate_lufs(rms: f32) -> f32 {
    if rms <= 0.0 {
        return -70.0;
    }
    20.0 * rms.log10()
}
