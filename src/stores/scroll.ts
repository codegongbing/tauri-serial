export const useScrollStore = defineStore('scroll', () => {
    const isScrolledToBottom = ref(true)
    return { isScrolledToBottom }
})