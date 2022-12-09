function getShapePoints(shape) {
  const dictionary = {
    "ROCK": 1,
    "PAPER": 2,
    "SCISSORS": 3
  }
  
  return dictionary[shape];
}

function letterToShape(shape) {
  const dictionary = {
    "A": "ROCK",
    "X": "ROCK",
    "B": "PAPER",
    "Y": "PAPER",
    "C": "SCISSORS",
    "Z": "SCISSORS"
  };

  return dictionary[shape];
}

function getWinPoints(status) {
  const dictionary = {
    "lose": 0,
    "draw": 3,
    "win": 6
  };

  return dictionary[status];
}

function getWinResult(shape, opponentShape) {
  const dictionary = {
    "ROCK": {
      "PAPER": "lose",
      "SCISSORS": "win"
    },
    "PAPER": {
      "ROCK": "win",
      "SCISSORS": "lose"
    },
    "SCISSORS": {
      "ROCK": "lose",
      "PAPER": "win",
    },
  }
  if (shape == opponentShape) {
    return "draw";
  }

  return dictionary[shape][opponentShape]
}

function resultToText(status) {
  const dictionary = {
    "X": "lose",
    "Y": "draw",
    "Z": "win",
  }

  return dictionary[status];
}

module.exports = {
  getShapePoints,
  letterToShape,
  getWinPoints,
  getWinResult,
  resultToText
};
