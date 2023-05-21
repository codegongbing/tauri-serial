<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
// import Greet from '@/components/Greet.vue';
async function alert_test() {
  await invoke('alert_test')
}

onMounted(() => {
})

</script>

<template>
  <div class="h-screen w-full">
    
  </div>
</template>

<style lang="scss" scoped></style>
