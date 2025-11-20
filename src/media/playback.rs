use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{Context, Result};
use gpui::{App, Global};
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink, Source};
use tracing::debug;

use super::equalizer::{Equalizer, EqualizerSource};
use crate::data::settings::Settings;

pub struct PlaybackContext {
    _stream: Arc<OutputStream>,
    sink: Arc<Sink>,
    equalizer: Arc<Mutex<Equalizer>>,
    volume: f32,
    is_paused: bool,
    current_file: Option<String>,
}

impl Global for PlaybackContext {}

impl PlaybackContext {
    const LOG_VOLUME_GROWTH_RATE: f32 = 6.908;
    const LOG_VOLUME_SCALE_FACTOR: f32 = 1000.0;
    const UNITY_GAIN: f32 = 1.0;

    pub fn new() -> Result<Self> {
        let stream = OutputStreamBuilder::open_default_stream()
            .context("Failed to open default audio output stream")?;

        let sink = Sink::connect_new(&stream.mixer());

        sink.pause();

        let equalizer = Arc::new(Mutex::new(Equalizer::new(44100, 2)));

        Ok(Self {
            _stream: Arc::new(stream),
            sink: Arc::new(sink),
            equalizer,
            volume: 0.5,
            is_paused: true,
            current_file: None,
        })
    }

    pub fn init(cx: &mut App) -> Result<()> {
        let mut context = Self::new()?;

        if let Some(settings) = cx.try_global::<Settings>() {
            context.apply_settings(settings);
        }

        cx.set_global(context);
        Ok(())
    }

    pub fn load_file(&mut self, path: impl AsRef<Path>, settings: &Settings) -> Result<()> {
        let path = path.as_ref();
        debug!("Loading audio file: {:?}", path);

        let file = File::open(path)
            .with_context(|| format!("Failed to open audio file: {:?}", path))?;

        let source = Decoder::new(BufReader::new(file))
            .context("Failed to decode audio file")?;

        let sample_rate = source.sample_rate();
        let channels = source.channels();

        debug!("Audio file info: {}Hz, {} channels", sample_rate, channels);

        *self.equalizer.lock().unwrap() =
            Equalizer::from_settings(sample_rate, &settings.config().equalizer);

        self.sink.stop();

        let eq_source = EqualizerSource::new(source, self.equalizer.clone());
        self.sink.append(eq_source);
        self.sink.set_volume(Self::log_volume(self.volume));

        self.current_file = Some(path.to_string_lossy().to_string());
        self.is_paused = true;

        debug!("Successfully loaded audio file");
        Ok(())
    }

    pub fn play(&mut self) {
        if self.is_paused {
            self.sink.play();
            self.is_paused = false;
            debug!("Playback started");
        }
    }

    pub fn pause(&mut self) {
        if !self.is_paused {
            self.sink.pause();
            self.is_paused = true;
            debug!("Playback paused");
        }
    }

    pub fn toggle_play_pause(&mut self) {
        if self.is_paused {
            self.play();
        } else {
            self.pause();
        }
    }

    pub fn stop(&mut self) {
        self.sink.stop();
        self.is_paused = true;
        self.current_file = None;
        debug!("Playback stopped");
    }

    pub fn set_volume(&mut self, vol: f32) {
        self.volume = vol.clamp(0.0, 1.0);
        let actual_vol = Self::log_volume(self.volume);
        self.sink.set_volume(actual_vol);
        debug!("Volume set to {:.2} (actual: {:.2})", self.volume, actual_vol);
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    pub fn is_playing(&self) -> bool {
        !self.is_paused
    }

    pub fn current_file(&self) -> Option<&str> {
        self.current_file.as_deref()
    }

    pub fn apply_equalizer_settings(&mut self, gains: &[f32], q_values: &[f32]) {
        let mut eq = self.equalizer.lock().unwrap();
        for i in 0..10.min(gains.len()).min(q_values.len()) {
            eq.set_gain(i, gains[i]);
            eq.set_q(i, q_values[i]);
        }
        debug!("Applied equalizer settings");
    }

    pub fn set_equalizer_gain(&mut self, band: usize, gain_db: f32) {
        if band < 10 {
            self.equalizer.lock().unwrap().set_gain(band, gain_db);
        }
    }

    pub fn set_equalizer_q(&mut self, band: usize, q: f32) {
        if band < 10 {
            self.equalizer.lock().unwrap().set_q(band, q);
        }
    }

    pub fn set_equalizer_enabled(&mut self, enabled: bool) {
        if !enabled {
            let mut eq = self.equalizer.lock().unwrap();
            for i in 0..10 {
                eq.set_gain(i, 0.0);
            }
            debug!("Equalizer disabled");
        } else {
            debug!("Equalizer enabled");
        }
    }

    pub fn get_equalizer_gains(&self) -> Vec<f32> {
        let eq = self.equalizer.lock().unwrap();
        eq.get_bands().iter().map(|b| b.gain_db).collect()
    }

    pub fn get_equalizer_qs(&self) -> Vec<f32> {
        let eq = self.equalizer.lock().unwrap();
        eq.get_bands().iter().map(|b| b.q).collect()
    }

    pub fn load_equalizer_from_settings(&mut self, settings: &Settings) {
        let config = settings.config();
        let eq_settings = &config.equalizer;

        let mut eq = self.equalizer.lock().unwrap();
        eq.apply_settings(eq_settings);

        debug!("Loaded equalizer settings from config (enabled: {})", eq_settings.enabled);
    }

    pub fn apply_settings(&mut self, settings: &Settings) {
        let config = settings.config();

        self.set_volume(config.audio.volume as f32 / 100.0);

        self.load_equalizer_from_settings(settings);

        debug!("Applied all settings to playback context");
    }

    fn log_volume(volume: f32) -> f32 {
        let mut amplitude = volume;
        if amplitude > 0.0 && amplitude < Self::UNITY_GAIN {
            amplitude =
                f32::exp(Self::LOG_VOLUME_GROWTH_RATE * volume) / Self::LOG_VOLUME_SCALE_FACTOR;
            if volume < 0.1 {
                amplitude *= volume * 10.0;
            }
        }
        amplitude
    }

    pub fn seek(&mut self, position: Duration) -> Result<()> {
        self.sink.try_seek(position)
            .map_err(|e| anyhow::anyhow!("Failed to seek to position: {:?}", e))
    }

    pub fn is_empty(&self) -> bool {
        self.sink.empty()
    }
}
