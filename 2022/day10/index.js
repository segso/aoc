const executeCommands = require("./executor");
const getSignalStrength = require("./signal");
const getDraw = require("./draw");

const fs = require("fs");
const input = fs.readFileSync("./input.txt", "utf-8").trim().split("\n");

const cycles = executeCommands(input);
const indexes = [20, 60, 100, 140, 180, 220];

console.log(
  "The total signal strength is:",
  getSignalStrength(indexes, cycles)
);

console.log(
  "\nThe image is:\n\n" +
  getDraw(6, cycles).join("\n")
);
