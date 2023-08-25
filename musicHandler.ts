import { audioDir, join } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/tauri";
import {
  copyFile,
  BaseDirectory,
  createDir,
  exists,
  writeTextFile,
  readTextFile,
} from "@tauri-apps/api/fs";
import { invoke } from "@tauri-apps/api";

export class MusicHandler {
  private static instance: MusicHandler | null = null;
  public audio: any;
  public history: Array<string>;
  public playNext: Array<string>;

  private src = "";

  private playEvent: (() => void)[] = [];

  onPlayEvent(listener: () => void) {
    this.playEvent.push(listener);
  }

  private onPlay() {
    for (const listener of this.playEvent) {
      listener();
    }
  }

  constructor() {
    this.audio = new Audio();
    this.audio.addEventListener('ended', this.onAudioEnded.bind(this));
    this.audio.addEventListener('timeupdate', this.onAudioTimeUpdate.bind(this));
  }

  static getInstance(): MusicHandler {
    if (!MusicHandler.instance) {
      MusicHandler.instance = new MusicHandler();
    }
    return MusicHandler.instance;
  }
  
  async setTrack(track: string) {
    this.src = track;
    var assetUrl = await this.createAssetUrl(track);
    this.audio.src = assetUrl;
  }

  playAudio() {
    try {
      this.audio.pause();   
    }
    catch (e) {
      console.log(e);
    }
    this.audio.currentTime = 0;
    this.audio.play();
    this.onPlay();
    this.updatePlayer()
  }

  volume(volume: number) {
    this.audio.volume = volume;
  }

  async saveVolume() {}

  async getVolume() {}

  set(time: number) {
    this.audio.currentTime = time;
  }

  pause() {
    this.audio.pause();
  }

  play() {
    this.audio.play();
  }

  getLenght() {
    return this.audio.duration;
  }

  pauseplay() {
    this.audio.paused != this.audio.paused;
  }

  onAudioEnded() {
    document.getElementById("pauseplay").src = "/svg/bold/play.svg";
  }

  onAudioTimeUpdate() {
    document.getElementById("pauseplay").src = "/svg/bold/play.svg";
  }

  async createAssetUrl(file: string) {
    const audioDirPath = await audioDir();
    const filePath = await join(audioDirPath, `savedMusic/${file}`);
    const assetUrl = convertFileSrc(filePath);

    return assetUrl;
  }

  async getInfo() {
    var musicInfo = JSON.parse(await readTextFile(`savedMusic/${this.src}.json`, {
      dir: BaseDirectory.Audio,
    }));

    if (musicInfo.imageURL == "" || musicInfo.imageURL == null) {
      musicInfo.imageURL = "/unknown.png";
    }
    if (musicInfo.trackName == "") {
      musicInfo.trackName = "Unknown Name";
    }
    if (musicInfo.artistName == "") {
      musicInfo.artistName = "Unknown Artist";
    }

    return musicInfo;
  }

  async updatePlayer() {
    var name = document.getElementById("name")
    var artist = document.getElementById("artist")
    var img = document.getElementById("img")
    name.classList.remove("empty");
    artist.classList.remove("empty");
    img.classList.remove("empty");
    var musicInfo = await this.getInfo();
    document.getElementById("img").style.setProperty("--bgsrc", `url('${musicInfo.imageURL}')`);
    document.getElementById("name").innerHTML = musicInfo.trackName;
    document.getElementById("artist").innerHTML = musicInfo.artistName;
    document.getElementById("pauseplay").src = "/svg/bold/pause.svg";
  }
}