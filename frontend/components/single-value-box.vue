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
    <Box :title="title">
      <div class="values">
        <div class="bigvalue">{{ bigValue }}</div>
        <div class="variation" :style="'color: ' + (variation == 'increase' ? '#59985E' : '#F44C3F')">
          <div v-if="variation == 'increase'"><ExpandMore /></div>
          <div v-else><ExpandLess /></div>
          <div class="value">
            {{ value }}
          </div>
        </div>
      </div>
    </Box>
  </div>
</template>

<style scoped lang="scss">
.bigvalue {
  font-size: 3rem;
}

.values {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 100%;
}
.wrapper {
  width: 350px;
  height: 275px;
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
}

.variation {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
}
</style>
