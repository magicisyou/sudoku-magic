<script lang="ts">
  import { gameLevel } from "../../stores";

  export let index = 0;
  export let lock = true;

  async function setCurrentGame() {
    $gameLevel = index;
  }
</script>

<a href={lock ? "/" : "/game"} on:click={setCurrentGame}>
  {index}
  {#if lock == true}
    <img src="/lock.png" alt="lock" />
  {/if}
</a>

<style scoped>
  a {
    width: 120px;
    height: 120px;
    border-radius: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 64px;
    text-decoration: none;
    background: #0005;
    color: #d78137;
    box-shadow: 2px 2px 10px #000;
    position: relative;

    &:hover {
      background-color: #0003;
    }

    &:active {
      background-color: #0002;
    }
  }

  img {
    position: absolute;
    top: 10px;
    left: 10px;
    width: 20;
    height: 20px;
  }
</style>
