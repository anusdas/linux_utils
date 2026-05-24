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
