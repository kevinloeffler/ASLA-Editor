import type {FileEntry} from '@tauri-apps/api/fs'
import { invoke } from '@tauri-apps/api/tauri'
import {GeneralPipelineError} from '$lib/errors.js'


export class ImageDirectoryManager {

    constructor() {}

    directory = new Map<string, Image>()

    private lock = new Lock()
    private uploading = false

    private ERRORS_WITH_RETRY = [GeneralPipelineError]
    private MAX_RETRIES = 3

    async update(snapshot: FileEntry[]): Promise<void> {
        console.log('waiting to update')
        await this.lock.acquire()
        console.log('updating:', snapshot)
        for (const image of snapshot) {
            if (!this.directory.has(image.path)) {
                this.directory.set(image.path, {tries: 0, ...image})
            }
        }
        this.lock.release()
        console.log('directory:', this.directory)
        await this.upload()
    }

    async startUpload(): Promise<void> {
        this.uploading = true
        await this.upload()
    }

    stopUpload(): void {
        this.uploading = false
    }

    private async upload(): Promise<void> {
        if (!this.uploading) { return }
        console.log('waiting to upload')
        await this.lock.acquire()
        console.log('uploading')

        for (let [_, image] of this.directory) {
            try {
                const response: ApiResponse = await invoke('process_image', {path: image.path, name: image.name})
                console.log('response:', response)

                // response was ok: remove image from directory
                if (response.status === 'ok') {
                    console.log(`uploaded ${image.name}`)
                    console.log('deleted:', this.directory.delete(image.path))
                    continue
                }

                // response was error: handle cases
                console.warn(`Error ${response.code}: ${response.msg}`)
                // TODO: replace console.error with warning message
                if (this.ERRORS_WITH_RETRY.some(el => el.statusCode === response.code)) {
                    if (image.tries > this.MAX_RETRIES) {
                        console.warn(`Image ${image.name} has exceeded retry threshold`)
                        // TODO: alert user that the image could not be uploaded, options:
                        //  Delete image -OR- ignore
                        continue
                    } else { image.tries += 1 }
                }
                console.warn(`Image ${image.name} could not be uploaded`)
                // TODO: alert user that the image could not be uploaded, options:
                //  Delete image -OR- ignore

            } catch (err) {
                console.error(err)
            }
        }

        this.lock.release()
    }

}

type Image = FileEntry & { tries: number }

class Lock {
    private isLocked: boolean = false;
    private queue: (() => void)[] = [];

    async acquire(): Promise<void> {
        return new Promise<void>((resolve) => {
            if (!this.isLocked) {
                this.isLocked = true;
                resolve();
            } else {
                this.queue.push(() => {
                    this.isLocked = true;
                    resolve();
                });
            }
        });
    }

    release(): void {
        if (this.isLocked) {
            this.isLocked = false;
            const next = this.queue.shift();
            if (next) {
                next();
            }
        }
    }
}


