class Player {
  private audio: HTMLAudioElement;
  private isStopped: boolean;
  private waitingToPlay: boolean;
  private lastVolume: number;
  private startTime: number;

  constructor() {
    this.audio = new Audio();
    this.isStopped = true;
    this.waitingToPlay = false;
    this.lastVolume = 10;
    this.startTime = Date.now();

    this.setupEventListeners();
  }

  private setupEventListeners() {
    this.audio.ontimeupdate = () => { };
    this.audio.oncanplay = () => {
      if (this.waitingToPlay) {
        this.waitingToPlay = false;
        this.startPlayback();
        this.isStopped = false;
      }
    };
    this.audio.onended = () => {
      this.next();
    };
    this.audio.addEventListener('error', () => {
      this.stop();
    });
  }

  private startPlayback() {
    this.audio.play();
    this.isStopped = false;
    this.startTime = Date.now();
  }

  private pausePlayback() {
    this.waitingToPlay = false;
    this.audio.pause();
    this.resetAndSavePlayTime();
  }

  private resetAndSavePlayTime() {
    const playTime = Date.now() - this.startTime;
    this.startTime = Date.now();
  }

  public setVolume(value: number) {
    const volume = Math.min(Math.max(value, 0), 1);
    this.lastVolume = this.audio.volume;
    this.audio.volume = volume;
  }

  public toggleMute() {
    if (this.audio.volume > 0) this.setVolume(0);
    else this.setVolume(this.lastVolume || 1);
  }

  public setPlayingFile(fileUrl: string, paused: boolean = false) {
    this.waitingToPlay = !paused;
    this.audio.src = fileUrl;
  }

  public stop() {
    this.waitingToPlay = false;
    this.audio.pause();
    this.resetAndSavePlayTime();
    this.isStopped = true;
    this.seek(0);
  }

  public playPause() {
    if (this.isStopped) return;
    else if (this.audio.paused) this.startPlayback();
    else this.pausePlayback();
  }

  public next() {
    // Logic to go to the next track
  }

  public previous() {
    // Logic to go to the previous track
  }

  public seek(to: number) {
    const newTime = Math.min(to, this.audio.duration || 0);
    this.audio.currentTime = newTime;
  }

  public getCurrentTime() {
    return this.audio.currentTime;
  }

  public getDuration() {
    return this.audio.duration;
  }

  public getProgress() {
    if (this.audio.duration > 0) {
      return (this.audio.currentTime / this.audio.duration) * 100;
    }
    return 0;
  }

  public async getCover(id: string) {
    try {
      const songData = await window.__TAURI__.core.invoke('read_songs_wrapper');
      return songData.songs[id]?.cover || '';
    } catch (error) {
      console.error('Error fetching cover image:', error);
    }
  }

  public async getTitle(id: string) {
    try {
      const songData = await window.__TAURI__.core.invoke('read_songs_wrapper');
      return songData.songs[id]?.title || 'Unknown Title';
    } catch (error) {
      console.error('Error fetching title:', error);
    }
  }

  public async getArtist(id: string) {
    try {
      const songData = await window.__TAURI__.core.invoke('read_songs_wrapper');
      return songData.songs[id]?.artist || 'Unknown Artist';
    } catch (error) {
      console.error('Error fetching artist:', error);
    }
  }
}

export default Player;