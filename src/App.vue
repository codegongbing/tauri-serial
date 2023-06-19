<script setup lang="ts">
import SerialSider from "@/components/SerialSider.vue";
import SerialOutPut from "@/components/SerialOutput.vue";
import SerialControl from "@/components/SerialControl.vue";
import SerialInput from "@/components/SerialInput.vue";

const serialControlRef = ref();

const serialOutputRef = ref();

const test = () => {
  serialControlRef.value.changeReadType();
};
</script>

<template>
  <div class="flex flex-col h-screen w-screen overflow-hidden" @contextmenu.prevent>
    <!--    <el-container class="container aspect-video flex flex-nowrap relative">-->
    <div class="p-5 h-screen flex flex-nowrap relative w-screen">
      <SerialSider></SerialSider>
      <div class="w-7"></div>
      <div class="flex flex-col flex-grow basis-3/4">
        <SerialOutPut></SerialOutPut>
        <SerialControl ref="serialControlRef"></SerialControl>
        <SerialInput></SerialInput>
      </div>
    </div>
    <!-- <div class="h-10"> aaa</div> -->
  </div>
</template>

<style lang="scss"></style>
