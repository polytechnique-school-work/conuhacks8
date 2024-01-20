<template>
  <label for="file-upload" class="upload-button">
    Upload your CSV File
  </label>
  <input 
    id="file-upload"
    type="file" 
    accept=".csv"
    @input="uploadFile()"
    ref="filePath"/>
</template>
<style scoped lang="scss">
@import "../assets/style/constants.scss";
.upload-button {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: center;
  height: 60px;
  width: 350px;
  color: map-get($map: $themes, $key: text);
  background-color: map-get($map: $themes, $key: primary);
}

</style>

<script lang="ts">
export default {
  methods: {
    async uploadFile(url: URL) {
      const file = this.$refs.filePath.files[0];
      if (!file) {
        console.log("No file selected");
        return;
      }
      
      const reader = new FileReader();
      reader.readAsText(file);
      reader.onload = async function(e) {
        if (!e.target){
          return;
        }
        const content = e.target.result;
        if (url) { 
          fetch(url, {
            method: "POST",
            body: await JSON.stringify({ content })
          })
        }
        window.location.href = "/dashboard";
      };
    }
  }
}
</script>