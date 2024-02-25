export default class Download {
  static async downloadVideoAsMp3(url: string, name: string): Promise<void> {
    try {
      await window.__TAURI__.core.invoke('download', { 
        url,
        name: name
      });
    } catch (error) {
      console.error('Error downloading video as MP3:', error);
      throw error;
    }
  }
}