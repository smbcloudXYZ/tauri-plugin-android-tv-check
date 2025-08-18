import { invoke } from "@tauri-apps/api/core";

export async function check(): Promise<{ isAndroidTv: boolean }> {
  return await invoke("plugin:android-tv-check|check");
}
