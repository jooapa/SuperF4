// const { invoke } = window.__TAURI__.tauri;

let exeName;
let greetMsgEl;

// async function greet() {
//   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   greetMsgEl.textContent = await invoke("cps", { name: exeName.value });
// }

window.addEventListener("DOMContentLoaded", () => {
  exeName = document.querySelector("#ms-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    // greet();

    var regex = /\.exe$/;
    var endsWithExe = regex.test(exeName);

    // Check the result
    if (endsWithExe) {
      var divElement = document.createElement("div");
      divElement.className = "blacklistDiv";
      divElement.textContent = exeName.value;
      var containerElement = document.querySelector(".container");
      containerElement.appendChild(divElement);
    } else {
      document.querySelector("#errorId").textContent = "Executable name must end with .exe";
    }    
  });
});


