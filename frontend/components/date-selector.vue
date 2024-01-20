<script lang="js">
export default {
  methods: {
    onClick() {
      console.log(this.$refs.date.value);
    },
  },
};
</script>

<template>
  <Box class="dateBox" title="Date">
    <div class="info">
      <input ref="date" type="date" id="start" name="trip-start" value="2024-01-20" @input="onClick" />
    </div>
  </Box>
</template>

<style lang="scss" scoped>
.dateBox {
  width: 350px;
  height: 60px;
  margin: 20px;
  gap: 0.5rem;

  .info {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 0.5rem;
  }

  display: flex;
  flex-direction: column;
  justify-content: center;
}
</style>
