<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { ref } from "vue";
import { invoke } from "@tauri-apps/api";

// forked from https://www.zhihu.com/question/26744174/answer/2468892079
let colorList = ['#e6194B', '#3cb44b', '#ffe119', '#4363d8', '#f58231', '#42d4f4', '#f032e6', '#fabed4', '#469990', '#dcbeff', '#9A6324', '#fffac8', '#800000', '#aaffc3', '#000075', '#a9a9a9', '#ffffff', '#000000']

type file = {
  [key: string]: {
    name: string,
    lastModified: string,
    md5: string
    color: string
  }
}
const file_list = ref<file>();
file_list.value = {};
const myCanvas = ref(null);
function sendImage(file: File) {
  const reader = new FileReader();
  reader.onload = async function (e) {
    // @ts-ignore
    const base64String = reader.result.split(',')[1];
    try {
      let a: string = await invoke('detect_obj', { file: base64String });
      const img = new Image();
      img.onload = function () {
        drawCanvas(img, JSON.parse(a))
      }
      //@ts-ignore
      img.src = e.target.result;
      console.log('Image sent successfully', a);
    } catch (error) {
      alert(error)
      console.error('Failed to send image:', error);
    }
  };
  reader.readAsDataURL(file);
}
function dragenterEvent(event: Event) {
  event.stopPropagation();
  event.preventDefault();
}

async function dragoverEvent(event: Event) {
  event.stopPropagation();
  event.preventDefault();
}

function dragleaveEvent(event: Event) {
  event.stopPropagation();
  event.preventDefault();
}

function dropEvent(event: DragEvent) {
  event.stopPropagation();
  event.preventDefault();
  const files = event.dataTransfer!.files;
  displayChsFile(files);
}

function displayChsFile(files: FileList) {
  for (const file of files) {
    sendImage(file)
  }
}
// @ts-ignore
const drawCanvas = (img, boxes) => {
  const canvas = myCanvas.value
  // @ts-ignore
  const ctx = canvas.getContext('2d');
  // @ts-ignore
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  ctx.drawImage(img, 0, 0);
  ctx.strokeStyle = "#00FF00";
  ctx.lineWidth = 3;
  ctx.font = "18px serif";
  let count = 0
  // @ts-ignore
  boxes.forEach(([x1, y1, x2, y2, label,pos]) => {
    ctx.strokeStyle = colorList[count];
    ctx.strokeRect(Number(x1), Number(y1), Number(x2) - Number(x1), Number(y2) - Number(y1));
    ctx.fillStyle = colorList[count];
    
    const width = ctx.measureText(label).width;
    ctx.fillRect(Number(x1), Number(y1), width + 10, 25);
    ctx.fillStyle = "#000000"
    ctx.fillText(label, Number(x1), Number(y1) + 18);
    const width2 = ctx.measureText(label).width;
    ctx.fillStyle = "#000000"
    ctx.fillText("  "+pos, Number(x1)+width2, Number(y1) + 18);
    count ++
  });
};
</script>

<template>
  <div class="file-drop" @dragenter="dragenterEvent" @dragover="dragoverEvent" @dragleave="dragleaveEvent"
    @drop="dropEvent">
    <!-- 头部 -->
    <p style="font-size: 24px"> Monster v0.0.1 </p>
    <!-- 内容区 -->
    <div class="middle-con">
      <canvas ref="myCanvas" width="500px" height="500px"></canvas>
    </div>
  </div>
</template>

<style scoped>
@import url('./assets/css/reset.css');
@import url('./assets/css/app.css');
</style>