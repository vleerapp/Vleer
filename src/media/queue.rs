use std::path::PathBuf;

use gpui::{App, Global};
use tracing::debug;

use crate::data::types::Cuid;

#[derive(Clone, Debug)]
pub struct QueueItem {
    pub song_id: Option<Cuid>,
    pub path: PathBuf,
    pub title: Option<String>,
    pub artist_name: Option<String>,
    pub album_name: Option<String>,
    pub duration: Option<i32>,
}

impl QueueItem {
    pub fn new(path: PathBuf) -> Self {
        Self {
            song_id: None,
            path,
            title: None,
            artist_name: None,
            album_name: None,
            duration: None,
        }
    }

    pub fn with_metadata(
        path: PathBuf,
        song_id: Option<Cuid>,
        title: Option<String>,
        artist_name: Option<String>,
        album_name: Option<String>,
        duration: Option<i32>,
    ) -> Self {
        Self {
            song_id,
            path,
            title,
            artist_name,
            album_name,
            duration,
        }
    }
}

pub struct Queue {
    items: Vec<QueueItem>,
    current_index: Option<usize>,
    shuffle: bool,
    repeat: RepeatMode,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RepeatMode {
    Off,
    All,
    One,
}

impl Global for Queue {}

impl Queue {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            current_index: None,
            shuffle: false,
            repeat: RepeatMode::Off,
        }
    }

    pub fn init(cx: &mut App) {
        cx.set_global(Queue::new());
    }

    pub fn add(&mut self, item: QueueItem) {
        self.items.push(item);
        if self.current_index.is_none() && !self.items.is_empty() {
            self.current_index = Some(0);
        }
        debug!("Added item to queue. Queue size: {}", self.items.len());
    }

    pub fn add_many(&mut self, items: Vec<QueueItem>) {
        let was_empty = self.items.is_empty();
        self.items.extend(items);
        if was_empty && !self.items.is_empty() {
            self.current_index = Some(0);
        }
        debug!(
            "Added {} items to queue. Queue size: {}",
            self.items.len(),
            self.items.len()
        );
    }

    pub fn current(&self) -> Option<&QueueItem> {
        self.current_index.and_then(|idx| self.items.get(idx))
    }

    pub fn current_index(&self) -> Option<usize> {
        self.current_index
    }

    pub fn next(&mut self) -> Option<&QueueItem> {
        if self.items.is_empty() {
            return None;
        }

        match self.repeat {
            RepeatMode::One => {
                return self.current();
            }
            RepeatMode::All => {
                if let Some(idx) = self.current_index {
                    self.current_index = Some((idx + 1) % self.items.len());
                } else {
                    self.current_index = Some(0);
                }
            }
            RepeatMode::Off => {
                if let Some(idx) = self.current_index {
                    if idx + 1 < self.items.len() {
                        self.current_index = Some(idx + 1);
                    } else {
                        return None;
                    }
                } else {
                    self.current_index = Some(0);
                }
            }
        }

        debug!("Moved to next track. Index: {:?}", self.current_index);
        self.current()
    }

    pub fn previous(&mut self) -> Option<&QueueItem> {
        if self.items.is_empty() {
            return None;
        }

        match self.repeat {
            RepeatMode::One => {
                return self.current();
            }
            RepeatMode::All => {
                if let Some(idx) = self.current_index {
                    if idx == 0 {
                        self.current_index = Some(self.items.len() - 1);
                    } else {
                        self.current_index = Some(idx - 1);
                    }
                } else {
                    self.current_index = Some(0);
                }
            }
            RepeatMode::Off => {
                if let Some(idx) = self.current_index {
                    if idx > 0 {
                        self.current_index = Some(idx - 1);
                    } else {
                        return None;
                    }
                } else {
                    self.current_index = Some(0);
                }
            }
        }

        debug!("Moved to previous track. Index: {:?}", self.current_index);
        self.current()
    }

    pub fn jump_to(&mut self, index: usize) -> Option<&QueueItem> {
        if index < self.items.len() {
            self.current_index = Some(index);
            debug!("Jumped to index {}", index);
            self.current()
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.current_index = None;
        debug!("Queue cleared");
    }

    pub fn remove(&mut self, index: usize) -> Option<QueueItem> {
        if index < self.items.len() {
            let item = self.items.remove(index);

            if let Some(current) = self.current_index {
                if current == index {
                    if self.items.is_empty() {
                        self.current_index = None;
                    } else if current >= self.items.len() {
                        self.current_index = Some(self.items.len() - 1);
                    }
                } else if current > index {
                    self.current_index = Some(current - 1);
                }
            }

            debug!(
                "Removed item at index {}. Queue size: {}",
                index,
                self.items.len()
            );
            Some(item)
        } else {
            None
        }
    }

    pub fn items(&self) -> &[QueueItem] {
        &self.items
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn toggle_shuffle(&mut self) {
        self.shuffle = !self.shuffle;
        debug!("Shuffle: {}", self.shuffle);
    }

    pub fn set_shuffle(&mut self, shuffle: bool) {
        self.shuffle = shuffle;
        debug!("Shuffle set to: {}", shuffle);
    }

    pub fn is_shuffle(&self) -> bool {
        self.shuffle
    }

    pub fn cycle_repeat(&mut self) {
        self.repeat = match self.repeat {
            RepeatMode::Off => RepeatMode::All,
            RepeatMode::All => RepeatMode::One,
            RepeatMode::One => RepeatMode::Off,
        };
        debug!("Repeat mode: {:?}", self.repeat);
    }

    pub fn set_repeat(&mut self, mode: RepeatMode) {
        self.repeat = mode;
        debug!("Repeat mode set to: {:?}", mode);
    }

    pub fn repeat_mode(&self) -> RepeatMode {
        self.repeat
    }

    pub fn has_next(&self) -> bool {
        if self.items.is_empty() {
            return false;
        }

        match self.repeat {
            RepeatMode::One | RepeatMode::All => true,
            RepeatMode::Off => {
                if let Some(idx) = self.current_index {
                    idx + 1 < self.items.len()
                } else {
                    !self.items.is_empty()
                }
            }
        }
    }

    pub fn has_previous(&self) -> bool {
        if self.items.is_empty() {
            return false;
        }

        match self.repeat {
            RepeatMode::One | RepeatMode::All => true,
            RepeatMode::Off => {
                if let Some(idx) = self.current_index {
                    idx > 0
                } else {
                    false
                }
            }
        }
    }
}

impl Default for Queue {
    fn default() -> Self {
        Self::new()
    }
}
