<script lang="ts">
  import Cell from "./atoms/Cell.svelte";
  import VSpace from "./atoms/VSpace.svelte";
  import HSpace from "./atoms/HSpace.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { createStore } from "@tauri-apps/plugin-store";
  import { gameLevel, unlockedLevel } from "../stores";
  import confetti from "canvas-confetti";
  import { appDataDir as getAppDatDir } from "@tauri-apps/api/path";

  type CellType = { Fixed: number } | { Free: number | null };

  interface Sudoku {
    values: CellType[][];
    status: boolean;
  }

  let sudoku: Sudoku;
  let appDataDir: string;

  async function saveToStore(level: number) {
    let store = await createStore("store.bin");
    await store.set("unlocked", { value: level });
    store.save();
  }

  async function get_app_data_dir() {
    appDataDir = await getAppDatDir();
  }

  async function evaluate() {
    invoke<Sudoku>("evaluate", {
      sudoku,
      index: $gameLevel,
      appDataDir,
    })
      .then((s) => {
        sudoku = s;

        if (sudoku.status) {
          confetti({
            particleCount: 100,
            spread: 70,
            origin: { y: 0.6 },
          });
          if ($gameLevel == $unlockedLevel) {
            saveToStore($unlockedLevel + 1);
          }
        }
      })
      .catch();
  }

  async function newGame() {
    invoke<Sudoku>("new_game", { index: $gameLevel, appDataDir })
      .then((s) => (sudoku = s))
      .catch();
  }

  async function initialize() {
    await get_app_data_dir();
    await newGame();
  }

  async function clear() {
    invoke<Sudoku>("clear", { index: $gameLevel, appDataDir })
      .then((s) => (sudoku = s))
      .catch();
  }

  initialize();
</script>

<div class="game">
  {#if sudoku != null}
    {#each sudoku.values as rows, i}
      {#each rows as cell, j}
        {#if "Free" in cell}
          <input
            type="number"
            bind:value={cell.Free}
            min="1"
            max="9"
            on:input={evaluate}
          />
        {:else if "Fixed" in cell}
          <Cell value={cell.Fixed} />
        {/if}
        {#if j == 2 || j == 5}
          <VSpace />
        {/if}
        {#if j == 8 && (i == 2 || i == 5)}
          <HSpace />
        {/if}
      {/each}
    {/each}
  {:else}
    <p>Loading....</p>
  {/if}
</div>
<button class="btn" on:click={clear}>Clear</button>

<style scoped>
  .game {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-around;
    flex-flow: wrap;
    gap: 1px;
    width: 372px;
  }

  input {
    width: 40px;
    height: 40px;
    border: none;
    outline: none;
    padding: 0;
    margin: 0;
    border-radius: 0;
    background-color: #d79ebd;
    text-align: center;
    color: #0b1329;
    font-family: serif;
    cursor: default;
  }

  input:hover {
    background-color: #e7aecd;
  }

  input:focus {
    background-color: #f7bedd;
  }

  input[type="number"]::-webkit-outer-spin-button,
  input[type="number"]::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }

  .btn {
    position: fixed;
    bottom: 20px;
    right: 20px;
    background: #0005;
    color: #d78137;
    border: none;
    border-radius: 4px;
    width: 60px;
    height: 30px;
    box-shadow: 2px 2px 10px #000;
    &:hover {
      background-color: #0003;
    }

    &:active {
      background-color: #0002;
    }
  }
</style>
