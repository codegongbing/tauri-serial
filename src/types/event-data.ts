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
    serialPayload,
    serialEventData,
}