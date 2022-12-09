module.exports = class InputIterator {
  lines;
  index;

  constructor(input) {
    this.index = 0;
    this.lines = input.split("\n");
  }

  hasNext() {
    return this.index < this.lines.length;
  }

  next() {
    return this.lines[this.index++];
  }

  goBack() {
    this.index--;
  }
}
