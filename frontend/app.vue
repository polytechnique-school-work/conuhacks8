<template>
  <NuxtLayout>
    <NuxtPage></NuxtPage>
  </NuxtLayout>
</template>

<style lang="scss">
@import "../assets/style/constants.scss";
body {
  margin: 0;
  padding: 0;
  min-width: 100vw;
  min-height: 100vh;
  font-family: "Noto Sans", sans-serif;
  font-optical-sizing: auto;
  font-style: normal;
  background-color: map-get($map: $themes, $key: background);
}

.hover {
  &:hover {
    cursor: pointer;
  }

  &:active {
    transform: scale(0.95);
  }
}
</style>
