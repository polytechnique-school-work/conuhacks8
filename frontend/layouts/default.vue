<template>
  <AppHeader />
  <div class="container">
    <!-- <Nuxt /> -->
    <div class="wrapper">
      <slot />
    </div>
  </div>
  <AppFooter />
</template>

<style scoped lang="scss">
.container {
  display: flex;
  flex-direction: column;
  flex-grow: 1;
}

.wrapper {
  flex: 1;
}

html,
body,
#__nuxt,
#__layout {
  height: 100%;
  margin: 0;
  padding: 0;
}
</style>
