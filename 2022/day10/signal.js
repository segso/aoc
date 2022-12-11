function getSignalStrength(indexesArray, cycles) {
  let total = 0;

  for (let index of indexesArray) {
    if (!cycles[index - 1]) {
      continue;
    }

    total += index * cycles[index - 1].value;
  }

  return total;
}

module.exports = getSignalStrength;
