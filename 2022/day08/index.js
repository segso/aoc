const fs = require("fs");
const input = fs.readFileSync("./input.txt", "utf-8").trim().split("\n");

let grid = [];

for (let i = 0; i < input.length; i++) {
  grid[i] = [];
  let line = input[i];
  for (let j = 0; j < line.length; j++) {
    grid[i][j] = parseInt(line.at(j));
  }
}

function isVisible(line, column) {
  let tree = grid[line][column];

  let isUpBlocked = false;
  let upTrees = 0;
  let isLeftBlocked = false;
  let leftTrees = 0;
  let isRightBlocked = false;
  let rightTrees = 0;
  let isDownBlocked = false;
  let downTrees = 0;

  // Look up
  for (let i = line - 1; i >= 0; i--){
    upTrees++;
    if (grid[i][column] >= tree) {
      isUpBlocked = true;
      break;
    }
  }

  // Look left
  for (let i = column - 1; i >= 0; i--){
    leftTrees++;
    if (grid[line][i] >= tree) {
      isLeftBlocked = true;
      break;
    }
  }

  // Look right
  for (let i = column + 1; i < grid[0].length; i++){
    rightTrees++;
    if (grid[line][i] >= tree) {
      isRightBlocked = true;
      break;
    }
  }

  // Look down
  for (let i = line + 1; i < grid.length; i++){
    downTrees++;
    if (grid[i][column] >= tree) {
      isDownBlocked = true;
      break;
    }
  }

  return {
    visible: !(isUpBlocked && isDownBlocked && isLeftBlocked && isRightBlocked),
    scenicScore: upTrees * downTrees * leftTrees * rightTrees
  };
}

let total = 0
let maxScenicScore = 0;

for (let i = 0; i < grid[0].length; i++) {
  for (let j = 0; j < grid.length ; j++) {
    let result = isVisible(i, j);
    total += result.visible;
    if (maxScenicScore < result.scenicScore) {
      maxScenicScore = result.scenicScore;
    }
  }
}

console.log("There are", total, "visible trees");
console.log("The highest scenic score is", maxScenicScore);
