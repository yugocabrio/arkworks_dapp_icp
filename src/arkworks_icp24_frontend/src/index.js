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

  // Generate proof
  const proof = create_proof(a, b);

  // Generate JSON proof
  const json_proof = create_json_proof(proof);
  console.log("Proof:", proof);
  console.log("JSON Proof:", json_proof);

  try {
    // Interact with the backend canister, calling the verify_groth16 method with the generated proof
    const verificationResult = await arkworks_icp24_backend.verify_groth16(proof);
    console.log("Verification Result:", verificationResult);

    // Display the JSON proof
    document.getElementById("json-proof").innerText = JSON.stringify(JSON.parse(json_proof), null, 2);

    // Display the verification result
    document.getElementById("verification-result").innerText = verificationResult === "Proof is valid" ? "Proof is valid!" : "Proof is invalid!";
  } catch (error) {
    document.getElementById("verification-result").innerText = "Error: " + error;
  }

  button.removeAttribute("disabled");

  return false;
});
