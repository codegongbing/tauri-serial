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
    const outputRecords = ref([] as OutputData[])
    const get = computed(() => outputRecords.value)
    const getEncoding = (index: number) => outputRecords.value[index].encoding
    const outputRecordsLength = computed(() => outputRecords.value.length)
    const clear = () => outputRecords.value = []
    const addRecord = (record: OutputData) => outputRecords.value.push(record)
    return { emitData, outputRecords, get, getEncoding, outputRecordsLength, clear, addRecord }
})
