const Monkey = require("./monkey");

module.exports = class MonkeyManager {
  monkeys;
  productOfDivideBy;
  isPartOne;

  constructor() {
    this.monkeys = [];
    this.productOfDivideBy = 1;
    this.isPartOne = true;
  }

  addMonkey(lines) {
    lines = lines.split("\n");
    let monkey = new Monkey(lines, this);
    this.monkeys.push(monkey);
    this.productOfDivideBy *= monkey.divisibleTest;
    return monkey;
  }

  doRound() {
    for (let monkey of this.monkeys) {
      monkey.inspectItems(this.isPartOne);
    }
  }

  addItemToMonkey(item, monkeyId) {
    this.monkeys[monkeyId].addItem(item);
  }
}
