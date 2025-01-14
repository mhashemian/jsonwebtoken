#![no_main]
use libfuzzer_sys::fuzz_target;
// use jsonwebtoken::{
//     crypto::{sign, verify},
//     decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
// };
use jsonwebtoken::{decode_header};
use std::{str, panic};



fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    // let privkey = include_bytes!("private_ecdsa_key.pk8");
    // let pubkey = include_bytes!("public_ecdsa_key.pk8");

    // let encrypted =
    //     sign(data, &EncodingKey::from_ec_der(privkey), Algorithm::ES256).unwrap();
    // verify(&encrypted, data, &DecodingKey::from_ec_der(pubkey), Algorithm::ES256)
    //     .unwrap();
    let token = str::from_utf8(data).unwrap();

    panic::catch_unwind(|| {
        decode_header(token);
    });
});
