<script setup lang="ts">
import AutoSend from '@/components/AutoSend.vue'

const inputRecord = ref("")

const send = () => {
    console.log(inputRecord.value);
}

</script>

<template>
    <div class="flex flex-col basis-1/4 border-solid border-2 border-gray-400 rounded-xl">
        <div class="h-2/3 flex">
            <div class="w-[93%]">
                <textarea id="send-panel-input" ref="inputRef" class="text-area" autocomplete="send-panel-input"
                    v-model="inputRecord"></textarea>
            </div>
            <div class="w-[7%] flex justify-end pt-2 pr-2">
                <div class="clear-content tooltip" data-tip="清除输入框">
                    <button class="btn btn-sm btn-circle bg-transparent border-none" @click="() => { inputRecord = '' }">
                        <img src="@/assets/clear_context.svg" class="p-[0.35rem]">
                    </button>
                </div>
            </div>
        </div>
        <div class="h-1/3 flex justify-end items-center">
            <AutoSend :inputRecord="inputRecord" @send="send"></AutoSend>
            <button class="btn btn-sm mr-2 mb-1 px-7" :class="inputRecord ? '' : 'btn-disabled'" @click="send">
                发 送
            </button>
        </div>

    </div>
</template>

<style lang="scss" scoped>
.text-area {
    resize: none;
    scrollbar-width: none;
    border: none;
    outline: none;
    @apply w-full h-full px-2 pt-1 basis-1/4 rounded-xl bg-transparent
}

.text-area::-webkit-scrollbar {
    width: 0;
    height: 0;
}

@media (prefers-color-scheme: dark) {
    button {
        cursor: url('@/assets/link.cur'), auto;
    }
}
</style>