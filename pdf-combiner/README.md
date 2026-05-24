# PDF Combiner Utility 📄🔗

> A fast, lightweight, pure-Rust command-line utility for merging multiple PDF files sequentially into a single output document.

---

## ✨ Features

- ⚡ Fast native performance (Rust-powered)
- 🪶 Lightweight with minimal overhead
- 📄 Merge multiple PDF files in sequence
- 🔧 Simple CLI workflow
- 🌍 Global system-wide installation support

---

## ⚙️ Compilation & Global Installation

Build the optimized release binary and install it as a globally accessible command:

```bash
# Build optimized release binary
cargo build --release

# Install globally
sudo cp target/release/pdf-combiner /usr/local/bin/combine-pdf
sudo chmod +x /usr/local/bin/combine-pdf
```

---

## 📦 Installation Methods

### Method 1 — Build From Source (Recommended)

Use this method if you want the latest build directly from source.

```bash
cargo build --release
sudo cp target/release/pdf-combiner /usr/local/bin/combine-pdf
sudo chmod +x /usr/local/bin/combine-pdf
```

---

### Method 2 — Install Pre-Compiled Binary

If you already have a compiled binary available:

```bash
sudo cp pdf-combiner/target/release/pdf-combiner /usr/local/bin/combine-pdf
sudo chmod +x /usr/local/bin/combine-pdf
```

---

## ✅ Verify Installation

Confirm the command is available system-wide:

```bash
which combine-pdf
combine-pdf --help
```

Expected output:

```bash
/usr/local/bin/combine-pdf
```

---

## 🚀 Usage

Basic usage:

```bash
combine-pdf file1.pdf file2.pdf file3.pdf output.pdf
```

Example:

```bash
combine-pdf invoice.pdf appendix.pdf contract.pdf merged-document.pdf
```

---

## 🛠 Requirements

Before building from source, ensure you have:

- Rust toolchain installed
- Cargo package manager
- Linux environment with `/usr/local/bin` in your PATH

Install Rust if needed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## 📁 Binary Location

Installed executable path:

```bash
/usr/local/bin/combine-pdf
```
