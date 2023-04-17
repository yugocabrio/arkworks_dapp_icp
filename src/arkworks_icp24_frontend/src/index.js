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
  resultElement.style.display = "none"; // 追加: submitボタンが押された時にResultを消す

  // Generate proof
  const proof = create_proof(a, b);

  // Generate JSON proof
  const json_proof = create_json_proof(proof);

  // Display the JSON proof
  const jsonProofElement = document.getElementById("json-proof");
  jsonProofElement.innerText = JSON.stringify(JSON.parse(json_proof), null, 2);
  jsonProofElement.style.display = "block"; // 追加: 証明が生成された時に表示を切り替える
  

  try {
    // Interact with the backend canister, calling the verify_groth16 method with the generated proof
    const verificationResult = await arkworks_icp24_backend.verify_groth16(proof);

    // Display the verification result
    resultElement.innerText = verificationResult === "Proof is valid" ? "Proof is True!" : "Proof is False";
    resultElement.style.display = "inline"; // 結果が得られた時に表示を切り替える
  } catch (error) {
    resultElement.innerText = "Error: " + error;
    resultElement.style.display = "inline"; // エラーが発生した時に表示を切り替える
  }

  button.removeAttribute("disabled");

  return false;
});

// Add reset button event listener
document.getElementById("reset-button").addEventListener("click", () => {
  document.getElementById("number1").value = "";
  document.getElementById("number2").value = "";
  document.getElementById("verification-result").innerText = "";
  const jsonProofElement = document.getElementById("json-proof");
  jsonProofElement.innerText = ""; // 追加: JSON証明をリセットする
  jsonProofElement.style.display = "none"; // 追加: JSON証明を非表示にする
});

