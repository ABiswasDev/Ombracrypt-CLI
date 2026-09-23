<p align="center">
  <img src="images/banner.png" alt="Ombracrypt Banner" width="800">
</p>

<h1 align="center">Ombracrypt CLI</h1>

<p align="center">
  <strong>The zero-trust quantum vault, built for the terminal.</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/github/v/release/ABiswasDev/Ombracrypt-CLI?style=flat-square" alt="Release">
  <img src="https://img.shields.io/github/license/ABiswasDev/Ombracrypt-CLI?style=flat-square" alt="License">
</p>

### Overview

Ombracrypt-CLI is an offline, zero-telemetry Post-Quantum Cryptographic (PQC) file encryption engine designed for headless servers and terminal power users. It secures sensitive digital assets against both classical vulnerabilities and future "Store Now, Decrypt Later" (SNDL) quantum attacks. Operating completely independently of cloud infrastructure and graphical dependencies, it functions seamlessly as a standalone tool or integrates directly into existing security pipelines, further fortified by a stealth Anti-Coercion mechanism.

Maintains a cryptographic parity with the [Ombracrypt GUI Application](https://github.com/ABiswasDev/Ombracrypt).

### Key Features
* **Zero Dependencies:** Distributed as a single, pre-compiled binary for Linux, macOS, and Windows. No runtime environments required.
* **Hybrid KEM Architecture:** Fuses classical X25519 (ECDH) with Post-Quantum ML-KEM (Kyber-768/1024) to mitigate both classical and quantum attacks.
* **Massive Payload Handling:** Strict 1MB chunked streaming architecture enables encryption of highly massive files with a near-zero, flat RAM footprint.
* **Anti-Coercion Trapdoor:** Features a Plausibly Deniable Duress Trapdoor that silently destroys the cryptographic key if an attacker forces you to surrender a secondary deception passcode.
* **Secure UI:** Implements strict, memory-safe password masking directly in the terminal to prevent shoulder-surfing and shell history leakage.
* **Memory Zeroization:** Integrates the `zeroize` crate to securely scrub all ephemeral keys, passcodes, and shared secrets from active RAM immediately upon task completion.

## Download & Installation

Download the latest standalone binary for your operating system from the [Releases Page](https://github.com/ABiswasDev/Ombracrypt-CLI/releases).

**Linux & macOS:**
```bash
# Extract the archive
tar -xvf ombracrypt-cli-linux-x64.tar.gz

# Make executable and move to system path for global use
chmod +x ombracrypt-cli

```
#### Note: Ombracrypt-CLI is a standalone, portable binary requiring no installation. You can run it directly from your current directory using ./ombracrypt-cli

#### Windows: Extract the `.zip` archive and run the executable directly from your command line using `.\ombracrypt-cli.exe`.


## Usage

Ombracrypt CLI uses a clean, interactive command-line interface designed for terminal environments. 

### Global Commands
To check your current installed version or view the general help menu:
```bash
ombracrypt-cli --version
ombracrypt-cli --help
```

*Note:* To view the detailed arguments and options for a specific operation, append `--help` directly to the subcommand:
```bash
ombracrypt-cli encrypt --help
ombracrypt-cli decrypt --help
```

### Encrypt a Vault
To lock a file or directory, you must specify your target path, your Post-Quantum KEM profile (`xwing` or `cypherpunk`), and your symmetric cipher (`aes256gcm` or `xchacha20`).

The cryptographic engine will interactively prompt you to securely enter your Master Password and an optional Deception Passcode directly in the terminal.

Here are the 4 available cryptographic combinations:

**1. AES-256-GCM & xwing KEM (Kyber-768)**
```bash
ombracrypt-cli encrypt --target "/driveA/data" --kem xwing --cipher aes256gcm
```

**2. AES-256-GCM & Cypherpunk KEM (Kyber-1024)**
```bash
ombracrypt-cli encrypt --target "/driveA/data" --kem cypherpunk --cipher aes256gcm
```

**3. XChaCha20-Poly1305 & xwing KEM (Kyber-768)**
```bash
ombracrypt-cli encrypt --target "/driveA/data" --kem xwing --cipher xchacha20
```

**4. XChaCha20-Poly1305 & Cypherpunk KEM (Kyber-1024)**
```bash
ombracrypt-cli encrypt --target "/driveA/data" --kem cypherpunk --cipher xchacha20
```

**Expected Output:** 
The engine will generate two new files in the exact same directory as your original target:
* `/driveA/data.obv` (The securely encrypted vault payload)
* `/driveA/data.obk` (Your physical Ombracrypt PQC Key)

### Decrypt a Vault
To unlock an `.obv` vault, you must provide the path to both the encrypted vault file and your corresponding `.obk` key file.

```bash
ombracrypt-cli decrypt --target "/driveA/data.obv" --key "/driveA/data.obk"
```

### Important Operational Notes

1. **Target Preparation & Output Position:** All files that need to be encrypted should be grouped together inside a specific folder (e.g., a `/data/` directory) located within a larger path (e.g., `/driveA/`). After encryption, the Vault (`.obv`) and Ombracrypt Key (`.obk`) will be generated inside `/driveA/`, immediately outside of the `/data/` directory.
2. **Process Interruptions:** If the application crashes or is interrupted for any reason, the encryption will be incomplete. A vault and key may still be generated, but there is a high possibility they will be corrupted. It is highly recommended to delete them and encrypt again in that case.
3. **Encryption Overwrites:** If a key (`.obk`) and vault (`.obv`) file already exist inside `/driveA/` during a new encryption process, they will eventually be replaced by the new output.
4. **Decryption Overwrites:** During decryption, you can expect the generated files to be extracted back inside `/driveA/`. If the `/data/` directory already exists inside `/driveA/`, all data inside it will be overwritten and replaced by the decrypted payload.
5. **Pathing Context:** The names `/driveA/` and `/data/` are used here strictly to make the explanation easy to follow. None of them are static names preferred by the application; users can select any path and folder names according to their choice.
6. **Default parameters:** If a KEM or cipher is not specified, --kem cypherpunk and --cipher aes256gcm will automatically be applied as the defaults parameters.


## Security & Threat Model

Ombracrypt is designed strictly for offline data-at-rest protection. 
* **In Scope:** Defending against quantum computing attacks (SNDL), physical device theft, and interactive physical coercion (via the Duress Trapdoor).
* **Out of Scope:** Defending against compromised host environments, active keyloggers, rootkits, or memory-scraping malware.
* **Zero-Knowledge Architecture:** There are no backdoors or recovery systems. If you lose your `.obk` key file or forget your Master Password, your data is mathematically unrecoverable.

For complete cryptographic proofs, entropy calculations, and architectural threat models, please reference the documentation in the main desktop repository:
* [Cryptographic Threat Model & Proofs](https://github.com/ABiswasDev/Ombracrypt/blob/main/docs/CRYPTOGRAPHY.md)


## Developer Guide: Building from Source

Because the CLI is a pure Rust application without web frontend dependencies, building from source is highly streamlined. 

**Prerequisites:**
* [Rust & Cargo](https://rustup.rs/) (Latest stable toolchain)

**Compilation:**
```bash
# Clone the repository
git clone https://github.com/ABiswasDev/Ombracrypt-CLI.git
cd Ombracrypt-CLI

# Build the highly optimized release binary
cargo build --release
```
The compiled executable will be available at `target/release/ombracrypt-cli`.

## Project Structure
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



## Authorship & Copyright

**Ombracrypt CLI** is developed and maintained by **Abhishek Biswas** ([@ABiswasDev](https://github.com/ABiswasDev)). 

This project is an official release under **[Ombraveil](https://www.linkedin.com/company/ombraveil/)**, an open-source community and initiative dedicated to advancing post-quantum cryptographic tools and research.

**Copyright:**
Copyright © 2026 Abhishek Biswas (Ombraveil). All rights reserved.

## License & Legal Disclaimer

* **AGPL-3.0 License:** Ombracrypt CLI is strictly open-source. Any individual or organization that modifies, distributes, or offers this software as a service over a network is legally required to release their modified source code under the exact same AGPL-3.0 license.
* **Liability & Ethical Use:** This software is provided strictly for educational and personal data-protection purposes. The developer assumes zero responsibility or liability for any data loss, misuse, or illegal activity conducted using this software. Users assume full, sole responsibility for their usage.

**Contact & Security Disclosures:**
For security vulnerability reports, cryptographic audits, or professional inquiries, please contact: `ombraveil.choice471@passinbox.com` 
*(For general bug reports and feature requests, please use the GitHub Issues tracker).*