// yourfriend, 2023

// this wrapper is needed for keeping sync track of the
// state between tauri & react

// this could totally be avoided, but because of speed concerns I decided to implement it.

interface Activity {
  details: string;
  state: string;
  image: string;
};

export class DiscordRPC {
  activityQueue: (Activity)[] = []

  lastKnownState: boolean = false;
  interval: number;

  constructor() {
      this.interval = setInterval(async ()=>{
          if(window.__TAURI__) {
              this.lastKnownState = await window.__TAURI__.invoke("get_rpc_state");
              if(this.activityQueue.length != 0 && this.lastKnownState) {
                  let last = this.activityQueue.at(-1)!;
                  this.activityQueue = [];
                  this.sendActivity(last);
              }
          }
      }, 1000) as unknown as number    
  }
  
  error() {
      throw new Error("Every usage of the DiscordRPC module should be safeguarded with a if statement, checking __TAURI__.")
  }

  start() {
      if(!window.__TAURI__) return this.error();
      window.__TAURI__.invoke("start_discord_rpc");    
  }

  clearActivity() {
      if(!window.__TAURI__) return this.error();
      window.__TAURI__.invoke("clear_activity");
  }

  private sendActivity(act: Activity) {
      if(!window.__TAURI__) return this.error();
      window.__TAURI__.invoke("set_discord_rpc", act);
  }

  
  queueActivity(act: Activity) {
      if(!window.__TAURI__) return this.error();
      if(!this.lastKnownState) {
          this.activityQueue.push(act);
      } else {
          this.sendActivity(act);
      }
  }
}

export default new DiscordRPC();
