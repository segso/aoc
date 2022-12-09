const fs = require("fs");

const input = fs.readFileSync("./input.txt", "utf-8").trim();

function hasRepeatedCharacters(text) {
  let storage = '';

  for (let character of text) {
    if (storage.indexOf(character) != -1) {
      return true;
    }

    storage += character;
  }

  return false;
}

function findMarkerIndex(markerLength) {
  let index = undefined;

  for (let i = 0; i <= input.length - markerLength; i++) {
    if (!hasRepeatedCharacters(input.substring(i, i + markerLength))) {
      index = i + markerLength;
      break;
    }
  }

  return index;
}

console.log("With a marker length of 4:", findMarkerIndex(4));
console.log("With a marker length of 14:",findMarkerIndex(14));
