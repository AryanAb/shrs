use std::{
    env,
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};

use super::{BuiltinCmd, Builtins};
use crate::{
    hooks::ChangeDirCtx,
    prelude::CmdOutput,
    shell::{Context, Runtime, Shell},
};

#[derive(Parser)]
#[clap(disable_help_flag = true, disable_help_subcommand = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Builtin,
}

#[derive(Default)]
pub struct HelpBuiltin {}
impl BuiltinCmd for HelpBuiltin {
    fn run(
        &self,
        sh: &Shell,
        ctx: &mut Context,
        rt: &mut Runtime,
        args: &Vec<String>,
    ) -> anyhow::Result<CmdOutput> {
        let cli = Cli::try_parse_from(args)?;

        match &cli.command {
            Commands::Builtin => {
                let cmds = sh.builtins.builtins.keys();
                let info = sh.keybinding.get_info();

                ctx.out.println("Builtin Commands:")?;

                for cmd in cmds {
                    ctx.out.println(cmd)?;
                }

                ctx.out.println("")?;
                ctx.out.println("Key Bindings:")?;
                for binding in info.keys() {
                    ctx.out.println(binding)?;
                }
            },
        }

        Ok(CmdOutput::success())
    }
}
