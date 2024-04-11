import { readSongs } from './Config';

class Player {
  private audio: HTMLAudioElement;
  private currentSongId: string;
  private songsData: Record<string, any>;

  constructor() {
    this.audio = new Audio();
    this.audio.preload = 'auto';
    this.audio.crossOrigin = 'anonymous';
    this.currentSongId = '';
    this.songsData = {};
    this.initializeSongsData();
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
      const path = await this.getPath(this.currentSongId);
      console.log("Audio path:", path); // Log the path for verification
      this.audio.src = path;
      this.audio.load(); // Ensure the audio is loaded
      this.audio.addEventListener('canplay', () => {
        console.log('Audio can play, attempting to play...');
        this.play();
      }, { once: true });
      this.audio.addEventListener('error', (e) => {
        console.error('Error with audio element:', e);
      });
      this.audio.addEventListener('loadedmetadata', () => {
        console.log('Metadata loaded, duration:', this.audio.duration);
      });
      this.audio.addEventListener('loadstart', () => console.log('Audio loading started'));
      this.audio.addEventListener('loadeddata', () => console.log('Audio data loaded'));
      this.audio.addEventListener('canplay', () => console.log('Audio can play'));
      this.audio.addEventListener('error', () => console.error('Audio playback error'));
      this.audio.addEventListener('abort', () => console.log('Audio loading aborted'));
      this.audio.addEventListener('suspend', () => console.log('Audio loading suspended'));
      this.audio.addEventListener('error', (e) => {
        console.error('Audio loading error', e);
      });
    } else {
      console.error('Song not found in songsData');
    }
  }

  public async getPath(id){
    const path = await window.__TAURI__.core.invoke("get_path")
    return path + "/" + id + ".webm"
  }

  public play() {
    this.audio.play().catch(e => console.error('Error playing audio:', e));
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

  public getCover(): string {
    return this.songsData[this.currentSongId]?.cover || '';
  }

  public getDuration(): number {
    return this.audio.duration || 0;
  }

  public getCurrentTime(): number {
    return this.audio.currentTime || 0;
  }
}

export default Player;