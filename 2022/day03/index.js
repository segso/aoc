const fs = require('fs');

const lines = fs.readFileSync("./input.txt", "utf-8").trim().split("\n");

let alphabet = "abcdefghijklmnopqrstuvwxyz"
alphabet += alphabet.toUpperCase()

let priorities = {};

for (let i = 0; i < alphabet.length; i++) {
  priorities[alphabet.at(i)] = i + 1;
}

function getRepeatedChar(text, ...comparison) {
  charsloop:
  for (let character of text) {
    for (let string of comparison) {
      if (string.indexOf(character) == -1) {
        continue charsloop;
      }
    }
    return character;
  }
}

let total = 0;

for (let line of lines) {
  let firstHalf = line.substring(0, line.length / 2);
  let secondHalf = line.replace(firstHalf, "");

  let priority = priorities[getRepeatedChar(firstHalf, secondHalf)];
  total += priority;
}

console.log("Priorities total with one rucksack:", total);

total = 0;
for (let i = 0; i < lines.length; i+=3) {
  let letter = getRepeatedChar(lines[i], lines[i + 1], lines[i + 2]);
  total += priorities[letter];
}

console.log("Priorities total of three rucksacks:", total)
