import {appConfigDir, BaseDirectory} from '@tauri-apps/api/path'
import {exists} from '@tauri-apps/api/fs'
import {invoke} from '@tauri-apps/api/tauri'
import {writable, get, type Writable} from 'svelte/store'


class GlobalState {

    private configFilePath: string = ''
    private environmentFilePath: string = ''

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

    uploadDirectory: Optional<string> = undefined
    apiEndpoint: Optional<string> = undefined

    async init() {
        this.configFilePath = (await appConfigDir()) + '/app.config'
        await this.openOrCreateLocalConfig()
        await this.loadEnvironment()
    }

    async reload(): Promise<void> {
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
            this._projects.set(environment.projects)
            this.uploadDirectory = environment.uploadDirectory
            this.apiEndpoint = environment.apiEndpoint
        } catch (err) {
            console.error(err)
        }
    }

}


export const STATE = new GlobalState()
