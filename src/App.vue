<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { once } from '@tauri-apps/api/event';

const getSerialProcess = async () => {
  await invoke('get_serial_process')
  await once('serial-port', (event: any) => {
    console.log(event);
  })
}
onMounted(() => {
})

</script>

<template>
  <div class="h-screen w-full">
    <div class="h-100 w-100">
      <button class="bg-gray-300" @click="getSerialProcess">点击获取serial</button>
    </div>
  </div>
</template>

<style lang="scss" scoped></style>
