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
  audioContext: AudioContext | null;
  sourceNode: MediaElementAudioSourceNode | null;
  eqFilters: BiquadFilterNode[];
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

export interface PlayerSettings {
  volume: number;
  currentSong: string;
  eq: Record<string, string>;
}

export interface UserSettings {
  playerSettings: PlayerSettings;
}

export const defaultSettings: UserSettings = {
  playerSettings: {
    volume: 100,
    currentSong: "",
    eq: {
      "32": "0.0",
      "64": "0.0",
      "125": "0.0",
      "250": "0.0",
      "500": "0.0",
      "1000": "0.0",
      "2000": "0.0",
      "4000": "0.0",
      "8000": "0.0",
      "16000": "0.0",
    },
  },
};