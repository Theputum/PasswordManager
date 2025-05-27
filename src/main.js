const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;

let keyInputEl;
let keyMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function key() {
  keyMsgEl.textContent = await invoke("key", { key: keyInputEl.value})
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
},

window.addEventListener("DOMContentLoaded", () => {
  keyInputEl = document.querySelector("#key-input");
  keyMsgEl = document.querySelector("#key-msg");
  document.querySelector("#key-form").addEventListener("submit", (e) => {
    e.preventDefault();
    key();
  });
}));
