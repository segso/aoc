const fs = require('fs');
const {
  getShapePoints,
  letterToShape,
  getWinPoints,
  getWinResult,
  resultToText
} = require("./utils");

const input = fs.readFileSync("./input.txt", "utf-8").trim().split("\n");

let totalPoints = 0;

for (let line of input) {
  const split = line.split(" ");
  const opponentShape = letterToShape(split[0]);
  const myShape = letterToShape(split[1]);

  const status = getWinPoints(getWinResult(myShape, opponentShape));
  const roundPoints = getShapePoints(myShape) + status;

  totalPoints += roundPoints;
}

console.log("First strategy points:", totalPoints);

const shapes = ["ROCK", "PAPER", "SCISSORS"];
totalPoints = 0;

for (let line of input) {
  const split = line.split(" ");
  const status = resultToText(split[1]);
  const opponentShape = letterToShape(split[0]);

  let myShape = opponentShape;

  if (status != "draw") {
    for (let shape of shapes) {
      if (getWinResult(shape, opponentShape) == status) {
        myShape = shape;
        break;
      }
    }
  }

  const myWinPoints = getWinPoints(status);
  const roundPoints = myWinPoints + getShapePoints(myShape);

  totalPoints += roundPoints;
}

console.log("Second strategy points:", totalPoints);
