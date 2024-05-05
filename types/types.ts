export interface Song {
  id: string;
  title: string;
  artist: string;
  length: number;
  cover: string;
  coverURL?: string;
  date_added: string;
  lastPlayed?: string;
}

export interface Playlist {
  id: string;
  name: string;
  date: Date;
  cover: string;
  songs: string[];
}

export interface SongsConfig {
  songs: Record<string, Song>;
  playlists: Record<string, Playlist>;
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
  lastUpdated: number;
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

export interface EQSettings {
  "32": string;
  "64": string;
  "125": string;
  "250": string;
  "500": string;
  "1000": string;
  "2000": string;
  "4000": string;
  "8000": string;
  "16000": string;
}

export type EQSettingsKeys = keyof EQSettings;

export type EQ = keyof EQSettings;

export interface PlayerSettings {
  volume: number;
  currentSong: string;
  eq: EQSettings;
}

export interface UserSettings {
  playerSettings: PlayerSettings;
  apiURL: string;
}

export const defaultSettings: UserSettings = {
  playerSettings: {
    volume: 50,
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
  apiURL: "",
};