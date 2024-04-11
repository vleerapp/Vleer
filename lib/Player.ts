import { readSongs } from './Config';

class Player {
  private static instance: Player;
  public audio: HTMLAudioElement;
  private currentSongId: string;
  private songsData: Record<string, any>;

  private constructor() {
    if (!Player.instance) {
      this.audio = new Audio();
      this.audio.volume = 1;
      this.audio.preload = 'auto';
      this.currentSongId = '';
      this.songsData = {};
      this.initializeSongsData();
      Player.instance = this;
    }
    return Player.instance;
  }

  private async initializeSongsData() {
    try {
      const songsConfig = await readSongs();
      this.songsData = songsConfig.songs;
    } catch (error) {
      console.error('Failed to initialize songs data:', error);
    }
  }

  public async setSong(id: string) {
    this.currentSongId = id;
    const song = this.songsData[id];
    if (song) {
      const songData = await this.getSongData(id);
      const blob = new Blob([songData], { type: 'audio/webm' });
      const url = URL.createObjectURL(blob);
      this.audio.src = url;
      this.audio.addEventListener('error', (e) => {
        console.error('Error with audio element:', e);
      });
    } else {
      console.error('Song not found in songsData');
    }
  }

  private async getSongData(id: string): Promise<ArrayBuffer> {
    try {
      const response = await window.__TAURI__.core.invoke('get_song_data', { songId: id });
      return new Uint8Array(response).buffer;
    } catch (error) {
      console.error('Failed to get song data:', error);
      throw new Error('Failed to get song data');
    }
  }

  public play() {
    this.audio.play();
  }

  public pause() {
    this.audio.pause();
  }

  public async skip() {
    const songIds = Object.keys(this.songsData);
    const currentIndex = songIds.indexOf(this.currentSongId);
    const nextIndex = (currentIndex + 1) % songIds.length;
    await this.setSong(songIds[nextIndex]);
  }

  public async rewind() {
    const songIds = Object.keys(this.songsData);
    const currentIndex = songIds.indexOf(this.currentSongId);
    const prevIndex = (currentIndex - 1 + songIds.length) % songIds.length;
    await this.setSong(songIds[prevIndex]);
  }

  public setVolume(value: number) {
    this.audio.volume = value;
  }

  public getTitle(): string {
    return this.songsData[this.currentSongId]?.title || 'Unknown Title';
  }

  public getArtist(): string {
    return this.songsData[this.currentSongId]?.artist || 'Unknown Artist';
  }

  public async getCover(id: string): Promise<string> {
    return this.songsData[id]?.cover || '';
  }

  public getDuration(): number {
    return this.audio.duration || 0;
  }

  public getCurrentTime(): number {
    return this.audio.currentTime || 0;
  }

  public static getInstance(): Player {
    if (!Player.instance) {
      Player.instance = new Player();
    }
    return Player.instance;
  }
}

export default Player;