<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
interface PicViewProps {
    img_path: String, 
    d_page: (x: number)=>void,
}
const { img_path, d_page } = defineProps<PicViewProps>();
console.log({img_path: img_path})
const image_base64 = ref("");
async function load_image(path: String) {
    console.log(path)
    image_base64.value = await invoke("load_image", { path });
}
if (img_path) load_image(img_path);

function onpicclick(event: MouseEvent) {
    console.log("click pic")
    const target = event.target as Element
    const rect = target.getBoundingClientRect();
    const x = event.clientX - rect.left;
    if (x < rect.width / 2) {
        d_page(-1);
    } else {
        d_page(1);
    }
}
</script>

<template>
    <div>
        <img @click="onpicclick" :src="'data:image/png;base64,' + image_base64">
    </div>
</template>

<style scoped>
img {
    width: 100%;
    height: 100%;
}
</style>