export default class Download {
  static async downloadVideoAsMp3(url: string, output_path: string): Promise<void> {
    try {
      await window.__TAURI__.core.invoke('download', { 
        url, 
        outputPath: output_path 
      });
    } catch (error) {
      console.error('Error downloading video as MP3:', error);
      throw error;
    }
  }
}