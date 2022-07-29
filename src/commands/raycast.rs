use crate::commands::core::get_fetcher;
use crate::common::shell::{self, ShellSpawnError};
use crate::parser::Parser;
use crate::{prelude::*, serializer};
use clap::{Args, Subcommand};
use std::io::{self, Write};

pub fn handle_var(hash: u64) -> Result<()> {
    let fetcher = get_fetcher()?;
    // let hash: u64 = 2087294461664323320;

    let mut buf = vec![];
    let mut parser = Parser::new(&mut buf, false);
    parser.set_hash(hash);
    let _res = fetcher
        .fetch(&mut parser)
        .context("Failed to parse variables intended for finder")?;

    let variables = parser.variables;
    let item_str = String::from_utf8(buf)?;
    let item = serializer::raycast_deser(&item_str)?;
    dbg!(&item);

    let x = variables.get_suggestion(&item.tags, "local_branch").expect("foo");
    dbg!(&x);

    let suggestion_command = x.0.clone();
    let child = shell::out()
        .stdout(Stdio::piped())
        .arg(&suggestion_command)
        .spawn()
        .map_err(|e| ShellSpawnError::new(suggestion_command, e))?;

    let text = String::from_utf8(
        child
            .wait_with_output()
            .context("Failed to wait and collect output from bash")?
            .stdout,
    )
    .context("Suggestions are invalid utf8")?;

    dbg!(&text);

    Ok(())
}

pub fn handle_list() -> Result<()> {
    let fetcher = get_fetcher()?;

    let mut stdout = io::stdout();
    let mut writer: Box<&mut dyn Write> = Box::new(&mut stdout);
    let mut parser = Parser::new(&mut writer, false);

    let _res = fetcher
        .fetch(&mut parser)
        .context("Failed to parse variables intended for finder")?;

    Ok(())
}

#[derive(Debug, Clone, Subcommand)]
pub enum Subcmd {
    List,
    Var { hash: u64 },
}

#[derive(Debug, Clone, Args)]
pub struct Input {
    #[clap(subcommand)]
    pub subcmd: Subcmd,
}

impl Runnable for Input {
    fn run(&self) -> Result<()> {
        use Subcmd::*;
        match self.subcmd {
            List => handle_list()?,
            Var { hash } => handle_var(hash)?,
        }
        Ok(())
    }
}
