const Grid = require("./grid");
const fs = require("fs");
const input = fs.readFileSync("./input.txt", "utf-8").trim().split("\n");

function calculateVisitedPositions(knotCount) {
  let grid = new Grid(knotCount - 2);

  for (let line of input) {
    let split = line.split(" ");

    for (let i = split[1]; i > 0; i--) {
      grid.move(split[0]);
    }
  }

  return grid.visitedPositions.size;
}

console.log(
  "The total of visited positions is:",
  "\n  with 2 knots: ",
  calculateVisitedPositions(2),
  "\n  with 10 knots:",
  calculateVisitedPositions(10)
);
