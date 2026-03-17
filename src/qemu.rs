use super::*;
use std::process::{Child, Command};
use std::time::Duration;
use wait_timeout::ChildExt;

/// Qemu run configuration
#[derive(Debug, Clone)]
pub struct QemuConfig {
    pub qemu_path: String,
    pub bios_path: String,
    /// When set, use pflash drives (code_path, vars_path) instead of -bios
    pub pflash: Option<(String, String)>,
    pub drives: Vec<QemuDriveConfig>,
    pub additional_args: Vec<String>,
    /// If true, print the full command line to stderr before spawning
    pub print_cmd: bool,
}

impl Default for QemuConfig {
    fn default() -> Self {
        Self {
            qemu_path: "qemu-system-x86_64".to_string(),
            bios_path: "OVMF.fd".to_string(),
            pflash: None,
            drives: Vec::new(),
            additional_args: vec!["-net".to_string(), "none".to_string()],
            print_cmd: false,
        }
    }
}

impl QemuConfig {
    /// Run an instance of qemu with the given config
    pub fn run(&self) -> Result<QemuProcess> {
        let mut args = Vec::new();
        if let Some((ref code_path, ref vars_path)) = self.pflash {
            args.push("-drive".to_string());
            args.push(format!(
                "if=pflash,format=raw,readonly=on,file={}",
                code_path
            ));
            args.push("-drive".to_string());
            args.push(format!("if=pflash,format=raw,file={}", vars_path));
        } else {
            args.push("-bios".to_string());
            args.push(self.bios_path.clone());
        }
        for (index, drive) in self.drives.iter().enumerate() {
            args.push("-drive".to_string());
            args.push(format!(
                "file={},index={},media={},format={}",
                drive.file, index, drive.media, drive.format
            ));
        }
        args.extend(self.additional_args.iter().cloned());

        if self.print_cmd {
            let quoted: Vec<String> = std::iter::once(self.qemu_path.as_str())
                .chain(args.iter().map(String::as_str))
                .map(|s| {
                    if s.contains(' ') || s.contains('"') || s.contains('\'') {
                        format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
                    } else {
                        s.to_string()
                    }
                })
                .collect();
            eprintln!("{}", quoted.join(" "));
        }

        let child = Command::new(&self.qemu_path).args(&args).spawn()?;
        Ok(QemuProcess { child })
    }
}

/// Qemu drive configuration
#[derive(Debug, Clone)]
pub struct QemuDriveConfig {
    pub file: String,
    pub media: String,
    pub format: String,
}

impl QemuDriveConfig {
    pub fn new(file: &str, media: &str, format: &str) -> Self {
        Self {
            file: file.to_string(),
            media: media.to_string(),
            format: format.to_string(),
        }
    }
}

pub struct QemuProcess {
    child: Child,
}

impl QemuProcess {
    /// Wait for the process to exit for `duration`.
    ///
    /// Returns `true` if the process exited and false if the timeout expired.
    pub fn wait(&mut self, duration: Duration) -> Option<i32> {
        self.child
            .wait_timeout(duration)
            .expect("Failed to wait on child process")
            .map(|exit_status| exit_status.code().unwrap_or(0))
    }

    /// Kill the process.
    pub fn kill(&mut self) -> std::io::Result<()> {
        self.child.kill()
    }
}
