import { invoke } from "@tauri-apps/api/core";
import { Config } from "./types/config.d.ts";
import { store } from "./store.ts";

const form = document.getElementById("configForm") as HTMLFormElement | null;
const apiUrlInput = document.getElementById("apiUrl") as
  | HTMLInputElement
  | null;
const intervalInput = document.getElementById("interval") as
  | HTMLInputElement
  | null;
const statusEl = document.getElementById("status") as
  | HTMLParagraphElement
  | null;

if (!form || !apiUrlInput || !intervalInput || !statusEl) {
  console.error(
    "Initialization failed: One or more required HTML elements were not found.",
  );
}

async function loadConfig() {
  try {
    await invoke<Config>("get_config");
    const config = await store.get<Config>("config");
    if (config) {
      console.log("Loaded config:", config);
      apiUrlInput!.value = config.api_url;
      intervalInput!.value = String(config.interval_secs);
    }
  } catch (e) {
    console.error("Error loading config:", e);
    statusEl!.textContent = "Error loading config.";
    statusEl!.style.color = "red";
  }
}

await loadConfig();

form!.addEventListener("submit", async (event: SubmitEvent) => {
  event.preventDefault();

  const newConfig: Config = {
    api_url: apiUrlInput!.value,
    // TODO: rust expects a u64 or BigInt but it doesn't know how to serialize it.
    // could be an issue with ts-rs, so may need to check it out
    interval_secs: parseInt(intervalInput!.value, 10) as unknown as bigint,
  };

  try {
    await invoke("update_config", { newConfig });
    store.set("config", newConfig);
    statusEl!.textContent = "Settings saved successfully!";
    statusEl!.style.color = "green";
  } catch (e) {
    console.error("Error saving config:", e);
    statusEl!.textContent = "Failed to save settings.";
    statusEl!.style.color = "red";
  }

  setTimeout(() => {
    if (statusEl) {
      statusEl!.textContent = "";
    }
  }, 3000);
});
