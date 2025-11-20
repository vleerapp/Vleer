mod home;
mod songs;

pub use home::HomeView;
pub use songs::SongsView;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppView {
    Home,
    Songs,
}

impl Default for AppView {
    fn default() -> Self {
        Self::Home
    }
}
