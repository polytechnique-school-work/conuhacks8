<script lang="ts">
export default {
  methods: {
    sendDate(date: string) {
      defineEmits(["date"])("date", date);
    },
    onClick() {
      // @ts-ignore
      sendDate(this.$refs.date.value);
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

  .info {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 0.5rem;

    & input {
      // reset style
      border: none;
      outline: none;
      background: none;
      // custom style
      font-size: 1rem;
      font-family: inherit;
      // remove calendar
    }
  }

  display: flex;
  flex-direction: column;
  justify-content: center;
}
</style>
