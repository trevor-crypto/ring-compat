//! Elliptic Curve Digital Signature Algorithm
//!
//! <https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm>

pub mod p256;
pub mod p384;

mod signing_key;
mod verify_key;

pub use self::{signing_key::SigningKey, verify_key::VerifyKey};
pub use ::ecdsa::{asn1, elliptic_curve::weierstrass::Curve, Signature};

use ring::signature::{EcdsaSigningAlgorithm, EcdsaVerificationAlgorithm};

/// Trait for associating a *ring* [`EcdsaSigningAlgorithm`] with an
/// elliptic curve
pub trait CurveAlg: Curve {
    /// *ring* signing algorithm
    fn signing_alg() -> &'static EcdsaSigningAlgorithm;

    /// *ring* verify algorithm
    fn verify_alg() -> &'static EcdsaVerificationAlgorithm;
}
