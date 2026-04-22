// Embedded Signature example derived from:
// https://www.ietf.org/archive/id/draft-rundgren-cbor-core-25.html#name-code-example
use cbor_core::{Value, map};
use sha2::{Sha256, Sha512, Digest};
use hmac::{Hmac, KeyInit, Mac};
use hex_literal;
use hex;

// static CSF_CONTAINER_LBL: Value = Value::SimpleValue(SimpleValue::from_u8(99).unwrap());
const CSF_ALG_LBL: Value = Value::Unsigned(1);
const CSF_SIG_LBL: Value = Value::Unsigned(6);
const CSF_CONTAINER_LBL: Value = Value::simple_value(99);

fn hmac<'a>(cose_alg: i32, key: &[u8; 32], data: &Vec<u8>) -> Vec<u8> {
    // This code looks pretty horrible but it will do for now...
    let result = match cose_alg {
        5 => {
            let mut t = Hmac::<Sha256>::new_from_slice(key)
                .expect("HMAC can take key of any size");
            t.update(data);
            t.finalize().into_bytes().to_vec()
        },
        7 => {
            let mut t = Hmac::<Sha512>::new_from_slice(key)
                .expect("HMAC can take key of any size");
            t.update(data);
            t.finalize().into_bytes().to_vec()
        },
        _ => panic! ("Unsupported COSE algorithm: {cose_alg}")
    };
    return result
}

fn main() {
    // Crypto data
    const SHARED_KEY: &[u8; 32] = 
        &hex_literal::hex!("7fdd851a3b9d2dafc5f0d00030e22b9343900cd42ede4948568a4a2ee655291a");
    const COSE_ALG: i32 = 5;

    const APP_P1_LBL: i32 = 1;                       // Application label
    const APP_P2_LBL: i32 = 2;                       //        ""

    ////////////////////////////////////
    // Create an unsigned CBOR object //
    ////////////////////////////////////
    let mut object = map!{};
    object.insert(APP_P1_LBL, "data");               // Application data
    object.insert(APP_P2_LBL, "more data");          //        ""


    ////////////////////////////////////////
    // Add a signature to the CBOR object //
    ////////////////////////////////////////
    let mut csf = map!{};                            // Create CSF container and
    csf.insert(CSF_ALG_LBL, COSE_ALG);               // add COSE algorithm to it
    object.insert(CSF_CONTAINER_LBL, csf);           // Add CSF container to object
    let sig = hmac(COSE_ALG,                         // Generate signature over
                   SHARED_KEY,                       // the current object
                   &object.encode());                // encode(): all we got so far
    object[&CSF_CONTAINER_LBL].insert(CSF_SIG_LBL, 
                                      sig);          // Add signature to CSF container
    let cbor_object = object.encode();               // Return CBOR as bytes
    println!("Diagnostic: {object:?}\nHex: {}", hex::encode(cbor_object));
}
