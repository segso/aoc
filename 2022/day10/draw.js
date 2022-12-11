function getPixel(cycleValue, pixelIndex) {
  let spritePixels = [cycleValue-1, cycleValue, cycleValue+1];

  return spritePixels.indexOf(pixelIndex) == -1
    ? "#"
    : ".";
}

module.exports = function getDraw(linesAmount, cycles) {
  const lineLength = (cycles.length - 1) / linesAmount;

  let lines = [];
  let offset = 0;
  let currentLine = "";

  for (let _ = 0; _ < linesAmount; _++) {
    for (let i = offset; i < lineLength + offset; i++) {
      currentLine += getPixel(cycles[i].value + offset, i);
    }

    lines.push(currentLine);
    currentLine = "";
    offset += lineLength;
  }

  return lines;
}
