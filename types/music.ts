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

export interface MusicSearchResponseItem {
  url: string;
  type: string;
  title: string;
  thumbnail: string;
  uploaderName: string;
  uploaderUrl: string;
  uploaderAvatar: any;
  uploadedDate: any;
  shortDescription: any;
  duration: number;
  views: number;
  uploaded: number;
  uploaderVerified: boolean;
  isShort: boolean;
}

export interface MusicSearchResponse {
  items: MusicSearchResponseItem[];
}
