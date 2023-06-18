<script setup lang="ts">

import { useDataStore, useScrollStore } from '@/stores';

const outputStore = useDataStore();
const scrollStore = useScrollStore();

const outputPanel = ref<HTMLElement | null>(null);

const scrollToBottom = () => {
    if (outputPanel.value !== null) {
        outputPanel.value.scrollTop = outputPanel.value.scrollHeight;
    }
}
watch(() => outputStore.outputRecordLength, () => {
    if (scrollStore.isScrolledToBottom) {
        setTimeout(() => scrollToBottom(), 0)
    }
})


const strToHex = (index: number) => {
    const hexData = outputStore.get[index].data.split('').map((item) => {
        return "0x" + item.charCodeAt(0).toString(16).toUpperCase()
    }).join(', ')
    outputStore.outputRecords[index].data = hexData
    outputStore.outputRecords[index].encoding = "hex"
}

const hexToStr = (index: number) => {
    const strData = outputStore.get[index].data.split(' ').map((item) => {
        return String.fromCharCode(parseInt(item, 16))
    }).join('')
    outputStore.outputRecords[index].data = strData
    outputStore.outputRecords[index].encoding = "str"
}

</script>

<template>
    <div ref="outputPanel" class="output-board">
        <template v-for="(record, index) in outputStore.get" :key="index">
            <div class="my-2 chat" :class="record.type === 'output' ? 'chat-start' : 'chat-end'">
                <div class="chat-header flex items-center">
                    <div data-tip="点击在str与hex中切换" class="tooltip chat-header-encoding mr-2" v-if="record.type === 'input'">
                        <span @click="strToHex(index)" v-if="outputStore.getEncoding(index) === 'str'"
                            class="duration-1000">
                            str
                        </span>
                        <span @click="hexToStr(index)" v-else>hex</span>
                    </div>
                    <time class="ml-1 text-xs opacity-50">
                        {{ "时间:" + record.time }}
                    </time>
                    <div data-tip="点击在str与hex中切换" class="tooltip chat-header-encoding ml-2" v-if="record.type === 'output'">
                        <span @click="strToHex(index)" v-if="outputStore.getEncoding(index) === 'str'"
                            class="duration-1000">
                            str
                        </span>
                        <span @click="hexToStr(index)" v-else>hex</span>
                    </div>
                </div>
                <div class="chat-bubble">{{ record.data }}</div>
            </div>
        </template>
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

        @apply border-gray-400
    }

}

.chat-header-encoding {
    cursor: url('@/assets/link.cur'), auto;
}
</style>