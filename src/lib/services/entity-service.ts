
export enum ENTITIES {
    Location = 'loc',
    Client = 'clt',
    Scale = 'mst',
    Date = 'date',
    CreationDate = 'cdate',
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
