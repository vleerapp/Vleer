export default class DiscordRPC {
  static async update(
    state: string,
    details: string,
    largeImage: string,
    largeImageText: string,
  ): Promise<void> {
    try {
      await window.__TAURI__.core.invoke("update_activity", {
        state,
        details,
        largeImage,
        largeImageText,
      });
    } catch (error) {
      console.error("Failed to update Discord RPC:", error);
    }
  }

  static async clear(): Promise<void> {
    try {
      await window.__TAURI__.core.invoke("clear_activity");
    } catch (error) {
      console.error("Failed to disconnect Discord RPC:", error);
    }
  }
}
