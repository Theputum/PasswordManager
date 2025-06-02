const { invoke } = window.__TAURI__.core;

let keyInputEl;
let keyListEl;

async function key() {
  const entries = await invoke("key", { key: keyInputEl.value });

  renderList(entries);
  keyInputEl.value = "";
}

async function loadEntries() {
  const entries = await invoke("load_entries");
  renderList(entries);
}

function renderList(entries) {
  keyListEl.innerHTML = "";
  entries.forEach((entry) => {
    const li = document.createElement("li");
    li.textContent = entry;
    keyListEl.appendChild(li);
  });
}

window.addEventListener("DOMContentLoaded", () => {
  keyInputEl = document.querySelector("#key-input");
  keyListEl = document.querySelector("#key-list");
  document.querySelector("#key-form").addEventListener("submit", (e) => {
    e.preventDefault();
    key();
  });
  loadEntries();
});
