use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use console_error_panic_hook;

use serde_json::json;

use ark_bn254::{Bn254, Fr};
use ark_groth16::{create_random_proof, verify_proof, Proof, ProvingKey, VerifyingKey};
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError, Variable},
};
use ark_std::rand::{rngs::StdRng, SeedableRng};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_ff::fields::PrimeField;


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

        cs.enforce_constraint(lc!() + a, lc!() + b, lc!() + c)?;

        Ok(())
    }
}

// ベタ張りされたProving Keyを定義します。
// これはダミーのデータです。実際のProving Keyデータに置き換えてください。
const PK_BYTES: &[u8] = &[229, 115, 20, 235, 13, 45, 74, 206, 247, 160, 181, 99, 6, 164, 172, 29, 201, 155, 154, 29, 193, 90, 52, 220, 84, 154, 5, 33, 113, 189, 152, 27, 30, 101, 51, 175, 146, 195, 131, 190, 86, 179, 159, 5, 32, 240, 195, 245, 113, 63, 64, 76, 197, 5, 244, 136, 122, 136, 34, 79, 191, 73, 86, 45, 162, 249, 72, 18, 158, 8, 13, 54, 117, 149, 252, 194, 248, 166, 190, 238, 44, 8, 143, 78, 119, 253, 253, 185, 237, 170, 235, 75, 64, 125, 56, 22, 23, 228, 45, 237, 146, 66, 54, 203, 183, 168, 43, 167, 75, 154, 227, 25, 138, 238, 246, 51, 226, 144, 169, 147, 30, 211, 150, 166, 230, 16, 157, 26, 187, 134, 21, 246, 227, 223, 255, 104, 237, 43, 227, 44, 228, 3, 91, 18, 232, 87, 146, 121, 92, 47, 135, 188, 13, 41, 153, 207, 157, 42, 129, 34, 71, 149, 231, 183, 44, 123, 229, 149, 84, 206, 17, 87, 172, 45, 180, 157, 39, 87, 232, 27, 174, 151, 80, 90, 13, 129, 204, 190, 136, 190, 177, 47, 62, 46, 44, 22, 212, 44, 200, 33, 166, 38, 99, 166, 205, 89, 220, 250, 98, 27, 154, 167, 71, 45, 245, 74, 111, 19, 80, 12, 29, 234, 97, 30, 2, 0, 0, 0, 0, 0, 0, 0, 178, 111, 145, 194, 234, 209, 3, 170, 178, 55, 31, 18, 79, 214, 223, 214, 130, 152, 232, 197, 206, 146, 73, 13, 73, 21, 202, 17, 37, 199, 13, 3, 242, 211, 246, 113, 151, 162, 72, 254, 198, 45, 39, 219, 253, 179, 136, 218, 57, 140, 30, 239, 186, 2, 168, 243, 199, 230, 149, 230, 138, 203, 178, 9, 96, 249, 0, 79, 53, 145, 16, 252, 158, 245, 126, 133, 145, 188, 55, 168, 176, 169, 235, 85, 57, 55, 41, 147, 89, 238, 9, 133, 186, 114, 48, 155, 35, 202, 89, 75, 106, 125, 171, 138, 121, 54, 155, 244, 95, 244, 35, 196, 70, 210, 63, 200, 45, 209, 190, 30, 121, 25, 207, 151, 73, 221, 184, 157, 4, 0, 0, 0, 0, 0, 0, 0, 29, 147, 62, 67, 243, 11, 23, 181, 104, 213, 227, 65, 101, 29, 193, 60, 105, 243, 187, 251, 255, 249, 81, 58, 66, 181, 162, 214, 149, 132, 50, 19, 208, 33, 227, 69, 67, 30, 203, 13, 216, 17, 174, 191, 3, 203, 87, 193, 60, 43, 109, 213, 143, 252, 197, 38, 228, 67, 250, 132, 26, 216, 20, 163, 205, 20, 118, 189, 21, 205, 30, 133, 185, 178, 67, 65, 109, 121, 214, 67, 101, 207, 25, 38, 181, 106, 242, 147, 28, 170, 171, 11, 186, 236, 135, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 205, 20, 118, 189, 21, 205, 30, 133, 185, 178, 67, 65, 109, 121, 214, 67, 101, 207, 25, 38, 181, 106, 242, 147, 28, 170, 171, 11, 186, 236, 135, 15, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 27, 162, 204, 191, 111, 93, 165, 47, 253, 174, 89, 205, 246, 95, 106, 165, 187, 50, 210, 254, 73, 214, 82, 212, 33, 53, 229, 76, 27, 240, 124, 7, 5, 105, 94, 166, 65, 230, 138, 60, 254, 205, 70, 220, 208, 10, 194, 185, 253, 251, 16, 105, 0, 239, 119, 8, 142, 204, 165, 193, 252, 248, 138, 17, 3, 0, 0, 0, 0, 0, 0, 0, 2, 99, 228, 150, 92, 125, 34, 34, 132, 24, 0, 34, 115, 35, 114, 103, 225, 226, 89, 174, 95, 55, 27, 78, 244, 18, 179, 231, 67, 76, 210, 21, 45, 52, 202, 181, 232, 43, 235, 15, 51, 179, 179, 170, 75, 233, 194, 114, 34, 64, 29, 234, 233, 84, 195, 85, 96, 116, 151, 178, 49, 32, 226, 34, 74, 204, 164, 184, 214, 229, 139, 46, 20, 240, 31, 106, 143, 35, 238, 226, 162, 94, 4, 16, 79, 82, 77, 196, 21, 108, 97, 11, 99, 101, 116, 25, 2, 0, 0, 0, 0, 0, 0, 0, 31, 203, 97, 93, 9, 168, 247, 16, 147, 77, 158, 123, 174, 37, 23, 222, 246, 147, 75, 136, 99, 216, 180, 92, 66, 196, 29, 37, 215, 239, 238, 128, 86, 155, 59, 233, 89, 132, 198, 94, 18, 39, 206, 223, 231, 81, 79, 196, 8, 156, 180, 105, 227, 218, 49, 42, 216, 238, 197, 156, 36, 176, 255, 174];

