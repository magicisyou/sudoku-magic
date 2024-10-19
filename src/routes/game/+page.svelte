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
    padding-top: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  img {
    position: fixed;
    bottom: 20px;
    right: 90px;
    height: 25;
    width: 25;
    padding: 2.5px;
    border-radius: 5px;
    box-shadow: 2px 2px 10px #000;
    cursor: default;

    &:hover {
      background-color: #0003;
    }
  }
</style>
