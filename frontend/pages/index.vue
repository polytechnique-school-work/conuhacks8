<template>
  <div class="page-container">
    <div class="items-container">
      <div class="image"><AutoadminLogo /></div>
      <UploadButton />
    </div>
  </div>
</template>
<style scoped lang="scss">
.page-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
}
.items-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  padding: 20px;
}

.image {
  width: 100%;
  max-width: 400px;
  height: auto;
  margin-bottom: 20px;
}
</style>
