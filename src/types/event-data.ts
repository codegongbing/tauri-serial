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

interface outputEvent {
    event: string,
    windowLabel: string,
    payload: emitData
}

interface emitData {
    data: string,
    is_close: boolean
}

export type {
    serialPayload,
    serialEventData,
    outputEvent,
    emitData
}