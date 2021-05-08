use anyhow::{anyhow, Result};
use std::process::Command;

pub fn branches() -> Result<Vec<String>> {
    let output = Command::new("git")
        .args(&["for-each-ref", "--format=%(refname:short)", "refs/heads/"])
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("Failed to list branches"));
    }

    let output = std::str::from_utf8(&output.stdout)?.trim();
    Ok(output.split("\n").map(String::from).collect())
}

pub fn delete_branches(branches: &[&str]) -> Result<()> {
    let mut args = vec!["branch", "-D"];
    args.extend_from_slice(branches);

    let output = Command::new("git").args(&args).output()?;

    if !output.status.success() {
        return Err(anyhow!("Failed to delete branches"));
    }

    Ok(())
}
