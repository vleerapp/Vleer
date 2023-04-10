<template>
  <input type="text" class="search" placeholder="Search" @input="handleInputChange" />
</template>

<script>
export default {
  methods: {
    handleInputChange(event) {
      const inputValue = event.target.value;
      if (inputValue) {
        this.$router.push(`/search?q=${inputValue}`);
      }
    }
  }
};
</script>

<style>
.search {
  width: 325px;
  height: 40px;
  background-color: #1e1d23;
  border: 1px solid #26252b;
  border-radius: 10px;
  font-family: "Inter", sans-serif;
  font-size: 16px;
  display: flex;
  align-items: center;
  padding-inline: 10px;
  color: var(--tx);
  outline: none;
  margin-bottom: 28px;
}
</style>