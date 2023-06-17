export const useEncodingStore = defineStore('encoding', () => {
    const readEncoding = ref('str')
    const writeEncoding = ref('str')
    return { readEncoding, writeEncoding }
})
