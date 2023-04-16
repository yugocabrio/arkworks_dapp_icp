use ic_cdk::*;
use ic_cdk_macros::*;

use ark_bn254::{Bn254, Fr};
use ark_ff::Field;
use ark_groth16::{prepare_verifying_key, verify_proof};
use ark_serialize::CanonicalDeserialize;

use once_cell::sync::Lazy;

// Replace the content inside vec![] with the content of your vk.bin
static VK_BYTES: Lazy<Vec<u8>> = Lazy::new(|| vec![
    229, 115, 20, 235, 13, 45, 74, 206, 247, 160, 181, 99, 6, 164, 172, 29, 201, 155, 154, 29, 193, 90, 52, 220, 84, 154, 5, 33, 113, 189, 152, 27, 30, 101, 51, 175, 146, 195, 131, 190, 86, 179, 159, 5, 32, 240, 195, 245, 113, 63, 64, 76, 197, 5, 244, 136, 122, 136, 34, 79, 191, 73, 86, 45, 162, 249, 72, 18, 158, 8, 13, 54, 117, 149, 252, 194, 248, 166, 190, 238, 44, 8, 143, 78, 119, 253, 253, 185, 237, 170, 235, 75, 64, 125, 56, 22, 23, 228, 45, 237, 146, 66, 54, 203, 183, 168, 43, 167, 75, 154, 227, 25, 138, 238, 246, 51, 226, 144, 169, 147, 30, 211, 150, 166, 230, 16, 157, 26, 187, 134, 21, 246, 227, 223, 255, 104, 237, 43, 227, 44, 228, 3, 91, 18, 232, 87, 146, 121, 92, 47, 135, 188, 13, 41, 153, 207, 157, 42, 129, 34, 71, 149, 231, 183, 44, 123, 229, 149, 84, 206, 17, 87, 172, 45, 180, 157, 39, 87, 232, 27, 174, 151, 80, 90, 13, 129, 204, 190, 136, 190, 177, 47, 62, 46, 44, 22, 212, 44, 200, 33, 166, 38, 99, 166, 205, 89, 220, 250, 98, 27, 154, 167, 71, 45, 245, 74, 111, 19, 80, 12, 29, 234, 97, 30, 2, 0, 0, 0, 0, 0, 0, 0, 142, 35, 79, 204, 247, 169, 227, 22, 14, 72, 215, 179, 133, 248, 68, 113, 21, 238, 92, 205, 249, 24, 213, 211, 166, 106, 106, 15, 86, 162, 23, 170, 242, 211, 246, 113, 151, 162, 72, 254, 198, 45, 39, 219, 253, 179, 136, 218, 57, 140, 30, 239, 186, 2, 168, 243, 199, 230, 149, 230, 138, 203, 178, 9
]);

static PUBLIC_INPUT: Lazy<Vec<Fr>> = Lazy::new(|| vec![Fr::from(6)]);

#[update]
fn verify_groth16(proof: Vec<u8>) -> String {
    let vk = match ark_groth16::VerifyingKey::<Bn254>::deserialize(&mut &VK_BYTES[..]) {
        Ok(vk) => vk,
        Err(e) => {
            return format!("Failed to deserialize vk: {:?}", e);
        }
    };

    let proof = match ark_groth16::Proof::<Bn254>::deserialize(&mut &proof[..]) {
        Ok(proof) => proof,
        Err(e) => {
            return format!("Failed to deserialize proof: {:?}", e);
        }
    };

    match verify_proof(&prepare_verifying_key(&vk), &proof, &*PUBLIC_INPUT) {
        Ok(result) => {
            if result {
                "Proof is valid".to_string()
            } else {
                "Proof is invalid".to_string()
            }
        }
        Err(e) => {
            format!("Failed to verify proof: {:?}", e)
        }
    }
}



// 以下テスト関数です。
// serialize化されたProofとvkを使って、verificationをすることが確認されました。
static PROOF_BYTES: Lazy<Vec<u8>> = Lazy::new(|| vec![
    82, 16, 85, 60, 130, 236, 10, 208, 97, 131, 248, 163, 138, 147, 220, 101, 125, 233, 90, 64, 89, 67, 211, 240, 220, 125, 97, 233, 111, 180, 74, 159, 233, 45, 92, 5, 165, 104, 187, 86, 50, 183, 108, 210, 90, 164, 13, 247, 39, 132, 207, 46, 10, 102, 98, 106, 38, 138, 242, 130, 222, 139, 86, 39, 232, 147, 7, 45, 109, 97, 139, 54, 218, 88, 163, 19, 47, 26, 31, 105, 243, 36, 235, 117, 34, 16, 209, 25, 63, 106, 67, 213, 60, 103, 128, 16, 5, 106, 139, 14, 10, 246, 192, 253, 104, 20, 196, 70, 158, 173, 209, 38, 77, 149, 199, 59, 57, 236, 223, 247, 99, 91, 201, 177, 166, 122, 247, 44
]);

#[ic_cdk_macros::query]
fn test_verify_groth16() -> String {
    let vk = match ark_groth16::VerifyingKey::<Bn254>::deserialize(&mut &VK_BYTES[..]) {
        Ok(vk) => vk,
        Err(e) => {
            return format!("Failed to deserialize vk: {:?}", e);
        }
    };

    let proof = match ark_groth16::Proof::<Bn254>::deserialize(&mut &PROOF_BYTES[..]) {
        Ok(proof) => proof,
        Err(e) => {
            return format!("Failed to deserialize proof: {:?}", e);
        }
    };

    match verify_proof(&prepare_verifying_key(&vk), &proof, &*PUBLIC_INPUT) {
        Ok(result) => {
            if result {
                "Proof is valid".to_string()
            } else {
                "Proof is invalid".to_string()
            }
        }
        Err(e) => {
            format!("Failed to verify proof: {:?}", e)
        }
    }
}