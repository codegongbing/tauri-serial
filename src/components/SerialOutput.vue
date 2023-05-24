<script setup lang="ts">

import { useOutputStore } from '@/store/useOutputStore';
import { time } from 'console';

const outputStore = useOutputStore();

const outputPanel = ref<HTMLElement | null>(null);

const scrollToBottom = () => {
    if (outputPanel.value !== null) {
        outputPanel.value.scrollTop = outputPanel.value.scrollHeight;
    }
}
watch(() => outputStore.recordLength, () => {
    setTimeout(() => scrollToBottom(), 0)
})

const test = () => {
    //判断outputStore.get是否为[]
    if (outputStore.get.length === 0) {

    }
    outputStore.addRecord("测试数据");
    outputStore.addRecord("测试数据");
    outputStore.addRecord("测试数据");
    outputStore.addRecord("测试数据");

}

const getNowTime = () => {
    const now = new Date()
    return now.getHours() + ":" + now.getMinutes() + ":" + now.getSeconds()
}

</script>

<template>
    <div ref="outputPanel" @click="test" class="output-board">
        <div v-for="(record, index) in outputStore.get" :key="index" class="chat chat-start">
            <div class="chat-header flex items-center">
                <button class="ml-4">这里应该放串口名称</button>
                <time class="ml-4 text-xs opacity-50">
                    {{ "时间:" + getNowTime() }}
                </time>
            </div>
            <div class="chat-bubble">{{ record }}</div>
        </div>
    </div>
</template>

<style lang="scss" scoped>
::-webkit-scrollbar {
    width: 4px;
    height: 8px;
}

::-webkit-scrollbar-thumb {
    background: #ccc;
    border-radius: 4px;
}

::-webkit-scrollbar-track {
    border-radius: 4px;
    margin: 15px;
}

::-webkit-scrollbar-button {
    width: 0;
    height: 0;
}

.output-board {
    @apply basis-3/4 border-solid border-2 rounded-xl p-2 overflow-y-auto scroll-smooth relative
}

@media (prefers-color-scheme: dark) {
    .output-board {

        // :deep(.el-scrollbar__wrap){
        //     overflow-y: hidden;
        // }

        @apply border-gray-400
    }

}
</style>