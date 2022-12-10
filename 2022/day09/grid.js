const Position = require("./position");

module.exports = class Grid {
  visitedPositions;
  headPosition;
  extraKnots;
  tailPosition;

  constructor(extraKnots) {
    this.visitedPositions = new Set();
    this.visitedPositions.add("0_0");
    this.headPosition = new Position();
    this.tailPosition = new Position();

    this.extraKnots = [];
    for (let i = 0; i < extraKnots; i++) {
      this.extraKnots[i] = new Position();
    }
  }

  recalculate() {
    let knots = [...this.extraKnots, this.tailPosition];
    let lastKnot = this.headPosition;
    
    for (let i = 0; i < knots.length; i++) {
      let knot = knots[i];

      let difference = {
        x: lastKnot.x - knot.x,
        y: lastKnot.y - knot.y,
      };

      let updateNext = Math.abs(difference.x) == 2 ||
        Math.abs(difference.y) == 2;

      if (!updateNext) {
        break;
      }
      
      difference.x /= difference.x ? Math.abs(difference.x) : 1;
      difference.y /= difference.y ? Math.abs(difference.y) : 1;

      knot.x += difference.x;
      knot.y += difference.y;
        
      if (i == knots.length - 1) {
        this.visitedPositions.add(
          knot.x + "_" + knot.y
        );
      }

      lastKnot = knot;
    }
  }

  move(direction) {
    const directions = {
      "U": {x: 0, y: -1},
      "D": {x: 0, y: 1},
      "L": {x: -1, y: 0},
      "R": {x: 1, y: 0},
    };

    this.headPosition.x += directions[direction].x;
    this.headPosition.y += directions[direction].y;
    this.recalculate();
  }
}
