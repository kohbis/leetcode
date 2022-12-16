class MyQueue {
  private queue: number[];

  constructor() {
    this.queue = [];
  }

  push(x: number): void {
    this.queue.push(x);
    return;
  }

  pop(): number {
    if (this.empty()) {
      return -1;
    } else {
      return this.queue.shift();
    }
  }

  peek(): number {
    if (this.empty()) {
      throw new Error("Unreachable!");
    } else {
      return this.queue[0];
    }
  }

  empty(): boolean {
    return this.queue.length == 0;
  }
}

/**
 * Your MyQueue object will be instantiated and called as such:
 * var obj = new MyQueue()
 * obj.push(x)
 * var param_2 = obj.pop()
 * var param_3 = obj.peek()
 * var param_4 = obj.empty()
 */
