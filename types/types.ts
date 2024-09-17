export interface Song {
  id: string;
  title: string;
  artist: string;
  album: string;
  cover: string;
  date_added: Date;
  duration: number;
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
  eq: {
    [key: string]: string;
  };
  lossless: boolean;
  loop: boolean;
  muted: boolean;
  queue: Song[];
  shuffle: boolean;
  streaming: boolean;
  volume: number;
}