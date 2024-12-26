export interface EQSettings {
  values: { [key: string]: string };
}

export interface History {
  id: string;
  date_played: Date;
  song: Song;
}

export interface Album {
  id: string;
  name: string;
  artist: string;
  cover: string;
  date_added: Date;
  songs: Song[];
}

export interface Playlist {
  id: string;
  name: string;
  date_created: Date;
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
  id: string;
  title: string;
  artist: string;
  album: string;
  cover: string;
  date_added: Date;
  duration: number;
}