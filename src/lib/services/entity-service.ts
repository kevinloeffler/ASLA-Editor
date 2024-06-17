
export enum ENTITIES {
    Location = 'LOC',
    Client = 'CLT',
    Scale = 'MST',
    Date = 'DATE',
    CreationDate = 'CDATE',
    None = 'O',
}

export function entityToText(entity: string) {
    switch (entity) {
        case ENTITIES.Location:
            return 'Ort'
        case ENTITIES.Client:
            return 'Bauherr:in'
        case ENTITIES.Scale:
            return 'Masstab'
        case ENTITIES.Date:
            return 'Datum'
        case ENTITIES.CreationDate:
            return 'Erstellungsdatum'
        default:
            return 'Undefiniert'
    }
}

export function hashCode(obj: any) {
    let str = JSON.stringify(obj)
    let hash = 0
    for (let i = 0; i < str.length; i++) {
        let char = str.charCodeAt(i)
        hash = ((hash << 5) - hash) + char
        hash |= 0 // Convert to 32bit integer
    }
    return hash
}
