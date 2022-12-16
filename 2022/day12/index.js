const fs = require("fs");
const input = fs.readFileSync("./input.txt", "utf-8").trim();

let grid = [];

for (let line of input.split("\n")) {
  grid.push(line.split(""));
}

function getIndexesOfLetter(letter) {
  for (let x = 0; x < grid[0].length; x++) {
    for (let y = 0; y < grid.length; y++) {
      if (grid[y][x] == letter) {
        return {x: x, y: y};
      }
    }
  }
}

const startIndexes = getIndexesOfLetter("S");
const endIndexes = getIndexesOfLetter("E");

grid[startIndexes.y][startIndexes.x] = "a";
grid[endIndexes.y][endIndexes.x] = "z";

const dijkstraMap = new Map();

function isAccessible(from, to) {
  let alphabet = "abcdefghijklmnopqrstuvwxyz";

  if (alphabet.indexOf(to) + 1 >= alphabet.indexOf(from)) {
    return true;
  }

  return false;
}

function visitSlot(x, y, currentSteps) {
  const mapKey = x + "-" + y;
  if (
    dijkstraMap.has(mapKey) &&
    dijkstraMap.get(mapKey) <= currentSteps
  ) {
    return;
  } else {
    dijkstraMap.set(mapKey, currentSteps);
  }

  const nextPositions = [
    {x: x, y: y-1}, // up
    {x: x, y: y+1}, // down
    {x: x+1, y: y}, // right
    {x: x-1, y: y}, // left
  ];

  for (let pos of nextPositions) {
    if (!grid[pos.y] || !grid[pos.y][pos.x]) {
      continue;
    }

    let slot = grid[pos.y][pos.x];
    if (slot && isAccessible(grid[y][x], slot)) {
      visitSlot(pos.x, pos.y, currentSteps + 1);
    }
  }
}

visitSlot(endIndexes.x, endIndexes.y, 0);
let minimumSteps = dijkstraMap.get(startIndexes.x + "-" + startIndexes.y);

console.log(
  "The minimum steps of getting from start to end is:",
  minimumSteps
);

let stepsArray = [minimumSteps];
let startPositions = [];

for (let x = 0; x < grid[0].length; x++) {
  for (let y = 0; y < grid.length; y++) {
    if (grid[y][x] == "a") {
      startPositions.push({x: x, y: y});
    }
  }
}

startPositions = startPositions.filter(p => {
  return !(p.x == startIndexes.x && p.y == startIndexes.y)
});

for (let pos of startPositions) {
  stepsArray.push(dijkstraMap.get(pos.x + "-" + pos.y))
}

minimumSteps = null;

for (let steps of stepsArray) {
  if (minimumSteps == null || minimumSteps > steps) {
    minimumSteps = steps;
  }
}

console.log(
  "The minimum steps starting from anywhere is:",
  minimumSteps
);
