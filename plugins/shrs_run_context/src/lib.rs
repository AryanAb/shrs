//! Save and load shell run context
//!
//!

mod builtin;

use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    path::{Path, PathBuf},
};

use builtin::{LoadBuiltin, SaveBuiltin};
use shrs::prelude::*;

pub struct RunContextState {
    pub(crate) run_contexts: HashMap<String, Runtime>,
    pub(crate) context_file: Option<PathBuf>,
}

pub struct RunContextPlugin {
    context_file: Option<PathBuf>,
}

impl RunContextPlugin {
    pub fn new() -> Self {
        RunContextPlugin { context_file: None }
    }

    pub fn with_file(path: &Path) -> Self {
        RunContextPlugin {
            context_file: Some(path.to_owned()),
        }
    }
}

impl Plugin for RunContextPlugin {
    fn meta(&self) -> PluginMeta {
        PluginMeta {
            name: "Run Context".into(),
            description: String::new(),
        }
    }

    fn init(&self, shell: &mut shrs::ShellConfig) -> anyhow::Result<()> {
        shell.builtins.insert("save", SaveBuiltin);
        shell.builtins.insert("load", LoadBuiltin);

        // if context file was provided, read file into context state
        if let Some(context_file) = &self.context_file {
            // TODO create file if not exist

            let contents = fs::read_to_string(context_file)?;
            let run_contexts: HashMap<String, Runtime> = ron::from_str(&contents)?;

            shell.state.insert(RunContextState {
                run_contexts,
                context_file: Some(context_file.clone()),
            });
        } else {
            shell.state.insert(RunContextState {
                run_contexts: HashMap::new(),
                context_file: None,
            });
        }

        Ok(())
    }
}
