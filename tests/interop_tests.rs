//! Cross-Application Interoperability Test Suite (CLI <-> Ombracrypt Desktop v2)
//!
//! Verifies bidirectional cryptographic format compatibility between Ombracrypt CLI
//! and the upstream Desktop GUI. CLI-to-Desktop compatibility is verified reciprocally
//! by the desktop repository consuming CLI-generated fixtures, while this test suite
//! ingests Desktop-generated fixtures from `tests/fixtures/desktop_v2/`.

use ombracrypt::pipeline::{decrypt_vault_core, encrypt_vault_core};
use std::fs;
use std::path::PathBuf;

#[test]
#[ignore = "requires desktop fixture artifact"]
fn test_desktop_generated_vault_decrypted_by_cli() {
    let fixture_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/desktop_v2");
    let vault_file = fixture_dir.join("desktop_test.obv");
    let key_file = fixture_dir.join("desktop_test.obk");
    let extract_dir = std::env::temp_dir().join("ombracrypt_interop_extract");

    let _ = fs::remove_dir_all(&extract_dir);
    fs::create_dir_all(&extract_dir).unwrap();

    let res = decrypt_vault_core(
        None,
        vault_file.to_str().unwrap(),
        Some(key_file.to_str().unwrap()),
        extract_dir.to_str().unwrap(),
        "DesktopTestPassword2026!",
        None,
    );

    assert!(
        res.is_ok(),
        "CLI must decrypt Desktop-generated vault: {:?}",
        res
    );
    let extracted_secret = extract_dir.join("desktop_test/secret.txt");
    assert!(
        extracted_secret.exists(),
        "Extracted payload must be present"
    );
}

#[test]
#[ignore = "requires desktop fixture artifact"]
fn test_desktop_generated_obp_sealed_by_cli() {
    let fixture_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/desktop_v2");
    let pubkey_file = fixture_dir.join("desktop_recipient.obp");
    let temp_dir = std::env::temp_dir().join("ombracrypt_cli_seal_target");
    let vault_out = std::env::temp_dir().join("ombracrypt_cli_seal_vault");

    let _ = fs::remove_dir_all(&temp_dir);
    let _ = fs::remove_dir_all(&vault_out);
    fs::create_dir_all(&temp_dir).unwrap();
    fs::create_dir_all(&vault_out).unwrap();
    fs::write(temp_dir.join("payload.txt"), b"Sealed from CLI").unwrap();

    let res = encrypt_vault_core(
        None,
        temp_dir.to_str().unwrap(),
        vault_out.to_str().unwrap(),
        pubkey_file.to_str().unwrap(),
        true, // use existing public key
        "xchacha20",
        "xwing",
        "SealPin2026!",
        "",
        None,
    );

    assert!(
        res.is_ok(),
        "CLI must seal vault using Desktop-generated .obp: {:?}",
        res
    );
}
