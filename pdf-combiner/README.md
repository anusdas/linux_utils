# PDF Combiner Utility 📄🔗

A fast, lightweight, pure-Rust command-line utility to merge multiple PDF files sequentially into a single target file.

---

## ⚙️ Compilation & Global Setup

To compile the optimized native Linux binary and install it globally as a system command, execute the following commands from this directory:

```bash
# 1. Build the optimized release binary
cargo build --release

# 2. Copy the binary to your local system path
sudo cp target/release/pdf-combiner /usr/local/bin/combine-pdf
sudo chmod +x /usr/local/bin/combine-pdf



## 💾 Detailed System-Wide Installation Guide

Deploying the utility system-wide moves the binary execution pointer into the system's global environments. This allows any local user or background automation script to call the command globally without needing explicit path definitions.

### 📦 Installation Procedures

Choose one of the two methods below to register the utility on your machine:

#### Method A: Direct Deployment (Pre-Compiled Binary)
If you wish to quickly deploy the portable executable included directly in this repository without triggering a local compilation sequence, copy it to your local binaries path:
```bash
sudo cp pdf-combiner/target/release/pdf-combiner /usr/local/bin/combine-pdf





Ask the system shell to find the exact binary location mapping:
```bash
which combine-pdf
```bash
