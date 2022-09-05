import { MineSweeper } from "../pkg/minesweeper2";
import { memory } from "wasm-game-of-life/minesweeper2_bg";
console.log("okay");

const CELL_SIZE = 20; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

let game = "";

const state_types = {
  0: "empty",
  1: "numbered",
  2: "numbered",
  3: "numbered",
  4: "numbered",
  5: "numbered",
  6: "numbered",
  7: "numbered",
  8: "numbered",
  9: "CoveredNoMine",
  10: "CoveredMine",
  11: "FlaggedNoMine",
  12: "FlaggedMine",
  13: "Exploded",
};

// Construct the game, and get its width and height.

let playing = false;
let w = 10;
let h = 10;

const resetBoard = () => {
  let grid = document.getElementById("game-grid");
  grid.innerHTML = "";
};

const setupBoard = () => {
  const width = Number(document.getElementById("size-input").value);
  const height = Number(document.getElementById("size-input").value);
  const mines = Number(document.getElementById("mines-input").value);

  if (width > 100 || height > 100) {
    alert("bad game dimension");
    return;
  }

  if (mines >= width * height) {
    alert("too many mines");
    return;
  }

  game = MineSweeper.new(width, height, mines);
  console.log(`Starting New Game. ${width}x${height} with ${mines} mines.`);

  h = height;
  w = width;

  const cellsPtr = game.state();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);
  console.log(`cells: ${cells}`);

  createGrid(width, height);
};

const playResetButton = document.getElementById("play-reset");
playResetButton.addEventListener("click", (event) => {
  console.log("here");
  if (playing) {
    playing = false;
    playResetButton.innerText = "Play Game";
    resetBoard();
  } else {
    playing = true;
    playResetButton.innerText = "Reset";
    setupBoard();
  }
});

function clickBox(col, row) {
  game.click(col, row);
  const cellsPtr = game.state();
  const cells = new Uint8Array(memory.buffer, cellsPtr, w * h);
  console.log(`Click -> cells: ${cells}`);
}

function createGrid(width, height) {
  const gameGrid = document.getElementById("game-grid");
  const cells = new Uint8Array(memory.buffer, game.game_state, w * h);
  let idx = 0;
  for (var y = 0; y < h; y++) {
    let row = document.createElement("tr");
    for (var x = 0; x < w; x++) {
      let gamestate = cells[idx];
      let box = document.createElement("td");
      box.classList.add("grid-square");
      box.classList.add(state_types[gamestate]);
      box.id = `${x}${y}`;
      box.style.cssText += `width:${100 / w}%;`;
      box.style.cssText += `height:${100 / h}%;`;
      box.addEventListener("click", (e) => {
        console.log(`Box ${box.id} was clicked`);
        let row = box.id[0];
        let col = box.id[1];
        clickBox(col, row);
      });

      // add appropriate text
      if (gamestate == 0 || gamestate == 9 || gamestate == 10) {
        // empty cell or covered, nothing to add
      } else if (gamestate > 0 && gamestate < 9) {
        box.innerText = `${gamestate}`;
      } else if (gamestate == 11 || gamestate == 12) {
        box.innerText = "🚩";
      } else if (gamestate == 13) {
        box.innerText = "💣";
      }
      row.appendChild(box);
      idx += 1;
    }
    gameGrid.appendChild(row);
  }
}
