<script setup lang="ts">
import SerialSider from '@/components/SerialSider.vue';
import SerialOutPut from '@/components/SerialOutput.vue';
import SerialInput from '@/components/SerialInput.vue';
</script>

<template>
  <div class="flex h-screen w-screen">
    <!--    <el-container class="container aspect-video flex flex-nowrap relative">-->
    <el-container class="p-5 h-screen flex flex-nowrap relative w-screen">
      <SerialSider class=""></SerialSider>
      <div class="w-7"></div>
      <el-container class="flex flex-col flex-grow basis-3/4">
        <SerialOutPut></SerialOutPut>
        <div class="h-10">
        </div>
        <SerialInput></SerialInput>
      </el-container>
    </el-container>
    <div class="h-10"> aaa</div>
  </div>
</template>

<style lang="scss"></style>
