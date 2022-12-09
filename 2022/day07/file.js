module.exports = class File {
  name;
  size;

  constructor(name, size) {
    this.name = name;
    this.size = parseInt(size);
  }

  getName() {
    return this.name;
  }

  getSize() {
    return this.size;
  }
}
