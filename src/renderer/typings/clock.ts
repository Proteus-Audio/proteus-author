class Clock {
  currentTime: number;
  playing: boolean;
  moment: Date;

  constructor() {
    this.currentTime = 0;
    this.playing = false;
    this.moment = new Date();
  }

  get seconds(): number {
    if (this.playing) this._setTimeFromDate();
    return this.currentTime;
  }

  private _setTimeFromDate() {
    const now = new Date();
    this.currentTime += ((now.getTime() - this.moment.getTime()) / 1000);
    this.moment = now;
  }

  play() {
    this.playing = true;
    this.moment = new Date();
  }

  pause() {
    this.playing = false;
    this._setTimeFromDate();
  }

  stop() {
    this.playing = false;
    this.currentTime = 0;
  }

  seek(time:number) {
    this.currentTime = time;
  }
}

export { Clock };
