const fs = require("fs");
const {start} = require("repl");
const paths = fs.readFileSync("./input.txt", "utf-8").trim().split("\n");

const map = new Map();
const X = 0, Y = 1;

function getBetweenPointsAsArray(startPoint, endPoint) {
  startPoint = startPoint.split(",").map(e => parseInt(e));
  endPoint = endPoint.split(",").map(e => parseInt(e));

  let different = X;
  let noDifferent = Y;
  let points = [{}];
  points[0][X] = startPoint[X];
  points[0][Y] = startPoint[Y];

  if (startPoint[X] == endPoint[X]) {
    different = Y;
    noDifferent = X;
  }
  let difference = endPoint[different] - startPoint[different];
  let sum = difference == 0 ? 0 : (difference < 0 ? -1 : 1);

  if (sum == 0) {
    return points;
  }

  let limit = Math.abs(difference);

  for (let i = 1; i <= limit; i++) {
    let newPoint = {};
    newPoint[different] = startPoint[different] + i * sum;
    newPoint[noDifferent] = startPoint[noDifferent];
    points.push(newPoint);
  }

  return points;
}

let highestY = 0;

for (let path of paths) {
  let points = path.split(" -> ");

  for (let i = 1; i < points.length; i++) {
    for (let point of getBetweenPointsAsArray(points[i-1], points[i])) {
      map.set(point[X] + "-" + point[Y], "ROCK");
      if (highestY < point[Y]) {
        highestY = point[Y];
      }
    }
  }
}

function getFromMap(x, y, isFloorEnabled) {
  if (isFloorEnabled && y == highestY + 2) {
    return "ROCK";
  }
  return map.get(x + "-" + y) ?? "AIR";
}

let sourcePoint = {};
sourcePoint[X] = 500;
sourcePoint[Y] = 0;

function dropSand(isFloorEnabled) {
  sandPoint = {};
  sandPoint[X] = sourcePoint[X];
  sandPoint[Y] = sourcePoint[Y];
  let isMoving = true;

  while (isMoving) {
    if (!isFloorEnabled && sandPoint[Y] == highestY) {
      return true;
    }

    if (getFromMap(sandPoint[X], sandPoint[Y]+1, isFloorEnabled) == "AIR") {
      sandPoint[Y]++;
      continue;
    }

    if (getFromMap(sandPoint[X]-1, sandPoint[Y]+1, isFloorEnabled) == "AIR") {
      sandPoint[X]--;
      sandPoint[Y]++;
      continue;
    }

    if (getFromMap(sandPoint[X]+1, sandPoint[Y]+1, isFloorEnabled) == "AIR") {
      sandPoint[X]++;
      sandPoint[Y]++;
      continue;
    }

    map.set(sandPoint[X] + "-" + sandPoint[Y], "SAND");
    if (sandPoint[X] == sourcePoint[X] &&
      sandPoint[Y] == sourcePoint[Y]) {
      return true;
    }
    isMoving = false;
  }
}

for (let i = 0 ;; i++) {
  if (dropSand(false)) {
    console.log("Units of sand until falling to void:", i);
    break;
  }
}

map.forEach((v, k) => {
  if (map.get(k) == "SAND") {
    map.delete(k);
  }
});

for (let i = 1 ;; i++) {
  if (dropSand(true)) {
    console.log("Units of sand until blocking source:", i);
    break;
  }
}
