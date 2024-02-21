<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { ref } from 'vue';
import PicView from './components/PicView.vue';
import { invoke } from '@tauri-apps/api/tauri';
let page = ref(0);
let trans = ref(0);
const ss: string[][] = [[],[]]
const images = ref(ss)
async function load_dir_by_file(file_path: string) {
  const files: Array<string> = await invoke("load_dir_by_file", { filePath: file_path });
  let translate_dir:string = "";
  const image_files = files.filter((filename: string) => {
    if (filename.includes("trans")) {
      translate_dir = filename;
    }
    return filename.endsWith('.png') || filename.endsWith('.jpg')
  })
  image_files.sort((a, b) => a.localeCompare(b, undefined, { numeric: true, sensitivity: 'base' }))
  images.value[0] = image_files;
  if (translate_dir!=="") {
    const files: Array<string> = await invoke("load_dir_by_file", { filePath: translate_dir+'/any.jpg' });
    const image_files = files.filter((filename: string) => {
      return filename.endsWith('.png') || filename.endsWith('.jpg')
    });
    image_files.sort((a, b) => a.localeCompare(b, undefined, { numeric: true, sensitivity: 'base' }))
    images.value[1] = image_files;
    console.log(images.value[1])
  }
}
load_dir_by_file("/Users/mole/Downloads/ALEX STORY/1.png");

function d_page(d: number){
  page.value = Math.max(0, Math.min(page.value+d, images.value[0].length-1));
  console.log({page: page.value})
}
window.onkeydown = (e:KeyboardEvent) => {
  if (e.key === 'k') {
    trans.value = 1 - trans.value;
    console.log({trans: trans.value})
  }
}
</script>

<template>
  <div class="container">
    <div style="height: 100%;width: 100%;">
      <PicView v-if="images.length" :key="trans+'-'+page" :img_path="images[trans][page]" :d_page="d_page"  />
    </div>
  </div>
</template>

<style scoped></style>
