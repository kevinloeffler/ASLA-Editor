import {appConfigDir, BaseDirectory} from '@tauri-apps/api/path'
import {createDir, exists, writeTextFile} from '@tauri-apps/api/fs'
import {invoke} from '@tauri-apps/api/tauri'
import {writable, get, type Writable} from 'svelte/store'
import {open} from '@tauri-apps/api/dialog'


class GlobalState {

    private configFilePath: string = ''
    private environmentFilePath: string = ''

    hasEnvironment: Writable<boolean> = writable(false)

    uploadDirectory: Optional<string> = undefined
    apiEndpoint: Optional<string> = undefined

    async init() {
        this.configFilePath = (await appConfigDir()) + 'app.config'
        // await this.openOrCreateLocalConfig()
        await this.loadConfig()
        await this.loadEnvironment()
    }

    async reload(): Promise<void> {
        await this.loadConfig()
        await this.loadEnvironment()
    }

    private async loadConfig(): Promise<any> {
        try {
            if (! await exists(this.configFilePath)) throw new Error('config file does not exist')
            const config: any = await invoke('load_config_file', {path: this.configFilePath})
            this.environmentFilePath = config.environment
        } catch (err) {
            console.warn('Cant load config file, showing welcome screen...', err)
            await invoke('show_welcome_window')
        }
    }

    private async loadEnvironment(): Promise<void> {
        try {
            if (! await exists(this.environmentFilePath)) throw new Error('environment file does not exist')
            const environment: any = await invoke('load_environment_file', {path: this.environmentFilePath})
            this._projects.set(environment.projects)
            this.uploadDirectory = environment.uploadDirectory
            this.apiEndpoint = environment.apiEndpoint
            this.hasEnvironment.set(true)
        } catch (err) {
            console.warn('Cant load environment:', err)
            this.hasEnvironment.set(false)
            await invoke('show_welcome_window')
        }
    }

    async writeConfigFile(environmentPath: string): Promise<boolean> {
        try {
            const config = {
                environment: environmentPath
            }
            // create app config directory
            const configDirPath = await appConfigDir()
            if (! await exists(configDirPath)) { await createDir(configDirPath) }
            // write config file
            await writeTextFile('app.config', JSON.stringify(config), {dir: BaseDirectory.AppConfig})
        } catch (err) {
            console.error('Could not write config file', err)
            return false
        }

        // await STATE.reload()
        return true
    }

    _projects: Writable<Project[]> = writable([])
    projects = {
        get: (code: string) => {
            return get(this._projects).find((project) => project.code === code)
        },
        all: () => {
            return get(this._projects)
        },
        add: async (project: Project) => {
            this._projects.update(projects => {
                projects.push(project)
                return projects
            })
            try {
                await invoke('update_environment_project', {
                    path: this.environmentFilePath,
                    newProjects: get(this._projects)})
            } catch (err) {
                console.error(err)
            }
        },
        update: async (project: Project) => {
            this._projects.update(projects => {
                const index = projects.findIndex(project => project.code === project.code)
                if (index > -1) {
                    projects[index] = project
                }
                return projects
            });
            try {
                await invoke('update_environment_project', {
                    path: this.environmentFilePath,
                    newProjects: get(this._projects)
                });
            } catch (err) {
                console.error(err)
            }
        },
        delete: async (code: string) => {
            this._projects.update(projects => projects.filter((project: Project) => project.code !== code))
            try {
                await invoke('update_environment_project', {
                    path: this.environmentFilePath,
                    newProjects: get(this._projects)
                });
            } catch (err) {
                console.error(err)
            }
        }
    }

    /*
    private async openOrCreateLocalConfig() {
        if (! await exists('app.config', {dir: BaseDirectory.AppConfig})) {
            // App is launched for the first time: create config
            console.warn('app config does not exists, creating a new one...')

            await invoke('show_welcome_window')

        } else {
            // Normal app launch: read config
            try {
                console.log('loading config file with path:', this.configFilePath)
                const config: any = await invoke('load_config_file', {path: this.configFilePath})
                this.environmentFilePath = config.environment
            } catch (err) {
                // could not read the config file: create a new one
                console.error(err)
            }
        }
    }

    private async loadEnvironment() {
        try {
            const environment: any = await invoke('load_environment_file', {path: this.environmentFilePath})
            this._projects.set(environment.projects)
            this.uploadDirectory = environment.uploadDirectory
            this.apiEndpoint = environment.apiEndpoint
            this.hasEnvironment.set(true)
            console.log('set hasEnv to true')
        } catch (err) {
            console.error(err)
        }
    }
    */

}


export const STATE = new GlobalState()
