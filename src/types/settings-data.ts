import {Ref} from "vue";

interface baudRate {
    value: string,
    label: string
}

interface dataBits {
    value: string,
    label: string
}

interface checkBit {
    value: string,
    label: string
}

interface stopBits {
    value: string,
    label: string
}

interface siderData {
    name: string,
    data: baudRate[] | dataBits[] | checkBit[] | stopBits[]
    value: Ref<string>,
}

interface serialPayload {
    id: number,
    pid: number,
    vid: number,
    port_name: string,
    port_type: string,
    product: string,
}

interface serialEventData {
    event: string,
    windowLabel: string,
    payload: serialPayload[]
    id: number,
}

export type {
    baudRate,
    dataBits,
    checkBit,
    stopBits,
    siderData,
}