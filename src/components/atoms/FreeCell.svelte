<script lang="ts">
  export let id = [0, 0];
  export let OnInput = () => {};
</script>

<input
  type="number"
  on:input={OnInput}
  id={`${id[0]}${id[1]}`}
  min="1"
  max="9"
/>

<style scoped>
  input {
    width: 50px;
    height: 50px;
    border: none;
    outline: none;
    padding: 0;
    margin: 0;
    border-radius: 0;
    background-color: #d79ebd;
    text-align: center;
    color: #0b1329;
    font-family: serif;
  }

  input:focus {
    background-color: #f7bedd;
  }

  input[type="number"]::-webkit-outer-spin-button,
  input[type="number"]::-webkit-inner-spin-button {
    -webkit-appearance: none;
    margin: 0;
  }
  /* input[type="number"] {
		-moz-appearance: textfield;
	} */
</style>
