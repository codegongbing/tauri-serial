interface OutputData {
    type: string
    encoding: string
    time: string
    data: string
}

interface emitData {
    data: string,
    is_suspended: boolean
}

export const useDataStore = defineStore('output', () => {
    const emitData = ref({ data: '', is_suspended: false } as emitData)
    const dataRecords = ref([] as OutputData[])
    const get = computed(() => dataRecords.value)
    const getEncoding = (index: number) => dataRecords.value[index].encoding
    const dataLength = computed(() => dataRecords.value.length)
    const clear = () => dataRecords.value = []
    const addRecord = (record: OutputData) => dataRecords.value.push(record)
    return { emitData, outputRecords: dataRecords, get, getEncoding, outputRecordLength: dataLength, clear, addRecord }
})
