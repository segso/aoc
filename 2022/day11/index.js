const MonkeyManager = require("./monkey-manager");
const fs = require("fs");
const monkeyLines = fs.readFileSync("./input.txt", "utf-8").trim().split("\n\n");

const manager = new MonkeyManager();
const secondManager = new MonkeyManager();
secondManager.isPartOne = false;

for (let monkey of monkeyLines) {
  manager.addMonkey(monkey);
  secondManager.addMonkey(monkey);
}

function getMonkeyBusiness(monkeyManager, rounds) {
  for (let i = 0; i < rounds; i++) {
    monkeyManager.doRound();
  }

  let topOne = 0;
  let topTwo = 0;

  for (let monkey of monkeyManager.monkeys) {
    let inspectedItems = monkey.inspectedItems;
    if (topOne < inspectedItems) {
      topTwo = topOne;
      topOne = inspectedItems;
    } else if (topTwo < inspectedItems) {
      topTwo = inspectedItems;
    }
  }
  return topOne * topTwo;
}

console.log(
  "The business of the two most active monkeys is",
  "\n  For part one:",
  getMonkeyBusiness(manager, 20),
  "\n  For part two:",
  getMonkeyBusiness(secondManager, 10000)
);
