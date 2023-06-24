const { invoke } = window.__TAURI__.tauri;

let exeName;
var regex = /[A-Za-z0-9]+\.exe/;

window.addEventListener("DOMContentLoaded", () => {
  exeName = document.querySelector("#ms-input");
  document.querySelector("#add-form").addEventListener("submit", (e) => {
    e.preventDefault();
    
    //check if exeName ends with .exe
    var endsWithExe = regex.test(exeName.value);
    //check if exeName already exists
    let element = document.getElementById(exeName.value);
    if (element !== null) {
      document.querySelector("#errorId").textContent =
        "Executable is already in the list";
    } else if (endsWithExe) {
      document.querySelector("#errorId").textContent = "";
      addExeDiv(exeName.value);
    } else {
      document.querySelector("#errorId").textContent =
        "Executable name must end with .exe";
    }

  });
});

function addExeDiv(paraexe) {
      var divElement = document.createElement("div");
      divElement.className = "blacklistDiv";
      divElement.id = paraexe;
      divElement.textContent = paraexe;
      divElement.onclick = function () {
        removeExeDiv(this);
      };
      var containerElement = document.querySelector(".container");
      containerElement.appendChild(divElement);
      invoke("add_exe_to_json", { name: paraexe });
}

function removeExeDiv(element) {
  var divName = element.id;
  console.log(divName);
  //delete div with id = divName
  var divElement = document.getElementById(divName);
  divElement.parentNode.removeChild(divElement);
  invoke("remove_exe_from_json", { name: divName });
}

