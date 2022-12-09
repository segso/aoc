const fs = require("fs");
const Stack = require("./stack");
const input = fs.readFileSync("./input.txt", "utf-8");

const sections = input.split("\n\n");
const stackLines = sections[0].split("\n");
const instructions = sections[1].split("\n");

const stackCount = parseInt(sections[0].slice(-2));

let stacks = [];

for (let i = 0; i < stackCount; i++) {
  stacks.push(new Stack());
}

for (let i = stackLines.length - 2; i >= 0; i--) {
  let lines = stackLines[i];

  for (let j = 0; j < stackCount; j++) {
    let crate = lines.at((j+1) * 4 - 3);

    if (crate) {
      stacks[j].push(crate);
    }
  }
}

let copy = [];
for (let stack of stacks) {
  copy.push(stack.clone());
}

function execInstructions(doReverse) {
  for (let instruction of instructions) {
    const numbers = instruction.split(/\D+/).slice(1);
  
    if (numbers.length != 0) {
      let crates = stacks[numbers[1] - 1].pop(numbers[0]).filter(f => f);

      if (doReverse) {
        crates = crates.reverse();
      }

      stacks[numbers[2] - 1].pushArray(crates);
    }
  }

  let output = "";

  for (let stack of stacks) {
    output += stack.popLast();
  }

  return output;
}

console.log("Stack tops with first crane:", execInstructions(false));
stacks = copy;
console.log("Stack tops with second crane:", execInstructions(true));
