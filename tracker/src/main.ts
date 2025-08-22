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

const startButton = document.getElementById("startButton") as HTMLButtonElement;
const stopButton = document.getElementById("stopButton") as HTMLButtonElement;
const trackingStatus = document.getElementById(
  "trackingStatus",
) as HTMLSpanElement;

function setTrackingState(isRunning: boolean) {
  if (isRunning) {
    trackingStatus.textContent = "Running";
    trackingStatus.style.color = "green";
    startButton.disabled = true;
    stopButton.disabled = false;
  } else {
    trackingStatus.textContent = "Stopped";
    trackingStatus.style.color = "red";
    startButton.disabled = false;
    stopButton.disabled = true;
  }
}

if (form && apiUrlInput && intervalInput && statusEl) {
  startButton.addEventListener("click", async () => {
    await invoke("start_tracking");
    setTrackingState(true);
  });

  stopButton.addEventListener("click", async () => {
    await invoke("stop_tracking");
    setTrackingState(false);
  });

  globalThis.addEventListener("DOMContentLoaded", async () => {
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
      // TODO: rust expects a u64 or BigInt but it doesn't know how to serialize it.
      // could be an issue with ts-rs, so may need to check it out
      interval_secs: parseInt(intervalInput.value, 10) as unknown as bigint,
    };

    try {
      await invoke("update_config", { newConfig });
      store.set("config", newConfig);
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
