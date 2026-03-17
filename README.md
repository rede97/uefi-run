# uefi-run [![Latest Version]][crates.io] [![Build Status]][travis]

[Build Status]: https://travis-ci.org/Richard-W/uefi-run.svg?branch=master
[travis]: https://travis-ci.org/Richard-W/uefi-run
[Latest Version]: https://img.shields.io/crates/v/uefi-run.svg
[crates.io]: https://crates.io/crates/uefi-run

**Directly run UEFI applications in qemu**

---

This helper application takes an EFI executable, builds a FAT filesystem around
it, adds a startup script and runs qemu to run the executable.

It does not require root permissions since it uses the [fatfs](https://crates.io/crates/fatfs)
crate to build the filesystem image directly without involving `mkfs`, `mount`,
etc.

## Usage

### BIOS vs pflash

- **Default**: uses qemu’s `-bios` with a single OVMF image (e.g. `OVMF.fd`). Many distros now only ship split OVMF (code + vars), which does not work with `-bios`.
- **`--pflash`**: use two pflash drives (OVMF code + vars). Code and vars are auto-detected under `/usr/share/OVMF/`:
  - `OVMF_CODE_4M.fd` (read-only)
  - `OVMF_VARS_4M.fd` (template; copied to a writable location if not already present)
- **`--ovmf-vars-dir <dir>`**: directory for the vars copy (default: current directory). If the vars file already exists there, it is not overwritten.
- **`--ovmf-code` / `--ovmf-vars`**: override the default paths when using `--pflash`.

Example with pflash and custom qemu:

```bash
uefi-run --pflash -q /path/to/qemu-system-x86_64 ./app.efi -- -m 32 -serial stdio
```

### Other options

- **`--print-cmd`**: print the full qemu command line to stderr before running (useful for debugging or copying the command).
- **`-d` / `--boot`**: install the app as the default boot loader (`EFI/Boot/BootX64.efi`) so the firmware runs it directly, without entering the UEFI shell or running `startup.nsh`.

## Installation

### Snap

uefi-run can be installed from the snapstore:
```bash
snap install --edge uefi-run
```
The confinement of this snap is somewhat strict. It can only access non-hidden files in the user's
home directory. Also it has no network access.

### Cargo

You can install cargo and rust using the rustup tool:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After cargo has been installed you can build and install uefi-run:
```bash
cargo install uefi-run
```

### Packages provided by third parties

Third-party packages are controlled by their respective maintainers. They are not associated to
this project. Use at your own risk.

* [AUR PKGBUILD for Arch Linux](https://aur.archlinux.org/packages/uefi-run) contributed by @rubo3
