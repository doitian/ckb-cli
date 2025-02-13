mod cell;
mod cell_input;
mod key;
mod script;
mod tx;

pub use cell::LocalCellSubCommand;
pub use cell_input::LocalCellInputSubCommand;
pub use key::LocalKeySubCommand;
pub use script::LocalScriptSubCommand;
pub use tx::LocalTxSubCommand;

use std::path::PathBuf;

use ckb_core::block::Block;
use ckb_sdk::{GenesisInfo, HttpRpcClient};
use clap::{App, ArgMatches, SubCommand};
use jsonrpc_types::BlockNumber;

use super::CliSubCommand;
use crate::utils::printer::{OutputFormat, Printable};

pub struct LocalSubCommand<'a> {
    rpc_client: &'a mut HttpRpcClient,
    genesis_info: Option<GenesisInfo>,
    db_path: PathBuf,
}

impl<'a> LocalSubCommand<'a> {
    pub fn new(
        rpc_client: &'a mut HttpRpcClient,
        genesis_info: Option<GenesisInfo>,
        db_path: PathBuf,
    ) -> LocalSubCommand<'a> {
        LocalSubCommand {
            rpc_client,
            genesis_info,
            db_path,
        }
    }

    fn genesis_info(&mut self) -> Result<GenesisInfo, String> {
        if self.genesis_info.is_none() {
            let genesis_block: Block = self
                .rpc_client
                .get_block_by_number(BlockNumber(0))
                .call()
                .map_err(|err| err.to_string())?
                .0
                .expect("Can not get genesis block?")
                .into();
            self.genesis_info = Some(GenesisInfo::from_block(&genesis_block)?);
        }
        Ok(self.genesis_info.clone().unwrap())
    }

    pub fn subcommand() -> App<'static, 'static> {
        SubCommand::with_name("local").subcommands(vec![
            LocalKeySubCommand::subcommand(),
            LocalCellSubCommand::subcommand(),
            LocalCellInputSubCommand::subcommand(),
            LocalScriptSubCommand::subcommand(),
            LocalTxSubCommand::subcommand(),
            SubCommand::with_name("secp-dep"),
        ])
    }
}

impl<'a> CliSubCommand for LocalSubCommand<'a> {
    fn process(
        &mut self,
        matches: &ArgMatches,
        format: OutputFormat,
        color: bool,
    ) -> Result<String, String> {
        match matches.subcommand() {
            ("key", Some(m)) => LocalKeySubCommand::new(self.rpc_client, self.db_path.clone())
                .process(m, format, color),
            ("script", Some(m)) => {
                LocalScriptSubCommand::new(self.rpc_client, self.db_path.clone())
                    .process(m, format, color)
            }
            ("cell", Some(m)) => LocalCellSubCommand::new(self.rpc_client, self.db_path.clone())
                .process(m, format, color),
            ("cell-input", Some(m)) => {
                LocalCellInputSubCommand::new(self.rpc_client, self.db_path.clone())
                    .process(m, format, color)
            }
            ("tx", Some(m)) => LocalTxSubCommand::new(self.rpc_client, self.db_path.clone())
                .process(m, format, color),
            ("secp-dep", _) => {
                let result = self.genesis_info()?.secp_dep();
                Ok(result.render(format, color))
            }
            _ => Err(matches.usage().to_owned()),
        }
    }
}
