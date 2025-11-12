mod home;

pub use home::HomeView;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppView {
    Home,
    // Add more views here as needed
    // Artists,
    // Albums,
    // Playlists,
}

impl Default for AppView {
    fn default() -> Self {
        Self::Home
    }
}
