-- Create artists table
CREATE TABLE IF NOT EXISTS artists (
    id TEXT PRIMARY KEY, -- CUID format: e.g., 'clh3sa0f70000jv0g3yt3d4ko'
    name TEXT UNIQUE NOT NULL,
    image TEXT,
    favorite BOOLEAN DEFAULT FALSE
);

-- Create albums table
CREATE TABLE IF NOT EXISTS albums (
    id TEXT PRIMARY KEY, -- CUID
    title TEXT NOT NULL,
    artist TEXT, -- References artists.id (CUID)
    cover TEXT,
    favorite BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (artist) REFERENCES artists(id) ON DELETE CASCADE
);

-- Create songs table
CREATE TABLE IF NOT EXISTS songs (
    id TEXT PRIMARY KEY, -- CUID
    title TEXT NOT NULL,
    artist_id TEXT, -- References artists.id (CUID)
    album_id TEXT, -- References albums.id (CUID)
    genre TEXT,
    date TEXT, -- SQLite doesn't have a native DATE type, uses TEXT
    date_added TEXT DEFAULT (DATE('now')),
    duration INTEGER, -- Duration in seconds
    cover TEXT,
    track_number INTEGER,
    favorite BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (artist_id) REFERENCES artists(id) ON DELETE CASCADE,
    FOREIGN KEY (album_id) REFERENCES albums(id) ON DELETE CASCADE
);

-- Create playlists table
CREATE TABLE IF NOT EXISTS playlists (
    id TEXT PRIMARY KEY, -- CUID
    name TEXT NOT NULL,
    description TEXT,
    image TEXT,
    date_created TEXT DEFAULT (DATE('now')),
    date_updated TEXT DEFAULT (DATE('now'))
);

-- Create events table for playback events
CREATE TABLE IF NOT EXISTS events (
    id TEXT PRIMARY KEY, -- CUID
    event_type TEXT CHECK(event_type IN ('PLAY', 'STOP', 'PAUSE', 'RESUME')) NOT NULL,
    context_id TEXT, -- References event_contexts.id (CUID)
    timestamp TEXT DEFAULT (DATETIME('now')),
    FOREIGN KEY (context_id) REFERENCES event_contexts(id) ON DELETE CASCADE
);

-- Create event_contexts table for event context information
CREATE TABLE IF NOT EXISTS event_contexts (
    id TEXT PRIMARY KEY, -- CUID
    song_id TEXT, -- References songs.id (CUID)
    playlist_id TEXT, -- References playlists.id (CUID)
    FOREIGN KEY (song_id) REFERENCES songs(id) ON DELETE CASCADE,
    FOREIGN KEY (playlist_id) REFERENCES playlists(id) ON DELETE CASCADE
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_songs_artist ON songs(artist_id);
CREATE INDEX IF NOT EXISTS idx_songs_album ON songs(album_id);
CREATE INDEX IF NOT EXISTS idx_albums_artist ON albums(artist);
CREATE INDEX IF NOT EXISTS idx_events_type ON events(event_type);
CREATE INDEX IF NOT EXISTS idx_events_timestamp ON events(timestamp);
CREATE INDEX IF NOT EXISTS idx_events_context ON events(context_id);
CREATE INDEX IF NOT EXISTS idx_event_contexts_song ON event_contexts(song_id);
CREATE INDEX IF NOT EXISTS idx_event_contexts_playlist ON event_contexts(playlist_id);
CREATE INDEX IF NOT EXISTS idx_songs_favorite ON songs(favorite);
CREATE INDEX IF NOT EXISTS idx_albums_favorite ON albums(favorite);
CREATE INDEX IF NOT EXISTS idx_artists_favorite ON artists(favorite);

-- View for song details with artist and album information
CREATE VIEW IF NOT EXISTS song_details AS
SELECT 
    s.id,
    s.title,
    s.duration,
    s.track_number,
    s.genre,
    s.date_added,
    s.favorite,
    a.name as artist_name,
    al.title as album_title,
    al.cover as album_cover
FROM songs s
LEFT JOIN artists a ON s.artist_id = a.id
LEFT JOIN albums al ON s.album_id = al.id;

-- View for playback history
CREATE VIEW IF NOT EXISTS playback_history AS
SELECT 
    e.id as event_id,
    e.event_type,
    e.timestamp,
    s.title as song_title,
    a.name as artist_name,
    p.name as playlist_name
FROM events e
LEFT JOIN event_contexts ec ON e.context_id = ec.id
LEFT JOIN songs s ON ec.song_id = s.id
LEFT JOIN artists a ON s.artist_id = a.id
LEFT JOIN playlists p ON ec.playlist_id = p.id
ORDER BY e.timestamp DESC;
