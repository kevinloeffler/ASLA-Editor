import {appConfigDir, BaseDirectory} from '@tauri-apps/api/path'
import {exists} from '@tauri-apps/api/fs'
import {invoke} from '@tauri-apps/api/tauri'


class GlobalState {

    private configFilePath: string = ''
    private environmentFilePath: string = ''

    private projectsCache: Project[] = []
    projects = {
        get: (code: string) => {
            return this.projectsCache.find( (project: Project) => project.code === code)
        },
        all: () => {
            return this.projectsCache
        },
        add: async (project: Project) => {
            this.projectsCache.push(project)
            try {
                await invoke('update_environment_project', {path: this.environmentFilePath, newProjects: this.projectsCache})
            } catch (err) {
                console.error(err)
            }
        },
        update: async (newProject: Project) => {
            const index = this.projectsCache.findIndex( project => project.code === newProject.code)
            if (index > -1) {
                this.projectsCache[index] = newProject
            }
            try {
                await invoke('update_environment_project', {path: this.environmentFilePath, newProjects: this.projectsCache})
            } catch (err) {
                console.error(err)
            }
        },
        delete: async (code: string) => {
            this.projectsCache = this.projectsCache.filter( (project: Project) => project.code !== code)
            try {
                await invoke('update_environment_project', {path: this.environmentFilePath, newProjects: this.projectsCache})
            } catch (err) {
                console.error(err)
            }
        }
    }

    uploadDirectory = undefined
    apiEndpoint = undefined

    async init() {
        this.configFilePath = (await appConfigDir()) + '/app.config'
        await this.openOrCreateLocalConfig()
        await this.loadEnvironment()
    }

    private async openOrCreateLocalConfig() {
        if (! await exists('app.config', {dir: BaseDirectory.AppConfig})) {
            // App is launched for the first time: create config
            console.warn('app config does not exists, creating a new one...')

            await invoke('show_welcome_window')

        } else {
            // Normal app launch: read config
            try {
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
            this.projectsCache = environment.projects
            this.uploadDirectory = environment.uploadDirectory
            this.apiEndpoint = environment.apiEndpoint
        } catch (err) {
            console.error(err)
        }
    }

}


export const STATE = new GlobalState()
