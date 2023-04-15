use ic_cdk::*;
use ic_cdk_macros::*;

use ark_bn254::{Bn254, Fr};
use ark_ff::Field;
use ark_groth16::{prepare_verifying_key, verify_proof};
use ark_serialize::CanonicalDeserialize;

use once_cell::sync::Lazy;

static VK_BYTES: &[u8] = include_bytes!("vk.bin");
// static PUBLIC_INPUT: &[Fr] = &[Fr::from(6)];
static PUBLIC_INPUT: Lazy<Vec<Fr>> = Lazy::new(|| vec![Fr::from(6)]);


#[update]
fn verify_groth16(proof: Vec<u8>) -> Result<bool, String> {
    let vk = match ark_groth16::VerifyingKey::<Bn254>::deserialize(&mut &VK_BYTES[..]) {
        Ok(vk) => vk,
        Err(e) => return Err(format!("Failed to deserialize vk: {:?}", e)),
    };

    let proof = match ark_groth16::Proof::<Bn254>::deserialize(&mut &proof[..]) {
        Ok(proof) => proof,
        Err(e) => return Err(format!("Failed to deserialize proof: {:?}", e)),
    };

    // match verify_proof(&prepare_verifying_key(&vk), &proof, PUBLIC_INPUT) {
    match verify_proof(&prepare_verifying_key(&vk), &proof, &*PUBLIC_INPUT) {
        Ok(result) => Ok(result),
        Err(e) => Err(format!("Failed to verify proof: {:?}", e)),
    }
}
