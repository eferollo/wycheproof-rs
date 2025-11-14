//! Composite-MLDSA-Sign tests

use super::*;

define_test_set!("Composite-MLDSA Sign", "composite_mldsa_sign_schema.json");

define_test_set_names!(
    MlDsa65Ed25519 => "mldsa65ed25519_sign",
);

define_algorithm_map!(
    "MLDSA65-Ed25519-SHA512" => MlDsa65Ed25519,
);

define_test_flags!(ValidSignature,);

define_test_group_type_id!(
    "MlDsa65Ed25519Sign" => MlDsa65Ed25519Sign,
);

define_test_group!(
    "privateKey" => privkey: ByteString,
    "source" => source: Source,
);

define_test!(msg: ByteString, sig: ByteString);
