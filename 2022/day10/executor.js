let currentCycle = 1;
let cycles = [];
let x = 1;

function addCycle(cycle, value) {
  cycles.push({cycle: cycle, value: value});
  currentCycle++;
}

function executeCommands(commands) {
  currentCycle = 1;
  cycles = [];
  x = 1;

  for (let line of commands) {
    let [command, number] = line.split(" ");
    number = parseInt(number)

    if (command == "noop") {
      addCycle(currentCycle, x);
      continue;
    }

    //addx command
    addCycle(currentCycle, x);
    addCycle(currentCycle, x);
    x += number;
  }

  addCycle(currentCycle, x);
  return cycles;
}

module.exports = executeCommands;
