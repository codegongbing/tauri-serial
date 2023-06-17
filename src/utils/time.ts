// 静态类
class Time {

    static getNowTime = () => {
        const now = new Date()
        return now.getHours() + ":" + now.getMinutes() + ":" + now.getSeconds()
    }
}

export default Time