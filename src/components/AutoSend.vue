<script setup lang="ts">
import { useElementHover } from "@vueuse/core";

// 接收父组件传递的字符串
const props = defineProps({
    inputRecord: {
        type: String,
        required: true
    }
})


const autoSendRef = ref();
const isHovered = useElementHover(autoSendRef);
const autoSendTime = ref(1000);
const isAutoSending = ref(false);

const emit = defineEmits(["send"]);

let timer: any = undefined;

const content = computed(() => {
    if (isAutoSending.value) {
        if (isHovered.value) {
            return "取消自动发送";
        }
        return "自动发送中";
    } else {
        if (isHovered.value && autoSendTime.value) {
            return "开始自动发送";
        }
        return "自动发送";
    }
});

async function timeoutFunc() {
    if (isAutoSending.value) {
        emit("send");
        if (autoSendTime.value <= 0) return;
        timer = setTimeout(timeoutFunc, autoSendTime.value);
    }
}

async function toggleAutoSend() {
    if (isAutoSending.value) {
        isAutoSending.value = false;
        clearTimeout(timer);
    } else {
        if (autoSendTime.value <= 0) return;
        isAutoSending.value = true;
        emit("send");
        timer = setTimeout(timeoutFunc, autoSendTime.value);
    }
}

const btnState = () => {

    if (props.inputRecord === "") {
        return 'btn-disabled';
    } else {
        if (isAutoSending.value) {
            return 'btn-success hover:btn-error'
        } else {
            return 'btn-ghost'
        }
    }
}

</script>

<template>
    <div class="dropdown dropdown-top dropdown-end dropdown-hover" ref="autoSendRef">
        <button class="btn btn-sm mb-1 mr-2 px-7" :class="btnState()" @click="toggleAutoSend">
            {{ content }}
        </button>
        <ul tabindex="0" class="dropdown-content p-3 shadow bg-base-200 rounded-box">
            <!-- 填写自动发送时间 与开始按钮 -->
            <div class="flex items-center justify-end">
                <input type="number" placeholder="输入间隔时长" class="ml-2 input input-bordered input-xs w-24"
                    v-model="autoSendTime" />
                <div class="w-1"></div>
                <div class="text-sm flex-none">ms/次</div>
            </div>
        </ul>
    </div>
</template>

<style lang="scss" scoped></style>