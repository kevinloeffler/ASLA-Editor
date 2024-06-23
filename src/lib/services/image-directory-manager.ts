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
        await this.lock.acquire()
        for (const image of snapshot) {
            if (!this.directory.has(image.path)) {
                this.directory.set(image.path, {tries: 0, ...image})
            }
        }
        this.lock.release()
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
        await this.lock.acquire()

        for (let [_, image] of this.directory) {
            try {
                const response: ApiResponse = await invoke('process_image', {path: image.path, name: image.name})

                // response was ok: remove image from directory
                if (response.status === 'ok') {
                    continue
                }

                // response was error: handle cases
                console.warn(`Error ${response.code}: ${response.msg}`)
                if (this.ERRORS_WITH_RETRY.some(el => el.statusCode === response.code)) {
                    if (image.tries > this.MAX_RETRIES) {
                        console.warn(`Image ${image.name} has exceeded retry threshold`)
                        continue
                    } else { image.tries += 1 }
                }
                console.warn(`Image ${image.name} could not be uploaded`)

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


