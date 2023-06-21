const { invoke } = window.__TAURI__.tauri;

let msInterval;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("cps", { name: msInterval.value });
}

window.addEventListener("DOMContentLoaded", () => {
  msInterval = document.querySelector("#ms-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});

