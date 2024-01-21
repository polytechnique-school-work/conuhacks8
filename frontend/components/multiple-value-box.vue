<script setup lang="ts">
const props = defineProps<{
  title: string;
  bigValue: string;
  value: string;
  variation: "increase" | "decrease";
}>();
</script>

<template>
  <div class="wrapper">
    <Box :title="title"> </Box>
  </div>
</template>

<style scoped lang="scss">
.wrapper {
  width: 350px;
  height: 275px;
  margin: 20px;
  gap: 0.5rem;
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
}
</style>
