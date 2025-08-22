import { load } from "@tauri-apps/plugin-store";

export const store = await load("tracker.json", {
  autoSave: false,
  defaults: {},
});
