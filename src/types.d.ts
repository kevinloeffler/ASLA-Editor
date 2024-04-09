
/* OPTIONAL TYPES */

// generic optional
type Optional<T> = undefined | T
// base type optionals
type u_string = string | undefined
type u_number = number | undefined
type u_boolean = boolean | undefined


type ApiResponse = ApiResponseOk | ApiResponseError

type ApiResponseOk = {
    status: 'ok',
    image: string,
    prediction: any,
}

type ApiResponseError = {
    status: 'error',
    code: number,
    msg: string,
}

type Project = {
    code: string,
    name: string,
    workingDirectory: string,
    exportDirectory: string,
    subfolders: boolean,
    artefacts: string[],
}

/* EDITOR */

type Metadata = {
    entities: Entity[],
    grading: {
        contrast: number,
        brightness: number,
        sharpness: number,
        whiteBalance: [number, number, number],
        manuallyChanged: boolean
    },
    format: {
        crop: {
            top: number,
            right: number,
            bottom: number,
            left: number
        },
        rotation: number,
        manuallyChanged: boolean
    }
}

type Entity = {
    label: string,
    text: string,
    boundingBox: {
        top: number,
        right: number,
        bottom: number,
        left: number
    },
    manuallyChanged: boolean
}

type EditorResponse = [string, Metadata]

