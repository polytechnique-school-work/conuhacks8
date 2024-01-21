<template>
  <div class="page-container">
    <div class="items-container">
      <div class="image"><AutoadminLogo /></div>
    </div>
    <div class="row-container">
      <UploadButton />
      <NuxtLink to="/dashboard/300"
        ><Button title="Dashboard"><SettingsIcon /></Button
      ></NuxtLink>
    </div>
  </div>
</template>
<style scoped lang="scss">
.row-container {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
  gap: 2em;
  margin-bottom: 4em;
}
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
