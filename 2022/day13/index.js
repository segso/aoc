const fs = require("fs");
let pairs = fs.readFileSync("./input.txt", "utf-8").trim().split("\n\n");

pairs = pairs.map(value => {
  let split = value.split("\n");
  return {
    firstValue: eval(split[0]),
    secondValue: eval(split[1])
  };
});

const RIGHT_ORDER = -1;
const WRONG_ORDER = 1;
const UNKNOWN = 0;

function compare(firstValue, secondValue) {
  if (typeof firstValue == "number" && typeof secondValue == "number") {
    if (firstValue < secondValue) {
      return RIGHT_ORDER;
    }

    if (firstValue == secondValue) {
      return UNKNOWN;
    }

    return WRONG_ORDER;
  }

  if (typeof firstValue == "object" && typeof secondValue == "object") {
    for (let i = 0; i < firstValue.length; i++) {
      let firstElement = firstValue[i];
      let secondElement = secondValue[i];
      if (secondElement == undefined) {
        return WRONG_ORDER;
      }

      let status = compare(firstElement, secondElement);

      if (status != UNKNOWN) {
        return status;
      }
    }

    if (secondValue.length > firstValue.length) {
      return RIGHT_ORDER;
    }

    return UNKNOWN;
  }

  if (typeof firstValue == "number") {
    return compare([firstValue], secondValue);
  }

  if (typeof secondValue == "number") {
    return compare(firstValue, [secondValue]);
  }
}

let total = 0;

for (let i = 0; i < pairs.length; i++) {
  let el = pairs[i];
  if (compare(el.firstValue, el.secondValue) == RIGHT_ORDER) {
    total += i + 1;
  }
}

console.log("The sum of the indexes in right order is:", total);

let sortedArray = [[[2]], [[6]]];
for (let el of pairs) {
  sortedArray.push(el.firstValue);
  sortedArray.push(el.secondValue);
}

sortedArray = sortedArray.sort((a,b) => compare(a, b));

// Stringify arrays for compare
sortedArray = sortedArray.map(el => JSON.stringify(el));

let startIndex = sortedArray.indexOf(JSON.stringify([[2]])) + 1;
let endIndex = sortedArray.indexOf(JSON.stringify([[6]])) + 1;

console.log("The product of packets indexes is:", startIndex * endIndex);
