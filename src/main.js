const { invoke } = window.__TAURI__.core;

let keyInputEl;
let keyListEl;

async function key() {
  const entries = await invoke("key", { key: keyInputEl.value });

  // Clear the list and re-render it with all entries
  keyListEl.innerHTML = "";
  entries.forEach((entry) => {
    const li = document.createElement("li");
    li.textContent = entry;
    keyListEl.appendChild(li);
  });

  // Optionally clear input after submission
  keyInputEl.value = "";
}

window.addEventListener("DOMContentLoaded", () => {
  keyInputEl = document.querySelector("#key-input");
  keyListEl = document.querySelector("#key-list");
  document.querySelector("#key-form").addEventListener("submit", (e) => {
    e.preventDefault();
    key();
  });
});
