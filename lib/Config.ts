interface Song {
  id: string;
  title: string;
  artist: string;
  length: number;
  cover: string;
  dateAdded: string;
}

interface SongsConfig {
  songs: Record<string, Song>;
}

export const writeSong = async (song: Song): Promise<void> => {
  await window.__TAURI__.invoke('write_song_wrapper', {
    id: song.id,
    title: song.title,
    artist: song.artist,
    length: song.length,
    cover: song.cover,
    dateAdded: song.dateAdded,
  });
};

export const readSongs = async (): Promise<SongsConfig> => {
  return await window.__TAURI__.core.invoke('read_songs_wrapper');
};