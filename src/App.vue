<script setup lang="ts">
import SerialSider from '@/components/SerialSider.vue';
import SerialOutPut from '@/components/SerialOutput.vue';
import SerialInput from '@/components/SerialInput.vue';
</script>

<template>
  <div class="flex flex-col h-screen w-screen overflow-hidden">
    <!--    <el-container class="container aspect-video flex flex-nowrap relative">-->
    <div class="p-5 h-screen flex flex-nowrap relative w-screen">
      <SerialSider></SerialSider>
      <div class="w-7"></div>
      <div class="flex flex-col flex-grow basis-3/4">
        <SerialOutPut></SerialOutPut>
        <div class="h-10">
        </div>
        <SerialInput></SerialInput>
      </div>
    </div>
    <!-- <div class="h-10"> aaa</div> -->
  </div>
</template>

<style lang="scss"></style>
