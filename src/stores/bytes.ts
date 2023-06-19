export const useBytesStore = defineStore('bytes', () => {
    const rxLength = ref(0)
    const txLength = ref(0)
    return { rxLength, txLength }
})