
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
