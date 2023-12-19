use std::{
    env::{current_dir, var_os},
    fs::{copy, read_dir},
    path::PathBuf,
};

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let helper_dir = current_dir()?;

    println!("cargo:warning=helper_dir - {helper_dir:?}");

    let out_dir = var_os("OUT_DIR")
        .ok_or(anyhow!("OUT_DIR variable is not set"))?
        .to_string_lossy()
        .to_string();

    let index = out_dir
        .find("target/")
        .ok_or(anyhow!("Failed to find target/ directory in workspace path: {out_dir}"))?;

    let project_dir = PathBuf::from(&out_dir[..index]);

    println!("cargo:warning=project_dir- {project_dir:?}");

    let integration_res = helper_dir.parent().unwrap().join("res");

    println!("cargo:warning=integration_res - {integration_res:?}");

    let res_dir = read_dir(&integration_res).unwrap();

    for dir in res_dir {
        println!("cargo:warning=dir- {dir:?}");
    }

    let root_res = project_dir.join("res");

    println!("cargo:warning=root_res - {root_res:?}");

    copy(
        integration_res.join("helper_contract.wasm"),
        root_res.join("helper_contract.wasm"),
    )
    .unwrap();

    println!("cargo:warning=Helper build.rs: OK");

    Ok(())
}
