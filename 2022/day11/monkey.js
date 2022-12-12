function getNumber(text) {
  return parseInt(text.replace(/\D+/g, ""));
}

module.exports = class Monkey {
  monkeyManager;
  inspectedItems;
  id;
  items;
  operation;
  divisibleTest;
  trueMonkey;
  falseMonkey;

  constructor(lines, monkeyManager) {
    this.monkeyManager = monkeyManager;

    this.inspectedItems = 0;

    this.id = getNumber(lines.shift());

    this.items = lines.shift().replaceAll(",", "")
      .split(" ").slice(4).map(n => parseInt(n));

    this.operation = lines.shift().split("=")[1]
      .trim().replaceAll("old", "item");

    this.divisibleTest = getNumber(lines.shift());
    this.trueMonkey = getNumber(lines.shift());
    this.falseMonkey = getNumber(lines.shift());

    return this;
  }

  addItem(item) {
    this.items.push(item);
  }

  inspectItems(isPartOne) {
    let copy = this.items.slice();
    this.items = [];

    for (let item of copy) {
      this.inspectedItems++;

      let worry = eval(this.operation);
      if (isPartOne) {
        worry /= 3;
      } else {
        worry %= this.monkeyManager.productOfDivideBy;
      }

      worry = Math.floor(worry);
      let monkeyId = 0;

      if (worry % this.divisibleTest == 0) {
        monkeyId = this.trueMonkey;
      } else {
        monkeyId = this.falseMonkey;
      }

      this.monkeyManager.addItemToMonkey(worry, monkeyId);
    }
  }
}
