<script lang="ts">
  import { unlockedLevel } from "../stores";
  import Button from "../components/atoms/Button.svelte";
  import { createStore, type Store } from "@tauri-apps/plugin-store";

  let store: Store;

  async function loadStore() {
    store = await createStore("store.bin");
    let val = await store.get<{ value: number }>("unlocked");
    if (val) {
      $unlockedLevel = val.value;
    }
  }

  loadStore();
</script>

<div class="main">
  <div class="buttons">
    {#each [1, 2, 3, 4, 5, 6, 7, 8] as i}
      {#if i <= $unlockedLevel}
        <Button index={i} lock={false} />
      {:else}
        <Button index={i} />
      {/if}
    {/each}
  </div>
</div>

<style scoped>
  .main {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    box-sizing: border-box;
  }

  .buttons {
    padding: 10px;
    box-sizing: border-box;
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    flex-wrap: wrap;
    gap: 20px;
  }
</style>
