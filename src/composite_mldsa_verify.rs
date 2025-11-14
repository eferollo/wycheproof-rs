//! Composite-MLDSA-Verify tests

use super::*;

define_test_set!(
    "Composite-MLDSA Verify",
    "composite_mldsa_verify_schema.json"
);

define_test_set_names!(
    MlDsa65Ed25519 => "mldsa65ed25519_verify",
);

define_algorithm_map!(
    "MLDSA65-Ed25519-SHA512" => MlDsa65Ed25519,
);

define_test_flags!(ValidSignature,);

define_test_group_type_id!(
    "MlDsa65Ed25519Verify" => MlDsa65Ed25519Verify,
);

define_test_group!(
    "publicKey" => pubkey: ByteString,
    "source" => source: Source,
);

define_test!(msg: ByteString, sig: ByteString);
