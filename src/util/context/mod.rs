use anyhow::{Context, Ok};

use crate::util::errors::ScoopResult;
use std::{
    env,
    path::{Path, PathBuf},
};
/// Configuration information for scoop. This is not specific to a command, it is information
/// relating to scoop itself.
#[derive(Debug)]
pub struct GlobalContext {
    scoop_dir: PathBuf,
    /// The current working directory of cargo
    cwd: PathBuf,
}

impl GlobalContext {
    /// Creates a new config instance.
    ///
    /// This is typically used for tests or other special cases. `default` is
    /// preferred otherwise.
    ///
    pub fn new(scoop_dir: PathBuf, cwd: PathBuf) -> Self {
        // let scoop = PathBuf::from(env::var("SCOOP").unwrap_or("~/.scoop".to_string()));
        // let mut scoop_dir = PathBuf::from(scoop);
        GlobalContext {
            scoop_dir: scoop_dir,
            cwd: cwd,
        }
    }
    /// Creates a new instance, with all default settings from env.
    pub fn try_default() -> ScoopResult<GlobalContext> {
        let scoop_dir = PathBuf::from(env::var("SCOOP").context(
            "Scoop couldn't find its directory. This probably means that $SCOOP was not set.",
        )?);
        let cwd =
            env::current_dir().context("couldn't get the current directory of the process")?;
        Ok(GlobalContext::new(scoop_dir, cwd))
    }

    /// The scoop directory.
    pub fn scoop_dir(&self) -> &PathBuf {
        &self.scoop_dir
    }

    /// The current working directory.
    pub fn cwd(&self) -> &Path {
        &self.cwd
    }
}

impl Default for GlobalContext {
    /// Creates a new instance, with all default settings.
    fn default() -> Self {
        GlobalContext {
            scoop_dir: PathBuf::from("~/.scoop"),
            cwd: PathBuf::from("~"),
        }
    }
}

pub mod prelude {
    pub use crate::util::context::GlobalContext;
}
