<script setup lang="ts">
const props = defineProps<{
  amount: number;
}>();
</script>

<template>
    <div class="input-card">
        <input type="number" class="text-container" :value="props.amount">
        </input>
    </div>
</template>

<style scoped lang="scss">
@import "../assets/style/constants.scss";
.input-card {
    width: 100%;
    max-width: 280px;
    height: 60px;
    background-color: map-get($map: $themes, $key: text-light);
    border-radius: 5px;
    color: map-get($map: $themes, $key: text);
    text-align: center;
    justify-content: center;
    display: flex;
}
.text-container {
    display: flex;
    align-items: center;
    justify-content: center;
    text-align: right;
    width: 100%;
}
input {
    background: none;
    border: none;
    outline: none;
    padding: 0;
    margin: 0;
    /* Add any other properties you wish to reset */
}


</style>