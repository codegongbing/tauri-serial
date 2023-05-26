interface OutputData {
    type: string
    encoding: string
    time: string
    data: string
}

export const useOutputStore = defineStore('output', () => {
    const outputRecords = ref([] as OutputData[])
    const get = computed(() => outputRecords.value)
    const getEncoding = (index: number) => outputRecords.value[index].encoding
    const outputRecordLength = computed(() => outputRecords.value.length)
    const clear = () => outputRecords.value = []
    const addRecord = (record: OutputData) => outputRecords.value.push(record)
    return { outputRecords, get, getEncoding, outputRecordLength, clear, addRecord }
})
