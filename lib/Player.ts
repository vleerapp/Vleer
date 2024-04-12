import { readSongs } from './Config';

class Player {
  private static instance: Player;
  public audio: HTMLAudioElement;
  public currentSongId: string;

  private constructor() {
    if (!Player.instance) {
      this.audio = new Audio();
      this.audio.volume = 1;
      this.audio.preload = 'auto';
      this.currentSongId = '';
      Player.instance = this;
    }
    return Player.instance;
  }

  public async setSong(id: string) {
    this.currentSongId = id;
    const songsData = (await readSongs()).songs;
    const song = songsData[id];
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
    const songsData = (await readSongs()).songs;
    const songIds = Object.keys(songsData);
    const currentIndex = songIds.indexOf(this.currentSongId);
    const nextIndex = (currentIndex + 1) % songIds.length;
    await this.setSong(songIds[nextIndex]);
  }

  public async rewind() {
    const songsData = (await readSongs()).songs;
    const songIds = Object.keys(songsData);
    const currentIndex = songIds.indexOf(this.currentSongId);
    const prevIndex = (currentIndex - 1 + songIds.length) % songIds.length;
    await this.setSong(songIds[prevIndex]);
  }

  public setVolume(value: number) {
    this.audio.volume = value;
  }

  public async getTitle(): string {
    const songsData = (await readSongs()).songs;
    return songsData[this.currentSongId]?.title || 'Unknown Title';
  }

  public async getArtist(): string {
    const songsData = (await readSongs()).songs;
    return songsData[this.currentSongId]?.artist || 'Unknown Artist';
  }

  public async getCover(id: string): Promise<string> {
    try {
      const coverBase64 = await window.__TAURI__.core.invoke('get_cover_base64', { id: id });
      const coverBlob = this.base64ToBlob(coverBase64, 'image/jpeg');
      return URL.createObjectURL(coverBlob);
    } catch (error) {
      console.error('Failed to get cover image:', error);
      return '';
    }
  }

  private base64ToBlob(base64: string, mimeType: string): Blob {
    const byteCharacters = atob(base64);
    const byteNumbers = new Array(byteCharacters.length);
    for (let i = 0; i < byteCharacters.length; i++) {
      byteNumbers[i] = byteCharacters.charCodeAt(i);
    }
    const byteArray = new Uint8Array(byteNumbers);
    return new Blob([byteArray], { type: mimeType });
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