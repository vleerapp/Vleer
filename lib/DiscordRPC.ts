export default class DiscordRPC {
  static async update(
    state: string,
    details: string,
    largeImage: string,
    largeImageText: string,
    smallImage: string,
    smallImageText: string
  ): Promise<void> {
    try {
      await window.__TAURI__.core.invoke("update_activity", {
        state,
        details,
        largeImage,
        largeImageText,
        smallImage,
        smallImageText,
      });
    } catch (error) {
      console.error("Failed to update Discord RPC:", error);
    }
  }
}
