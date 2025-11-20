use std::path::Path;
use std::time::Duration;

use anyhow::{Context, Result};
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::tag::Accessor;

#[derive(Debug, Clone, Default)]
pub struct AudioMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub track_number: Option<u32>,
    pub year: Option<i32>,
    pub duration: Option<Duration>,
    pub genre: Option<String>,
    pub replaygain_track_gain: Option<f32>,
    pub replaygain_track_peak: Option<f32>,
}

impl AudioMetadata {
    pub fn from_path(path: &Path) -> Result<Self> {
        let tagged_file = lofty::read_from_path(path)
            .with_context(|| format!("Failed to read audio file: {:?}", path))?;

        let properties = tagged_file.properties();
        let duration = properties.duration();

        let tag = tagged_file
            .primary_tag()
            .or_else(|| tagged_file.first_tag());

        let mut metadata = AudioMetadata {
            duration: Some(duration),
            ..Default::default()
        };

        if let Some(tag) = tag {
            metadata.title = tag.title().map(|s| s.to_string());
            metadata.artist = tag.artist().map(|s| s.to_string());
            metadata.album = tag.album().map(|s| s.to_string());
            metadata.album_artist = tag
                .get_string(&lofty::tag::ItemKey::AlbumArtist)
                .map(|s| s.to_string());
            metadata.track_number = tag.track();
            metadata.year = tag.year().map(|y| y as i32);
            metadata.genre = tag.genre().map(|s| s.to_string());

            metadata.replaygain_track_gain = tag
                .get_string(&lofty::tag::ItemKey::ReplayGainTrackGain)
                .and_then(parse_replaygain_db);
            metadata.replaygain_track_peak = tag
                .get_string(&lofty::tag::ItemKey::ReplayGainTrackPeak)
                .and_then(|s| s.trim().parse().ok());
        }

        if metadata.title.is_none() {
            metadata.title = path
                .file_stem()
                .and_then(|s| s.to_str())
                .map(|s| s.to_string());
        }

        Ok(metadata)
    }
}

fn parse_replaygain_db(s: &str) -> Option<f32> {
    s.trim()
        .trim_end_matches(" dB")
        .trim_end_matches("dB")
        .parse()
        .ok()
}
