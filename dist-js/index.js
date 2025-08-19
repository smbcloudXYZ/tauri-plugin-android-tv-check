import { invoke } from '@tauri-apps/api/core';

async function check() {
    return await invoke("plugin:android-tv-check|check");
}

export { check };
