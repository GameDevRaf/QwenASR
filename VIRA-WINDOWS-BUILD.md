# Vira Windows x64 build of QwenASR

This fork exists only so Vira can ship a prebuilt Windows binary of the upstream
[huanglizhuo/QwenASR](https://github.com/huanglizhuo/QwenASR) CLI (MIT). Upstream
publishes Linux and macOS binaries but no Windows binary, and asking users to
install Rust and the MSVC build tools during installation is not acceptable.

- Upstream tag: `qwen-asr-cli-v0.9.1`, commit `d39208de340a8b0c3a28f53ac1fd160a7118666f`.
- Change: Vira's Windows compatibility patch only (a `memmap2`-free owned-buffer
  weight loader and a Windows int8 sidecar stub). No model or algorithm changes.
- Target: `x86_64-pc-windows-msvc`, stable Rust, CPU baseline `x86-64-v3`
  (AVX2 and FMA, the same baseline as upstream's Linux release), no BLAS,
  statically linked CRT (imports only core Windows system DLLs).
- Build command:

  ```text
  RUSTFLAGS="-C target-cpu=x86-64-v3 -C target-feature=+crt-static"
  cargo build --release --locked -p qwen-asr-cli --no-default-features --target x86_64-pc-windows-msvc
  ```

- Release asset: `qwen-asr-0.9.1-x86_64-pc-windows-msvc.tar.gz` (contains `qwen-asr.exe` and `LICENSE`).
  Vira pins the asset's SHA-256 and verifies it before extracting.
- Verified: `qwen-asr.exe` transcribed a 15 second narration correctly with
  `Qwen/Qwen3-ASR-0.6B` (revision `5eb144179a02acc5e5ba31e748d22b0cf3e303b0`).

The binary is not code-signed. Windows SmartScreen does not apply to a process
started by another program, but antivirus products may scan it on first run.
