const fs = require('fs');

const input = fs.readFileSync('./input.txt', 'utf-8').split("\n\n");

let topOne = 0;
let topTwo = 0;
let topThree = 0;

for (let elf of input) {
  const lines = elf.trim().split('\n');

  let sum = 0;
  for (let line of lines) {
    sum += parseInt(line);
  }

  if (topOne < sum) {
    topThree = topTwo;
    topTwo = topOne;
    topOne = sum;
  } else if (topTwo < sum) {
    topThree = topTwo;
    topTwo = sum;
  } else if (topThree < sum) {
    topThree = sum;
  }
}

console.log("Most calories:", topOne);
console.log("Top 3 sum:", topOne + topTwo + topThree);
