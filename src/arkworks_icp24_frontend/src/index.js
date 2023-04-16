import { arkworks_icp24_backend } from "../../declarations/arkworks_icp24_backend";
import init, { create_proof } from "./rust_zkp/pkg/rust_zkp.js";

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

  // Interact with the backend canister, calling the verify_groth16 method with the generated proof
  const greeting = await arkworks_icp24_backend.verify_groth16(proof);

  button.removeAttribute("disabled");

  document.getElementById("greeting").innerText = greeting;

  return false;
});