#[wasm_bindgen]
pub fn create_proof(a: u32, b: u32) -> Result<Vec<u8>, JsValue> {
    let rng = &mut StdRng::seed_from_u64(0u64);
    
    let pk: ProvingKey<Bn254> = CanonicalDeserialize::deserialize(&mut &PK_BYTES[..]).map_err(|_| JsValue::from_str("Failed to deserialize ProvingKey"))?;

    let assignment = Circuit {
        a: Some(Fr::from(a)),
        b: Some(Fr::from(b)),
        c: Some(Fr::from(a * b)),
    };

    let public_input = assignment.c.clone().ok_or_else(|| JsValue::from_str("Failed to get public input"))?;
    
    let public_inputs = &[public_input];
    web_sys::console::log_1(&JsValue::from_str(&format!("Public inputs: {:?}", public_inputs)));

    let proof = create_random_proof(assignment, &pk, rng)
        .map_err(|e| JsValue::from_str(&format!("Failed to create random proof: {:?}", e)))?;

    let mut proof_vec = Vec::new();
    proof.serialize(&mut proof_vec).unwrap();

    Ok(proof_vec)
}

#[wasm_bindgen]
pub fn create_json_proof(proof_vec: Vec<u8>) -> Result<String, JsValue> {
    let proof = Proof::<Bn254>::deserialize(&proof_vec[..])
        .map_err(|e| JsValue::from_str(&format!("Failed to deserialize proof: {:?}", e)))?;

    let a: Vec<String> = vec![
        format!("{:?}", proof.a.x.into_repr()),
        format!("{:?}", proof.a.y.into_repr()),
    ];
    let b: Vec<Vec<String>> = vec![
        vec![
            format!("{:?}", proof.b.x.c0.into_repr()),
            format!("{:?}", proof.b.x.c1.into_repr()),
        ],
        vec![
            format!("{:?}", proof.b.y.c0.into_repr()),
            format!("{:?}", proof.b.y.c1.into_repr()),
        ],
    ];
    let c: Vec<String> = vec![
        format!("{:?}", proof.c.x.into_repr()),
        format!("{:?}", proof.c.y.into_repr()),
    ];

    let proof_json = json!({
        "curve": "bn254",
        "protocol": "groth16",
        "a": {
            "infinity": proof.a.infinity,
            "x": a[0],
            "y": a[1]
        },
        "b": {
            "infinity": proof.b.infinity,
            "x": {
                "c0": b[0][0],
                "c1": b[0][1]
            },
            "y": {
                "c0": b[1][0],
                "c1": b[1][1]
            }
        },
        "c": {
            "infinity": proof.c.infinity,
            "x": c[0],
            "y": c[1]
        }
    });

    Ok(serde_json::to_string(&proof_json).unwrap())
}
