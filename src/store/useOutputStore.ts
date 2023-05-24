// const useOutputStore = defineStore('output', {
//     state: () => ({ records: [] as string[] }),
//     getters: {
//         get: (state) => state.records,
//     },
//     actions: {
//         increment(record: string) {
//             this.records.push(record)
//         }
//     }
// })

export const useOutputStore = defineStore('output', () => {
    const records = ref([] as string[])
    const get = computed(() => records.value)
    const recordLength = computed(() => records.value.length)
    const clear = () => records.value = []
    const addRecord = (record: string) => records.value.push(record)
    return { records, get, recordLength, clear, addRecord }
})
