use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use console_error_panic_hook;

use ark_bn254::{Bn254, Fr};
use ark_groth16::{generate_random_parameters, prepare_verifying_key, create_random_proof, verify_proof, Proof};
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError, Variable},
};
use ark_std::rand::{rngs::StdRng, SeedableRng};
use ark_serialize::CanonicalSerialize;

// 追加
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    Ok(())
}

pub struct Circuit {
    pub a: Option<Fr>,
    pub b: Option<Fr>,
    pub c: Option<Fr>,
}

impl ConstraintSynthesizer<Fr> for Circuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        let a = cs.new_witness_variable(|| {
            self.a
                .ok_or(SynthesisError::AssignmentMissing)
        })?;
        let b = cs.new_witness_variable(|| {
            self.b
                .ok_or(SynthesisError::AssignmentMissing)
        })?;
        let c = cs.new_input_variable(|| {
            self.c
                .ok_or(SynthesisError::AssignmentMissing)
        })?;

        // 変更: 掛け算を行う制約を追加
        cs.enforce_constraint(lc!() + a, lc!() + b, lc!() + c)?;

        Ok(())
    }
}


#[wasm_bindgen]
pub fn create_proof(a: u32, b: u32) -> Result<Vec<u8>, JsValue> {
    let rng = &mut StdRng::seed_from_u64(0u64);

    let pk = {
        let c = Circuit {
            a: None,
            b: None,
            c: None,
        };
        generate_random_parameters::<Bn254, _, _>(c, rng).unwrap()
    };

    let assignment = Circuit {
        a: Some(Fr::from(a)),
        b: Some(Fr::from(b)),
        c: Some(Fr::from(a*b)),
    };

    let public_input = assignment.c.clone().ok_or_else(|| JsValue::from_str("Failed to get public input"))?; // 変更
    
    let public_inputs = &[public_input]; // 変更
    web_sys::console::log_1(&JsValue::from_str(&format!("Public inputs: {:?}", public_inputs)));

    let proof = create_random_proof(assignment, &pk, rng)
    .map_err(|e| JsValue::from_str(&format!("Failed to create random proof: {:?}", e)))?; // 変更

    let mut proof_vec = Vec::new();
    proof.serialize(&mut proof_vec).unwrap();

    Ok(proof_vec)
}
