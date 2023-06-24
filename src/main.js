const { invoke } = window.__TAURI__.tauri;

window.__TAURI__
  .invoke("get_blacklist_name")
  .then((blacklist) => {
        
    if (blacklist == null) {
      document.querySelector("#errorId").textContent =
        "Failed to retrieve the blacklist.json file ";
    } else {
      console.log(blacklist);
      blacklistString = objToString(blacklist);
      console.log(blacklistString);
      
      for (let i = 0; i < blacklistString.length; i++) {
        addExeDiv(blacklistString[i]);
      }
  }
  })
//------------------------------------------------------------
  .catch((error) => {
    // Handle the case when the blacklist is not available
    console.error("Failed to retrieve the blacklist:", error);

    document.querySelector("#errorId").textContent =
      "Failed to retrieve the blacklist.json file :better error: "+ error;
  });

let exeName;
var regex = /[A-Za-z0-9]+\.exe/;

function objToString(obj) {
  var str = "";
  for (var p in obj) {
    if (Object.prototype.hasOwnProperty.call(obj, p)) {
      str += obj[p];
    }
  }
  str = str.split(",");
  return str;
}

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
      invoke("add_exe_to_json", { name: exeName.value });

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
  document.querySelector("#errorId").textContent = "";
  var divName = element.id;
  console.log(divName);
  //delete div with id = divName
  var divElement = document.getElementById(divName);
  divElement.parentNode.removeChild(divElement);
  invoke("remove_exe_from_json", { name: divName });
}

