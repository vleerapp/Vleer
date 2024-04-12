export interface Song {
  id: string;
  title: string;
  artist: string;
  length: number;
  cover: string;
  coverURL?: string;
  date_added: string;
}

export interface SongsConfig {
  songs: Record<string, Song>;
}

export interface Player {
  audio: HTMLAudioElement;
  currentSongId: string;
}

export interface MusicStore {
  songsConfig: SongsConfig;
  player: Player;
}