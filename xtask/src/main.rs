use std::{process::Command, sync::mpsc::channel};

use anyhow::Ok;
use clap::Parser;
use ts_rs::TS;
use xshell::{cmd, Shell};

#[derive(Parser)]
enum XtaskCommand {
    TS,
    Dev,
}

const PACKAGE_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn main() -> Result<(), anyhow::Error> {
    let command = XtaskCommand::parse();
    println!("{PACKAGE_DIR}");

    match command {
        XtaskCommand::TS => {
            export_ts_types()?;
        }
        XtaskCommand::Dev => {
            export_ts_types()?;

            let (tx, rx) = channel();

            ctrlc::set_handler(move || tx.send(()).expect("bro"))?;

            let mut backend = Command::new("cargo")
                .args(["run", "-p", "scalar-axum", "--example", "test"])
                .spawn()?;

            let shell = Shell::new()?;

            shell.change_dir(format!("{PACKAGE_DIR}/../scalar-cp"));

            let _frontend = cmd!(shell, "bun run dev").run();

            rx.recv()?;

            backend.kill()?;
        }
    }

    Ok(())
}

fn export_ts_types() -> Result<(), anyhow::Error> {
    println!("Exporting typescript types...");
    let typescript_directory = format!("{PACKAGE_DIR}/../typescript_bindings");
    scalar_cms::Item::<()>::export_all_to(&typescript_directory)?;
    scalar_cms::Schema::export_all_to(&typescript_directory)?;
    scalar_cms::DocInfo::export_all_to(&typescript_directory)?;

    Ok(())
}
