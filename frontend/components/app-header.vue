<template>
  <div class="header">
    <NuxtLink to="/">
      <div class="image"><img src="/img/AutoAdmin.png" alt="" />AutoAdmin</div>
    </NuxtLink>
  </div>
</template>
<style scoped lang="scss">
@import "../assets/style/constants.scss";
.header {
  background-color: map-get($map: $themes, $key: background-dark);
  height: 5rem;
  display: flex;
  flex-direction: row;
  justify-content: space-between;
}

.image {
  height: 100%;
  img {
    height: 75%;
    width: auto;
  }
  padding: 0.5rem;
  box-sizing: border-box;
  display: flex;
  flex-direction: row;
  align-items: center;
  text-align: center;
  gap: 1rem;
  color: white;
  font-size: 1.5rem;
}
</style>
