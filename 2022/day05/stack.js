module.exports = class Stack {
  items = [];

  push(item) {
    if (item == " ") {
      return;
    }
    this.items.push(item);
  }

  pushArray(array) {
    for (let el of array) {
      this.push(el);
    }
  }

  popLast() {
    return this.items.pop();
  }

  pop(times) {
    let output = [];

    for (let i = 0; i < times; i++) {
      output.push(this.popLast());
    }

    return output;
  }

  setItems(items) {
    this.items = items;
  }

  clone() {
    let copy = new Stack();
    copy.setItems(this.items.slice());
    return copy;
  }
}
