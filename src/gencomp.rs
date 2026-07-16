use crate::cli::Cli;

use anyhow::{Context, Result};
use clap::{Command, CommandFactory};
use clap_complete::{Shell, generate as generate_completion};

use std::fs::{self, File};
use std::path::Path;

const APP_NAME: &str = "rust_encryption";

fn generate_impl(
    shell: Shell,
    app: &mut Command,
    app_name: &str,
    output_dir: &Path,
    relative_file: &str,
) -> Result<()> {
    let destination = output_dir.join(relative_file);

    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "Failed to create completion directory: {}",
                parent.display()
            )
        })?;
    }

    let mut output = File::create(&destination).with_context(|| {
        format!(
            "Failed to create completion file: {}",
            destination.display()
        )
    })?;

    generate_completion(shell, app, app_name, &mut output);

    Ok(())
}

/// Generate completion scripts for all shells used in the lecture.
pub fn generate(output_dir: &Path) -> Result<()> {
    let mut app = Cli::command();
    app.set_bin_name(APP_NAME);

    generate_impl(
        Shell::Bash,
        &mut app,
        APP_NAME,
        output_dir,
        "bash/rust_encryption",
    )?;

    generate_impl(
        Shell::Elvish,
        &mut app,
        APP_NAME,
        output_dir,
        "elvish/rust_encryption",
    )?;

    generate_impl(
        Shell::Fish,
        &mut app,
        APP_NAME,
        output_dir,
        "fish/rust_encryption",
    )?;

    generate_impl(
        Shell::PowerShell,
        &mut app,
        APP_NAME,
        output_dir,
        "powershell/rust_encryption",
    )?;

    generate_impl(
        Shell::Zsh,
        &mut app,
        APP_NAME,
        output_dir,
        "zsh/_rust_encryption",
    )?;

    println!("Completion files generated in: {}", output_dir.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_all_completion_files() {
        let output_dir = std::env::temp_dir().join(format!(
            "rust-encryption-completions-{}",
            std::process::id()
        ));

        let _ = fs::remove_dir_all(&output_dir);

        generate(&output_dir).unwrap();

        let expected_files = [
            "bash/rust_encryption",
            "elvish/rust_encryption",
            "fish/rust_encryption",
            "powershell/rust_encryption",
            "zsh/_rust_encryption",
        ];

        for file in expected_files {
            assert!(
                output_dir.join(file).is_file(),
                "Missing completion file: {file}"
            );
        }

        fs::remove_dir_all(output_dir).unwrap();
    }
}
