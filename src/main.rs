use eyre::{Context, Result};

struct GithubFilesystem {
    runtime: tokio::runtime::Runtime,
}
impl GithubFilesystem {
    fn new(runtime: tokio::runtime::Runtime) -> Self {
        Self { runtime }
    }
}


impl fuser::Filesystem for GithubFilesystem {
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    color_eyre::install()?;

    let runtime = tokio::runtime::Builder::new_multi_thread().build().wrap_err("building tokio runtime")?;

    let fs = GithubFilesystem::new(runtime);
    fuser::mount2(fs, "/tmp/github-fuse-mnt", &[]).wrap_err("mounting file system")?;

    Ok(())
}
