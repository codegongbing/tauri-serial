<script setup lang="ts">

import { baudRate, dataBits, checkBit, stopBits, siderData } from "@/types/settings-data";
import { serialEventData, serialPayload, outputEvent } from "@/types/event-data";
import { SerialInfo } from "@/types/serial-info";
import { invoke } from '@tauri-apps/api/tauri';
import { listen, once } from '@tauri-apps/api/event';
import { useDataStore, useEncodingStore, useBytesStore } from '@/stores';
import Time from '@/utils/time';

const outputStore = useDataStore();
const encodingStore = useEncodingStore();
const bytesStore = useBytesStore();

const baudRates = [
  { value: '115200', label: '波特率:115200' },
  { value: '57600', label: '波特率:57600' },
  { value: '38400', label: '波特率:38400' },
  { value: '19200', label: '波特率:19200' },
  { value: '9600', label: '波特率:9600' },
] as baudRate[]

const dataBits = [
  { value: '8', label: '数据位:8' },
  { value: '7', label: '数据位:7' },
  { value: '6', label: '数据位:6' },
  { value: '5', label: '数据位:5' },
] as dataBits[]

const checkBits = [
  { value: 'none', label: '校验位:none' },
  { value: 'even', label: '校验位:even' },
  { value: 'odd', label: '校验位:odd' },
] as checkBit[]

const stopBits = [
  { value: '1', label: '停止位:1' },
  { value: '2', label: '停止位:2' },
] as stopBits[]

const sideData = [
  { name: '波特率', data: baudRates as baudRate[], value: ref('115200') },
  { name: '数据位', data: dataBits as dataBits[], value: ref('8') },
  { name: '校验位', data: checkBits as checkBit[], value: ref('none') },
  { name: '停止位', data: stopBits as stopBits[], value: ref('1') },
] as siderData[]

const dialogTableVisible = ref(false)
// 是否连接
const isConnected = ref(false)
// 连接是否断开
const isPaused = ref(false)
// 是否暂停接收
const isSuspended = ref(false)
const serialInfo = ref([] as SerialInfo[])
const serialChoiceRef = ref()

const getSerialProcess = async () => {
  await invoke('get_serial_process')
  await once('serial-port', (event: serialEventData) => {
    serialInfo.value = []
    event.payload.forEach((item: serialPayload) => {
      serialInfo.value.push({ index: item.id, name: item.port_name })
    })
  })
}

onMounted(async () => {
  await getSerialProcess()
})

const chooseSerial = async (index: number) => {
  const serial = serialInfo.value[index].name

  await invoke('choose_serial', { serial: serial })
  let unlistenFn = await listen('output-data', (event: outputEvent) => {
    // 如果串口拔出，关闭监听
    if (!isPaused.value) {
      outputStore.emitData = event.payload
      if (outputStore.emitData.is_suspended) {
        unlistenFn()
      } else {
        outputStore.addRecord({
          type: "output",
          encoding: encodingStore.readEncoding,
          time: Time.getNowTime(),
          data: encodingStore.readEncoding === 'hex' ? outputStore.emitData.data.split('').map((item) => {
            return "0x" + item.charCodeAt(0).toString(16).toUpperCase()
          }).join(', ') : outputStore.emitData.data,
        })
        bytesStore.rxLength += outputStore.emitData.data.length
      }
    }
  })
  dialogTableVisible.value = false
  isConnected.value = true
  isSuspended.value = false
}

// let isDark = ref(window.matchMedia('(prefers-color-scheme: dark)').matches)

const setSerialSettings = async () => {
  dialogTableVisible.value = true

  await getSerialProcess()
  const data = {
    // 转为int
    baud_rate: parseInt(sideData[0].value.value),
    data_bits: parseInt(sideData[1].value.value),
    check_bit: sideData[2].value.value,
    stop_bits: parseInt(sideData[3].value.value),
  }
  await invoke('set_serial_settings', { data: data })
}

const closeOrReconnectSerial = async () => {
  if (isSuspended.value === true) {
    isSuspended.value = false
  } else {
    isSuspended.value = true
  }
  await invoke('close_or_reconnect_serial', { state: isSuspended.value })
}

