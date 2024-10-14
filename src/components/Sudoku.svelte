<script lang="ts">
  import Cell from "./atoms/Cell.svelte";
  import VSpace from "./atoms/VSpace.svelte";
  import HSpace from "./atoms/HSpace.svelte";
  import FreeCell from "./atoms/FreeCell.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { createStore, type Store } from "@tauri-apps/plugin-store";
  import { gameLevel, unlockedLevel } from "../stores";
  import confetti from "canvas-confetti";

  $: game = { values: [[]] };

  let store: Store;

  async function loadStore() {
    store = await createStore("store.bin");
  }

  async function saveToStore(level: number) {
    await store.set("unlocked", { value: level });
    store.save();
  }

  async function check() {
    let result = await invoke("evaluate");
    if (result) {
      confetti({
        particleCount: 100,
        spread: 70,
        origin: { y: 0.6 },
      });
      if ($gameLevel == $unlockedLevel) {
        console.log("hj");
        saveToStore($unlockedLevel + 1);
      }
    }
  }

  async function inputSync(this: HTMLInputElement) {
    await invoke("input_sync", {
      index: [Number(this.id[0]), Number(this.id[1])],
      value: Number(this.value),
    });
    check();
  }

  async function newGame() {
    game = await invoke("new_game", { index: $gameLevel });
  }

  loadStore();
  newGame();
</script>

<div class="game">
  {#each game.values as rows, i}
    {#each rows as val, j}
      {#if val == 0}
        {#key game}
          <FreeCell id={[i, j]} OnInput={inputSync} />
        {/key}
      {:else}
        <Cell value={val} />
      {/if}
      {#if j == 2 || j == 5}
        <VSpace />
      {/if}
      {#if j == 8 && (i == 2 || i == 5)}
        <HSpace />
      {/if}
    {/each}
  {/each}
</div>

<style scoped>
  .game {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-around;
    flex-flow: wrap;
    gap: 1px;
    width: 462px;
  }
</style>
