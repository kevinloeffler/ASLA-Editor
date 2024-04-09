abstract class AslaError extends Error {
    static statusCode: number  // for static access
    // @ts-ignore
    statusCode: number  // for normal access
    protected constructor(message: string) {
        super(message)
    }
}

export class ConnectionError extends AslaError {
    static statusCode = 201
    statusCode = 201
    constructor(message: string) {
        super(message)
    }
}

export class GeneralPipelineError extends AslaError {
    static statusCode = 301
    statusCode = 301
    constructor(message: string) {
        super(message)
    }
}

export class CouldNotStartPipelineError extends AslaError {
    static statusCode = 302
    statusCode = 302
    constructor(message: string) {
        super(message)
    }
}

export class UnsupportedFileTypeError extends AslaError {
    static statusCode = 303
    statusCode = 303
    constructor(message: string) {
        super(message)
    }
}
