extern crate amqp;
extern crate env_logger;

use std::fs::File;
use std::path::Path;
use ofborg::nix;

pub struct EvalChecker {
    name: String,
    op: nix::Operation,
    args: Vec<String>,
    nix: nix::Nix,
}

impl EvalChecker {
    pub fn new(name: &str, op: nix::Operation, args: Vec<String>, nix: nix::Nix) -> EvalChecker {
        EvalChecker {
            name: name.to_owned(),
            op: op,
            args: args,
            nix: nix,
        }
    }

    pub fn name(&self) -> String {
        format!("grahamcofborg-eval-{}", self.name)
    }

    pub fn execute(&self, path: &Path) -> Result<File, File> {
        self.nix.safely(self.op.clone(), path, self.args.clone(), false)
    }

    pub fn cli_cmd(&self) -> String {
        let mut cli = vec![self.op.to_string()];
        cli.append(&mut self.args.clone());
        return cli.join(" ");
    }
}
