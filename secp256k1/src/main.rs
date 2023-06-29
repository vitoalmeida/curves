use curv::{
    arithmetic::{traits::Integer, Converter, Modulo, Samplable},
    BigInt,
};
use secp256k1::*;

fn main() {
    // let elliptic_curve = EllipticCurve::new(
    //     BigInt::from(0),
    //     BigInt::from(7),
    //     BigInt::from_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F")
    //         .unwrap(),
    //     BigInt::from_hex("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798")
    //         .unwrap(),
    //     BigInt::from_hex("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8")
    //         .unwrap(),
    //     BigInt::from_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141")
    //         .unwrap(),
    // );
    let elliptic_curve = EllipticCurve::new(2, 2, 17, 5, 1, 19);

    println!("Curve details: {:?}", elliptic_curve);
    println!("a: {:?}", elliptic_curve.a);
}
// let private_key = generate_private_key(&gen_order);
// let public_key = generate_public_key(&private_key, &elliptic_curve);

// println!("Private Key: {}", private_key);
// println!("Public Key: {:?}", public_key);

// let message = "Hello, world!";
// let (r, s) = generate_signature(message, &private_key, &elliptic_curve);

// println!("Message: {}", message);
// println!("Signature (r, s): ({}, {})", r, s);
