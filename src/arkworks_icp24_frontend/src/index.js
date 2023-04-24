import init, { create_proof, create_json_proof } from "./rust_zkp/pkg/rust_zkp.js";
import { arkworks_icp24_backend } from "../../declarations/arkworks_icp24_backend";

(async () => {
  await init();
})();

document.querySelector("form").addEventListener("submit", async (e) => {
  e.preventDefault();
  const button = e.target.querySelector("button");

  const a = parseInt(document.getElementById("number1").value);
  const b = parseInt(document.getElementById("number2").value);

  button.setAttribute("disabled", true);

  const resultElement = document.getElementById("verification-result");
  resultElement.style.display = "none"; 

  const proof = create_proof(a, b);

  const json_proof = create_json_proof(proof);

  const jsonProofElement = document.getElementById("json-proof");
  jsonProofElement.innerText = JSON.stringify(JSON.parse(json_proof), null, 2);
  jsonProofElement.style.display = "block"; 
  
  try {
    const verificationResult = await arkworks_icp24_backend.verify_groth16(proof);

    resultElement.innerText = verificationResult === "Proof is valid" ? "Proof is True!" : "Proof is False";
    resultElement.style.display = "inline"; 
  } catch (error) {
    resultElement.innerText = "Error: " + error;
    resultElement.style.display = "inline"; 
  }

  button.removeAttribute("disabled");

  return false;
});

document.getElementById("reset-button").addEventListener("click", (e) => {
  e.preventDefault(); 
  document.getElementById("number1").value = "";
  document.getElementById("number2").value = "";
  const resultElement = document.getElementById("verification-result"); 
  resultElement.innerText = ""; 
  resultElement.style.display = "none"; 
  const jsonProofElement = document.getElementById("json-proof");
  jsonProofElement.innerText = "";
  jsonProofElement.style.display = "none";
});


