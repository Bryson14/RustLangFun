import { MineSweeper } from "../pkg/minesweeper2";
import { memory } from "wasm-game-of-life/minesweeper2_bg";

const CELL_SIZE = 20; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

// Construct the game, and get its width and height.

console.log("ok", cells);

let playing = false;

const resetBoard = () => {};

const setupBoard = () => {
  const width = Number(document.getElementById("size-input").innerText);
  const height = Number(document.getElementById("size-input").innerText);
  const mines = Number(document.getElementById("mines-input").innerText);
  const game = MineSweeper.new(width, height, bombs);

  const cellsPtr = game.game_state;
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  if (width > 100 || height > 100) {
    alert("bad game dimension");
    return;
  }

  if (mines > width * height) {
    alert("too many mines");
    return;
  }
};

const gameGrid = document.getElementById("game-grid");

const playResetButton = document.getElementById("play-reset");
playResetButton.addEventListener("click", (event) => {
  console.log("here");
  if (playing) {
    playing = false;
    resetBoard();
  } else {
    playing = true;
    setupBoard();
  }
});
