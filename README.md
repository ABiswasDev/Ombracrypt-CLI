<h1 align="center">Ombracrypt CLI</h1>

<p align="center">
  <strong>The zero-trust quantum vault, built for the terminal.</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/github/v/release/ABiswasDev/Ombracrypt-CLI?style=flat-square" alt="Release">
  <img src="https://img.shields.io/github/license/ABiswasDev/Ombracrypt-CLI?style=flat-square" alt="License">
</p>

### Overview

Ombracrypt CLI brings the full power of the Ombracrypt Post-Quantum Cryptographic (PQC) engine to headless servers, CI/CD pipelines, and terminal power users. It is a standalone, pure Rust binary designed to defend digital assets against future quantum computing-based cyberattacks ("Store Now, Decrypt Later" paradigms) without relying on external infrastructure, cloud services, or graphical dependencies.

Maintains 100% cryptographic parity with the [Ombracrypt Desktop Application](https://github.com/ABiswasDev/Ombracrypt).

### Key Features
* **Zero Dependencies:** Distributed as a single, pre-compiled binary for Linux, macOS, and Windows. No runtime environments required.
* **Hybrid KEM Architecture:** Fuses classical X25519 (ECDH) with Post-Quantum ML-KEM (Kyber-768/1024) to mitigate both classical and quantum attacks.
* **Massive Payload Handling:** Strict 1MB chunked streaming architecture enables encryption of highly massive files with a near-zero, flat RAM footprint.
* **Anti-Coercion Trapdoor:** Features a Plausibly Deniable Duress Trapdoor that silently destroys the cryptographic key if an attacker forces you to surrender a secondary deception passcode.
* **Secure UI:** Implements strict, memory-safe password masking directly in the terminal to prevent shoulder-surfing and shell history leakage.
* **Memory Zeroization:** Integrates the `zeroize` crate to securely scrub all ephemeral keys, passcodes, and shared secrets from active RAM immediately upon task completion.
---

## Download & Installation

Download the latest standalone binary for your operating system from the [Releases Page](https://github.com/ABiswasDev/Ombracrypt-CLI/releases).

**Linux & macOS:**
```bash
# Extract the archive
tar -xvf ombracrypt-cli-linux-x64.tar.gz

# Make executable and move to system path for global use
chmod +x ombracrypt-cli
sudo mv ombracrypt-cli /usr/local/bin/ombracrypt
```

**Windows:**
Extract the `.zip` archive and move `ombracrypt-cli.exe` into a directory included in your system's environment `PATH`.

---

## Usage

Ombracrypt CLI uses a clean, interactive command-line interface. 

### Encrypt a Vault
To lock a file or directory into a Quantum-Safe Vault (`.obv`) and generate your physical key (`.obk`):
```bash
ombracrypt encrypt --target "/path/to/sensitive_data" --kem cypherpunk --cipher xchacha20
```
*The engine will interactively prompt you to securely enter your Master Password and an optional Deception Passcode.*

### Decrypt a Vault
To unlock an `.obv` vault using your `.obk` key file:
```bash
ombracrypt decrypt --target "/path/to/sensitive_data.obv" --key "/path/to/sensitive_data.obk"
```

---

## Security & Threat Model

Ombracrypt is designed strictly for offline data-at-rest protection. 
* **In Scope:** Defending against quantum computing attacks (SNDL), physical device theft, and interactive physical coercion (via the Duress Trapdoor).
* **Out of Scope:** Defending against compromised host environments, active keyloggers, rootkits, or memory-scraping malware.
* **Zero-Knowledge Architecture:** There are no backdoors or recovery systems. If you lose your `.obk` key file or forget your Master Password, your data is mathematically unrecoverable.

For complete cryptographic proofs, entropy calculations, and architectural threat models, please reference the documentation in the main desktop repository:
* [Cryptographic Threat Model & Proofs](https://github.com/ABiswasDev/Ombracrypt/blob/main/docs/CRYPTOGRAPHY.md)

---

## Developer Guide: Building from Source

Because the CLI is a pure Rust application without web frontend dependencies, building from source is highly streamlined. 

**Prerequisites:**
* [Rust & Cargo](https://rustup.rs/) (Latest stable toolchain)

**Compilation:**
```bash
# Clone the repository
git clone [https://github.com/ABiswasDev/Ombracrypt-CLI.git](https://github.com/ABiswasDev/Ombracrypt-CLI.git)
cd Ombracrypt-CLI

# Build the highly optimized release binary
cargo build --release
```
The compiled executable will be available at `target/release/ombracrypt-cli`.

### Project Structure
```plaintext
ombracrypt-cli/
├── Cargo.toml                # Rust dependencies (pqcrypto-kyber, x25519, zeroize)
├── .github/workflows/        # Automated cross-platform release pipeline
└── src/
    ├── main.rs               # CLI routing, argument parsing, and UI prompts
    ├── crypto.rs             # Hybrid KEM, Argon2id KDF, and Phantom Block generation
    ├── pipeline.rs           # 1MB chunked streaming engine and TAR I/O packaging
    └── deception.rs          # Duress trapdoor protocol and .obk noise overwriter
```

---

## License & Legal Disclaimer

* **AGPL-3.0 License:** Ombracrypt CLI is strictly open-source. Any individual or organization that modifies, distributes, or offers this software as a service over a network is legally required to release their modified source code under the exact same AGPL-3.0 license.
* **Liability & Ethical Use:** This software is provided strictly for educational and personal data-protection purposes. The developer assumes zero responsibility or liability for any data loss, misuse, or illegal activity conducted using this software. Users assume full, sole responsibility for their usage.