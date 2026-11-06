//! Cross-Application Interoperability Test Suite (CLI <-> Ombracrypt Desktop v2)
//!
//! Verifies bidirectional cryptographic format compatibility between Ombracrypt CLI
//! and the upstream Desktop GUI. CLI-to-Desktop compatibility is verified reciprocally
//! by the desktop repository consuming CLI-generated fixtures, while this test suite
//! ingests Desktop-generated fixtures from `tests/fixtures/desktop_v2/`.

use ombracrypt::pipeline::{decrypt_vault_core, encrypt_vault_core};
use std::fs;
use std::path::PathBuf;

fn fixture_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/desktop_v2")
        .join(filename)
}

#[test]
#[ignore = "requires desktop fixture artifact: desktop_xwing_xchacha.obv / .obk"]
fn test_desktop_v2_xwing_xchacha_decrypted_by_cli() {
    let vault_file = fixture_path("desktop_xwing_xchacha.obv");
    let key_file = fixture_path("desktop_xwing_xchacha.obk");
    let extract_dir = std::env::temp_dir().join("ombracrypt_interop_xwing_xchacha");

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
        "CLI must decrypt Desktop-generated X-Wing + XChaCha20 vault: {:?}",
        res
    );
    let extracted_secret = extract_dir.join("desktop_test/secret.txt");
    assert!(
        extracted_secret.exists(),
        "Extracted payload must be present"
    );
    assert_eq!(
        fs::read_to_string(extracted_secret).unwrap(),
        "Ombracrypt Desktop v2 Canonical Payload 2026\n"
    );
}

#[test]
#[ignore = "requires desktop fixture artifact: desktop_xwing_aes256.obv / .obk"]
fn test_desktop_v2_xwing_aes256_decrypted_by_cli() {
    let vault_file = fixture_path("desktop_xwing_aes256.obv");
    let key_file = fixture_path("desktop_xwing_aes256.obk");
    let extract_dir = std::env::temp_dir().join("ombracrypt_interop_xwing_aes256");

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
        "CLI must decrypt Desktop-generated X-Wing + AES-256-GCM vault: {:?}",
        res
    );
    let extracted_secret = extract_dir.join("desktop_test/secret.txt");
    assert!(extracted_secret.exists());
}

#[test]
#[ignore = "requires desktop fixture artifact: desktop_cypherpunk_xchacha.obv / .obk"]
fn test_desktop_v2_cypherpunk_xchacha_decrypted_by_cli() {
    let vault_file = fixture_path("desktop_cypherpunk_xchacha.obv");
    let key_file = fixture_path("desktop_cypherpunk_xchacha.obk");
    let extract_dir = std::env::temp_dir().join("ombracrypt_interop_cypherpunk_xchacha");

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
        "CLI must decrypt Desktop-generated Cypherpunk + XChaCha20 vault: {:?}",
        res
    );
    let extracted_secret = extract_dir.join("desktop_test/secret.txt");
    assert!(extracted_secret.exists());
}

#[test]
#[ignore = "requires desktop fixture artifact: desktop_cypherpunk_aes256.obv / .obk"]
fn test_desktop_v2_cypherpunk_aes256_decrypted_by_cli() {
    let vault_file = fixture_path("desktop_cypherpunk_aes256.obv");
    let key_file = fixture_path("desktop_cypherpunk_aes256.obk");
    let extract_dir = std::env::temp_dir().join("ombracrypt_interop_cypherpunk_aes256");

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
        "CLI must decrypt Desktop-generated Cypherpunk + AES-256-GCM vault: {:?}",
        res
    );
    let extracted_secret = extract_dir.join("desktop_test/secret.txt");
    assert!(extracted_secret.exists());
}

#[test]
#[ignore = "requires desktop fixture artifact: desktop_recipient.obp"]
fn test_desktop_generated_obp_sealed_by_cli() {
    let pubkey_file = fixture_path("desktop_recipient.obp");
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
        true,
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