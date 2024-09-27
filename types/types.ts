export interface EQSettings {
  [key: string]: string
}

export interface History {
  date_played: Date;
  id: string;
  song: Song;
}

export interface Playlist {
  id: string;
  date_created: Date;
  name: string;
  songs: Song[];
}

export interface Settings {
  api_url: string;
  current_song: Song | null;
  eq: EQSettings;
  lossless: boolean;
  loop: boolean;
  muted: boolean;
  queue: Song[];
  shuffle: boolean;
  streaming: boolean;
  volume: number;
}

export interface Song {
  album: string;
  artist: string;
  cover: string;
  date_added: Date;
  duration: number;
  id: string;
  title: string;
}