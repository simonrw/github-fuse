use eyre::{Context, Result};

struct GithubFilesystem;

impl fuser::Filesystem for GithubFilesystem {
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;

    let fs = GithubFilesystem {};
    fuser::mount2(fs, "/tmp/github-fuse-mnt", &[]).wrap_err("mounting file system")?;

    Ok(())
}
