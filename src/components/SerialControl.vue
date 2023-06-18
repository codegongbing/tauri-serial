<script setup lang="ts">
import { useDataStore, useEncodingStore, useScrollStore } from '@/stores';
const dataStore = useDataStore();
const encodingStore = useEncodingStore();
const scrollStore = useScrollStore();

const changeReadEncoding = () => {
    console.log("changeReadType");
    if (encodingStore.readEncoding === 'str') {
        encodingStore.readEncoding = 'hex'
    } else {
        encodingStore.readEncoding = 'str'
    }
}

const changeWriteEncoding = () => {
    if (encodingStore.writeEncoding === 'str') {
        encodingStore.writeEncoding = 'hex'
    } else {
        encodingStore.writeEncoding = 'str'
    }
}

const changeScrollState = () => {
    scrollStore.isScrolledToBottom = scrollStore.isScrolledToBottom ? false : true;
}

</script>

<template>
    <div class="h-10 flex items-center justify-between">
        <div class="">
            <div class="tooltip ml-3" data-tip="点击切换接收格式">
                <label class="btn-control h-6 swap badge badge-info badge-outline text-white text-center"
                    :class="encodingStore.readEncoding === 'str' ? ['swap-active', ' badge-warning'] : ['badge-info']"
                    @click="changeReadEncoding">
                    <div class="swap-on fill-current w-15 mx-2">接收:str</div>
                    <div class="swap-off fill-current w-15 mx-2">接收:hex</div>
                </label>
            </div>
            <div class="tooltip ml-3" data-tip="点击切换发送格式">
                <label class="btn-control h-6 swap badge badge-info badge-outline text-white text-center"
                    :class="encodingStore.writeEncoding === 'str' ? ['swap-active', ' badge-warning'] : ['badge-info']"
                    @click="changeWriteEncoding">
                    <div class="swap-on fill-current w-15 m-2">发送:str</div>
                    <div class="swap-off fill-current w-15 m-2">发送:hex</div>
                </label>
            </div>
        </div>
        <div class="mr-4">
            <div class="tooltip mr-4" data-tip="点击切换是否滚动">
                <label class="btn-control h-6 swap badge badge-info badge-outline text-white text-center"
                    :class="scrollStore.isScrolledToBottom ? ['swap-active', 'badge-info'] : ['badge-warning']"
                    @click="changeScrollState">
                    <div class="swap-on fill-current w-15 m-2">自动滚动</div>
                    <div class="swap-off fill-current w-15 m-2">不滚动</div>
                </label>
                <!-- <button v-if="isAutoScroll" class="btn btn-warning btn-sm" @click="changeScrollState">
                                <span>自动滚动</span>
                            </button>
                            <button v-else class="btn btn-info btn-sm flex" @click="changeScrollState">
                                <span>不滚动</span>
                            </button> -->
            </div>
            <div class="btn-control h-6 badge badge-error hover:badge-outline cursor-pointer select-none"
                @click="dataStore.clear">清屏</div>
            <!-- <button @click="dataStore.clear" class="btn btn-outline btn-error btn-sm">清屏</button> -->
        </div>
    </div>
</template>

<style lang="scss" scoped>
@media (prefers-color-scheme: dark) {
    .btn-control {
        cursor: url('@/assets/link.cur'), auto !important;
    }
}
</style>