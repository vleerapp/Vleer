// lib/Download.ts

export default class Download {
  static async downloadVideoAsMp3(url: string, output_path: string): Promise<string> {
    console.log(url, output_path)
    try {
      const filePath = await window.__TAURI__.core.invoke('download_youtube_video_as_mp3', { url, outputPath: output_path });
      return filePath as string;
    } catch (error) {
      console.error('Error downloading video as MP3:', error);
      throw error;
    }
  }
}