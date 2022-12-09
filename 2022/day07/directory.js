module.exports = class Directory {
  name;
  directories;
  map;
  files;

  constructor(name, parent) {
    this.name = name;
    this.directories = [];
    this.map = new Map();
    this.files = [];

    this.map.set("..", parent);
  }

  getDirectoryByName(name) {
    return this.map.get(name);
  }

  addDirectory(directory) {
    this.directories.push(directory);

    this.map.set(directory.getName(), directory);
  }
  
  addFile(file) {
    this.files.push(file);
  }

  getName() {
    return this.name;
  }

  getDirectories() {
    return this.directories.slice();
  }

  getFiles() {
    return this.files.slice();
  }

  getTotalSize() {
    let size = 0;

    for (let file of this.files) {
      size += file.getSize();
    }

    for (let dir of this.directories) {
      size += dir.getTotalSize();
    }

    return size;
  }

  getAllDirectoriesAsArray() {
    let dirsArray = [];

    for (let dir of this.directories) {
      let innerDirectories = dir.getAllDirectoriesAsArray();

      if (innerDirectories.length != 0) {
        for (let innerDirectory of innerDirectories) {
          dirsArray.push(innerDirectory);
        }
      }
      dirsArray.push(dir);
    }

    return dirsArray;
  }
}
