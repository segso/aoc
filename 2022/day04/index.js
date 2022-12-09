const fs = require('fs');

const lines = fs.readFileSync("./input.txt", "utf-8").trim().split("\n");

function calculateRange(range) {
  let splitted = range.split("-");
  let start = parseInt(splitted[0]);
  let end = parseInt(splitted[1]);

  let output = [];

  for (let i = start; i <= end; i++) {
    output.push(i);
  }

  return output;
}

let total = 0

for (let line of lines) {
  const ranges = line.split(",");

  const firstRange = "-" + calculateRange(ranges[0]).join("-") + "-";
  const secondRange = "-" + calculateRange(ranges[1]).join("-") + "-";

  if (firstRange.indexOf(secondRange) != -1 ||
    secondRange.indexOf(firstRange) != -1
  ) {
    total++;
  }
}

console.log("Total of fully contains:", total);

total = 0;
for (let line of lines) {
  const ranges = line.split(",");

  const firstRange = calculateRange(ranges[0]);
  const secondRange = calculateRange(ranges[1]);

  for (let el of firstRange) {
    if (secondRange.indexOf(el) != -1) {
      total++;
      break;
    }
  }
}

console.log("Total of overlaps:", total);
