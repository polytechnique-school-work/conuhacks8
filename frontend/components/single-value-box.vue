<script setup lang="ts">
const props = defineProps<{
  bigValue: string;
  value: string;
  variation: "increase" | "decrease";
}>();
</script>

<template>
  <div class="wrapper">
    <Box title="Revenue">
      <div class="values">
        <div class="bigvalue">{{ bigValue }}</div>
        <div class="value">{{ value }}</div>
      </div>
    </Box>
  </div>
</template>

<style scoped lang="scss">
.wrapper {
  width: 350px;
  height: 60px;
  margin: 20px;
  gap: 0.5rem;
}
</style>
