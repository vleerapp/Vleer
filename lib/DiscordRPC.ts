// lib/DiscordRPC.ts

export default class DiscordRPC {
  static async update(state: string, details: string, largeImage: string, largeImageText: string, smallImage: string, smallImageText: string): Promise<void> {
    console.log('Updating Discord RPC with:', { state, details, largeImage, largeImageText, smallImage, smallImageText });
    try {
      console.log('Invoking update_activity command...');
      await window.__TAURI__.core.invoke('update_activity', {
        state,
        details,
        largeImage,
        largeImageText,
        smallImage,
        smallImageText
      });
      console.log('Successfully invoked update_activity command');
    } catch (error) {
      console.error('Failed to update Discord RPC:', error);
    }
  }
}