<script lang="ts">
  // import { invoke } from "@tauri-apps/api/core";
</script>

<div class="main">
  <img src="/sudoku-text.png" alt="sudoku" />
  <slot></slot>
</div>

<style scoped>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
    cursor: default;
  }

  img {
    position: fixed;
    top: 26px;
    width: 175px;
    height: auto;
  }

  .main {
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    box-sizing: border-box;
    background: linear-gradient(to top, #2b1829, #a52150);
  }
</style>
