const { invoke } = window.__TAURI__.tauri;

let exeName;
var regex = /\.exe$/;

window.addEventListener("DOMContentLoaded", () => {
  exeName = document.querySelector("#ms-input");
  document.querySelector("#add-form").addEventListener("submit", (e) => {
    e.preventDefault();

    //check if exeName already exists
    let element = document.getElementById(exeName.value);
    if (element !== null) {
      document.querySelector("#errorId").textContent =
        "Executable is already in the list";
      return;
    }

    //check if exeName ends with .exe
    var endsWithExe = regex.test(exeName.value);

    // Check the result
    if (endsWithExe) {
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
}

function removeExeDiv(element) {
  var divName = element.id;
  console.log(divName);
  //delete div with id = divName
  var divElement = document.getElementById(divName);
  divElement.parentNode.removeChild(divElement);
}
