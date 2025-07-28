import { invoke } from "@tauri-apps/api/core";

export async function ping(value: string): Promise<string | null> {
  return await invoke<{ value?: string }>("plugin:android-tv-check|ping", {
    payload: {
      value,
    },
  }).then((r) => (r.value ? r.value : null));
}

export async function check(): Promise<string | null> {
  return await invoke<{ value?: string }>("plugin:android-tv-check|check").then(
    (r) => (r.value ? r.value : null),
  );
}
