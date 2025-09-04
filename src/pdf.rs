use anyhow::{Context, Result};
use std::process::Command;

/// Extract text from a PDF using `pdftotext` (poppler-utils must be installed).
pub fn extract_text(path: &str) -> Result<String> {
    let txt_file = format!("{}.txt", path);

    // Run `pdftotext -layout file.pdf file.pdf.txt`
    let status = Command::new("pdftotext")
        .arg("-layout")
        .arg(path)
        .arg(&txt_file)
        .status()
        .with_context(|| format!("Failed to run pdftotext on {}", path))?;

    if !status.success() {
        anyhow::bail!("pdftotext failed for {}", path);
    }

    let raw_bytes = std::fs::read(&txt_file)
        .with_context(|| format!("Failed to read extracted text from {}", txt_file))?;

    // Convert lossy: replaces invalid UTF-8 with � so program won’t crash
    let text = String::from_utf8_lossy(&raw_bytes).to_string();

    Ok(text)
}
