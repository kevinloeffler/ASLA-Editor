import {invoke} from '@tauri-apps/api/tauri'
import {STATE} from '$lib/services/state-manager.js'

// create global state
STATE.init().then()  // removed await because top-level await support

setTimeout( () => invoke('start_app').then(), 1000 )
// short delay to wait for all windows being initialized
// should be done in rust with some kind of check if the windows are ready
