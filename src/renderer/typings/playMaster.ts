interface GetPlayersOptions {
  all?: boolean;
}

class PlayMaster {
  playing: boolean;
  currentTime: number;
  currentPlayers: HTMLCollectionOf<HTMLAudioElement> | undefined;

  constructor() {
    this.playing = false;
    this.currentTime = 0;
  }

  // get currentTime(): number {
  //   return this.getPlayers()[0]?.currentTime || 0;
  // }

  play() {
    const players = this.getPlayers();
    for (let i = 0; i < players.length; i++) {
      const player = players[i];
      player.play();
    }
  }

  playTime(callback:(time:number)=>void, time?:number) {
    this.playing = true;
    const newTime = time || this.currentTime;
    this.currentTime = newTime;
    setTimeout(() => {
      callback(newTime);
      if(this.playing) this.playTime(callback, (newTime + .05));
    }, 50);
  }

  pauseTime() {
    this.playing = false;
  }

  stopTime() {
    this.playing = false;
    this.currentTime = 0;
  }
  
  pause() {
    const currentTime = this.currentTime;
    const players = this.getPlayers({ all: true });
    for (let i = 0; i < players.length; i++) {
      const player = players[i];
      player.pause();
      player.currentTime = currentTime;
    }
  }
  stop() {
    const players = this.getPlayers({ all: true });
    for (let i = 0; i < players.length; i++) {
      const player = players[i];
      player.pause();
      player.currentTime = 0;
    }
  }

  setCurrentTime(currentTime?: number): void {
    currentTime = currentTime || this.currentTime;
    const players = this.getPlayers({ all: true });
    for (let i = 0; i < players.length; i++) {
      const player = players[i];
      player.currentTime = currentTime;
    }
  }

  updateCurrentPlayers(): void {
    this.currentPlayers = this.getPlayers();
  }

  getPlayers(options?: GetPlayersOptions): HTMLCollectionOf<HTMLAudioElement> {
    const all = options?.all || false;
    const players = document.getElementsByClassName(
      all ? "player" : "playable"
    ) as HTMLCollectionOf<HTMLAudioElement>;
    return players;
  }
}

export default PlayMaster;
