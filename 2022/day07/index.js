const InputIterator = require("./iterator");
const Directory = require("./directory");
const File = require("./file");
const fs = require('fs');

const input = fs.readFileSync("./input.txt", "utf-8").trim();
const iterator = new InputIterator(input);
iterator.next() // skip the first `$ cd /`

const root = new Directory("/", null);
let currentDirectory = root;

while (iterator.hasNext()) {
  let line = iterator.next();

  if (line.startsWith("$ ")) {
    line = line.slice(2);
    let command = line.split(/\s+/)[0];

    if (command == "cd") {
      let newDirectory = currentDirectory.getDirectoryByName(line.split(" ")[1]);

      if (newDirectory) {
        currentDirectory = newDirectory;
      }
      continue;
    }
    
    if (command == "ls") {
      while (iterator.hasNext()) {
        let file = iterator.next();

        if (file.startsWith("$ ")) {
          iterator.goBack();
          break;
        }

        let split = file.split(" ");
        if (split[0] == "dir") {
          currentDirectory.addDirectory(new Directory(split[1], currentDirectory));
          continue;
        }

        currentDirectory.addFile(new File(split[1], split[0]));
      }
    }
  }
}

const maximumSizeToCount = 100_000;
let totalSize = 0;

for (let directory of root.getAllDirectoriesAsArray()) {
  const size = directory.getTotalSize();
  if (size <= maximumSizeToCount) {
    totalSize += size;
  }
}

console.log(
  "The sum of all directories with a size less than",
  maximumSizeToCount,
  "is:",
  totalSize
);
console.log("\n---------------------------\n");

const diskSpace = 70_000_000
const updateSize = 30_000_000
const freeSpace = diskSpace - root.getTotalSize();
const neededSpace = updateSize - freeSpace;

let directoryToDelete = null;

for (let directory of root.getAllDirectoriesAsArray()) {
  let size = directory.getTotalSize();

  if (size < neededSpace) {
    continue;
  }
  
  if (!directoryToDelete) {
    directoryToDelete = directory;
    continue;
  }

  if (directoryToDelete.getTotalSize() > size) {
    directoryToDelete = directory;
  }
}

console.log("Directory to delete:");
console.log("  Name:", directoryToDelete.getName());
console.log("  Size:", directoryToDelete.getTotalSize());
