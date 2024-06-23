import {fs} from '@tauri-apps/api'
import type {FileEntry} from '@tauri-apps/api/fs'
import {invoke} from '@tauri-apps/api/tauri'
import {type Writable, writable} from 'svelte/store'
import {STATE} from '$lib/services/state-manager'

class UploadService {

    constructor(uploadDirectory: string) {
        this.uploadDirectory = uploadDirectory
        this.uploading.subscribe(value => this._uploading = value)
        this.images.subscribe(value => this._images = value)
        this.failedImages.subscribe(value => this._failedImages = value)
        this.currentUpload.subscribe(value => this._currentUpload = value)
    }

    private uploadDirectory: string

    uploading = writable(false)
    private _uploading: boolean = false

    images: Writable<FileEntry[]> = writable([])
    private _images: FileEntry[] = []

    failedImages: Writable<string[]> = writable([])
    private _failedImages: string[] = []

    currentUpload: Writable<Optional<string>> = writable(undefined)
    private _currentUpload: Optional<string> = undefined

    setDirectory(path: string) {
        this.uploadDirectory = path
    }

    async refresh(): Promise<void> {
        try {
            const files = await fs.readDir(this.uploadDirectory)
            this.images.set(files.filter(file => this.isImage(file.name)))
        } catch (err) {
            console.error(err)
        }
    }

    async startUpload(): Promise<void> {
        if (this._uploading) return

        await this.refresh()

        this.uploading.set(true)
        while (this._uploading) {
            let image: Optional<FileEntry> = undefined
            for (let img of this._images.toReversed()) {
                if (!img || this._failedImages.includes(img.name!)) {
                    continue
                }
                image = img
            }
            if (image) {
                await this.uploadImage(image)
            } else {
                await new Promise(resolve => setTimeout(resolve, 2000))
            }
            await this.refresh()
        }
    }

    stopUpload() {
        this.uploading.set(false)
    }

    whitelistImage(name: string) {
        this.failedImages.update( images => images.filter(image => image !== name))
    }

    private async uploadImage(image: FileEntry): Promise<void> {
        try {
            const pattern = /^ASLA_([a-zA-Z0-9]+)_.*$/
            const match = image.name?.match(pattern)
            const projectCode = match ? match[1] : ''
            const project = STATE.projects.get(projectCode)
            const artefacts = project?.artefacts || []
            const workingDirectory = project?.workingDirectory
            if (!workingDirectory) throw new Error('Image does not match any project')

            this.currentUpload.set(image.name)
            const _: ApiResponse = await invoke('process_image', {
                path: image.path,
                name: image.name,
                endpoint: STATE.apiEndpoint,
                directory: workingDirectory,
                artefacts: artefacts,
            })
        } catch (err) {
            this.failedImages.set([...this._failedImages, image.name!])
            console.error('Upload failed:', err)
        }
        this.currentUpload.set(undefined)
    }

    private isImage(filename: Optional<string>): boolean {
        const imageSuffix = ['.jpg', '.jpeg']
        for (let suffix of imageSuffix) {
            if (filename?.toLowerCase().endsWith(suffix)) {
                return true
            }
        }
        return false
    }

}


export const UPLOAD_SERVICE = new UploadService('')