</script>
<!-- 边栏 -->
<template>
  <div class="dark-aside basis-1/4 border-solid border-2 rounded-xl p-5">
    <div class="flex flex-col">
      <!--   设置   -->
      <div class="side-select flex flex-col">
        <!--    选择串口    -->
        <el-button class="serial-button w-full h-10" @click="setSerialSettings">
          <div class="inline-block w-2 h-2 rounded m-1"
            :class="{ 'bg-red-500': !isConnected, 'bg-green-500': isConnected }">
          </div>
          <span class="span-text">选择串口</span>
        </el-button>
        <!--    设置属性    -->
        <div ref="serialChoiceRef" v-for="sd in sideData">
          <el-select class="w-full mt-4" :teleported="false" v-model="sd.value.value" size="large">
            <el-option class="option-disabled" disabled :value="'设置' + sd.name" />
            <el-option class="text-gray-400" v-for="data in sd.data" :key="data.value" :label="data.label"
              :value="data.value" />
          </el-select>
        </div>
      </div>
      <button v-if="isConnected && !isSuspended" class="btn btn-error mt-4 suspend-btn" @click="closeOrReconnectSerial">
        <span class="text-xs">断 开</span>
      </button>
      <button v-if="isConnected && isSuspended" class="btn mt-4 suspend-btn" @click="closeOrReconnectSerial">
        <span class="text-xs">重 连</span>
      </button>
      <!--   空白   -->
      <div class="flex-1"></div>
      <!--   底部   -->
      <div></div>
    </div>
  </div>
  <!-- 对话框 -->
  <div class="serial-dialog">
    <el-dialog title="请选择要连接的串口" v-model="dialogTableVisible">
      <div v-for="item in serialInfo">
        <div class="serial-select" @click="chooseSerial(item.index)">{{ item.name }}</div>
      </div>
    </el-dialog>
  </div>
</template>

<style lang="scss" scoped>
.option-disabled {
  @apply flex justify-center
}

// @media (prefers-color-scheme: light) {}

// @media (prefers-color-scheme: dark) {
.option-disabled {
  cursor: url('@/assets/normal_simple.cur'), auto !important;
}

.serial-button {
  @apply bg-gray-800 border-gray-400 border-[2px]
}

.serial-button:hover {
  @apply bg-gray-800 border-gray-400
}

.serial-button:focus {
  @apply bg-gray-800 border-gray-400 border-[2px]
}

.serial-button::after {
  @apply bg-gray-800 border-gray-400 border-[2px]
}

.dark-aside {
  @apply border-gray-400
}

.side-select {
  :deep(.el-input__inner) {
    @apply bg-dark-primary-color text-gray-400
  }

  :deep(.el-input__suffix) {
    @apply bg-dark-primary-color text-gray-400
  }

  :deep(.el-input__wrapper) {
    @apply rounded-md bg-dark-primary-color text-gray-400;
  }


  // 下拉时边框
  :deep(.el-input.is-focus .el-input__wrapper) {
    box-shadow: 0 0 0 3px #DCDFE5 !important;
  }

  :deep(.el-input__wrapper.is-focus) {
    box-shadow: 0 0 0 1px #DCDFE5 inset !important;
  }

  :deep(.el-select-dropdown) {
    @apply bg-dark-primary-color text-gray-400;
  }

  :deep(.el-select-dropdown__item) {
    @apply bg-dark-primary-color text-gray-400;
  }

  :deep(.el-select-dropdown__item:hover) {
    @apply text-gray-400 bg-dark-hover-color;
  }

  :deep(.el-popper.is-light .el-popper__arrow::before) {
    @apply bg-dark-primary-color
  }

  :deep(.el-select-dropdown__item) {
    cursor: url('@/assets/link.cur'), auto;
  }

  :deep(.el-select .el-input__inner) {
    cursor: url('@/assets/link.cur'), auto !important;
  }
}

.span-text {
  @apply text-gray-400
}

.serial-dialog {
  :deep(.el-dialog) {
    @apply bg-[#262727] rounded-md border-[1px] border-gray-400
  }

  :deep(.el-dialog__header) {
    @apply mr-0 text-center
  }

  :deep(.el-dialog__title) {
    @apply text-gray-400
  }

  :deep(.el-dialog__body) {
    @apply pt-3 text-gray-400
  }

  :deep(.el-dialog__headerbtn .el-dialog__close) {
    @apply text-gray-400
  }
}

.serial-select {
  cursor: url('@/assets/link.cur'), auto;
  @apply bg-gray-700 text-gray-400 rounded-md p-2 m-2
}

.suspend-btn {
  cursor: url('@/assets/link.cur'), auto !important;
}

// }
</style>

<style lang="scss"></style>