const { invoke } = window.__TAURI__.core;

let keyInputEl;
let keyMsgEl;

async function key() {
  keyMsgEl.textContent = await invoke("key", { key: keyInputEl.value})
}

window.addEventListener("DOMContentLoaded", () => {
  keyInputEl = document.querySelector("#key-input");
  keyMsgEl = document.querySelector("#key-msg");
  document.querySelector("#key-form").addEventListener("submit", (e) => {
    e.preventDefault();
    key();
  });
});
