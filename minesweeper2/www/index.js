import { MineSweeper } from "../pkg/minesweeper2";
import { memory } from "wasm-game-of-life/minesweeper2_bg";
console.log("okay");

const CELL_SIZE = 20; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

let playing = false;
let game = "";
let w = 10;
let h = 10;

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

const resetBoard = () => {
  let grid = document.getElementById("game-grid");
  grid.innerHTML = "";
  playing = false;
  let playResetButton = document.getElementById("play-reset");
  playResetButton.innerText = "Play Game";
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

  let playResetButton = document.getElementById("play-reset");
  playResetButton.innerText = "Reset";
  playing = true;

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
    resetBoard();
  } else {
    setupBoard();
  }
});

function clickBox(col, row) {
  game.click(col, row);
  const cellsPtr = game.state();
  const cells = new Uint8Array(memory.buffer, cellsPtr, w * h);
  debugger;

  let game_over = false;
  for (let i = 0; i < cells.length; i++) {
    if (cells[i] == 13) {
      game_over = true;
      break;
    }
  }
  if (game_over) {
    resetBoard();
    return;
  }
  createGrid();

  console.log(`Click -> cells: ${cells}`);
}

function createGrid() {
  const gameGrid = document.getElementById("game-grid");
  gameGrid.innerHTML = "";
  const cells = new Uint8Array(memory.buffer, game.state(), w * h);
  let idx = 0;
  for (var y = 0; y < h; y++) {
    let row = document.createElement("tr");
    for (var x = 0; x < w; x++) {
      let gamestate = cells[idx];
      let box = document.createElement("td");
      box.classList.add("grid-square");
      box.classList.add(state_types[gamestate]);
      box.id = `${x}${y}`;
      box.style.cssText += `width:${100 / h}%;`;
      box.style.cssText += `height:${100 / h}%;`;
      box.addEventListener("click", (e) => {
        console.log(`Box ${box.id} was clicked`);
        clickBox(x, y);
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
