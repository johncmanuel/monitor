import { invoke } from "@tauri-apps/api/core";
import { Config } from "./types/config.d.ts";

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

if (form && apiUrlInput && intervalInput && statusEl) {
  window.addEventListener("DOMContentLoaded", async () => {
    try {
      const config = await invoke<Config>("get_config");
      apiUrlInput.value = config.api_url;
      intervalInput.value = String(config.interval_secs);
    } catch (e) {
      console.error("Error loading config:", e);
      statusEl.textContent = "Error loading config.";
      statusEl.style.color = "red";
    }
  });

  form.addEventListener("submit", async (event: SubmitEvent) => {
    event.preventDefault();

    const newConfig: Config = {
      api_url: apiUrlInput.value,
      interval_secs: BigInt(parseInt(intervalInput.value, 60)),
    };

    try {
      await invoke("update_config", { newConfig });
      statusEl.textContent = "Settings saved successfully!";
      statusEl.style.color = "green";
    } catch (e) {
      console.error("Error saving config:", e);
      statusEl.textContent = "Failed to save settings.";
      statusEl.style.color = "red";
    }

    setTimeout(() => {
      if (statusEl) {
        statusEl.textContent = "";
      }
    }, 3000);
  });
} else {
  console.error(
    "Initialization failed: One or more required HTML elements were not found.",
  );
}
