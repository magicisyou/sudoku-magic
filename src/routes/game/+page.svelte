<script lang="ts">
  import Sudoku from "../../components/Sudoku.svelte";
</script>

<div>
  <Sudoku />
  <a href="/">
    <img src="/home.svg" alt="home" />
  </a>
</div>

<style scoped>
  div {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  img {
    position: fixed;
    top: 30px;
    left: 50px;
    width: 50px;
    height: 50px;
    padding: 10px;
    border-radius: 50%;
    box-shadow: 0 0 16px #d78137;

    &:hover {
      box-shadow: 0 0 20px #d78137;
    }
  }
</style>
