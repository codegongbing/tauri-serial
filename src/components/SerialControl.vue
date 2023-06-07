<script setup lang="ts">

const readType = ref('str');

const changeReadType = () => {
    if (readType.value === 'str') {
        readType.value = 'hex'
    } else {
        readType.value = 'str'
    }
}

</script>

<template>
    <div class="h-10 flex items-center">
        <div class="tooltip" data-tip="点击切换接收格式">
            <label class="swap badge badge-info badge-outline text-white text-center"
                :class="readType === 'str' ? ['swap-active', ' badge-warning'] : ['badge-info']" @click="changeReadType">
                <div class="swap-on fill-current w-15 m-2">接收:str</div>
                <div class="swap-off fill-current w-15 m-2">接收:hex</div>
            </label>
        </div>
    </div>
</template>

<style lang="scss" scoped></style>