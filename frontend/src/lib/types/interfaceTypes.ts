export interface Signal {
    name: string;
    uom: string;
    uom_symbol: string;
}

export type InterfaceType = {
    [key:string]: Signal
}

export interface InterfaceData {
    interface_type: string,
    uuid: string,
    name: string,
    url: string,
    signals: InterfaceType
}


