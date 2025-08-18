import { invoke } from "@tauri-apps/api/core";

export async function check(): Promise<string | null> {
  return await invoke<{ value?: string }>("plugin:androidtvcheck|check").then(
    (r) => (r.value ? r.value : null),
  );
}
