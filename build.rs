use std::{env, fs, path::Path};

// use ethers::utils::Solc;

// --- No longer works after breaking changes to ethers solc
// Compile contracts/** and write the bytecode and abi of each contract to build/
fn main() {
    // // Can't write to OUT_DIR, because can't pass OUT_DIR to ethers' abigen.
    // // Temorarily do bad build script things so we can hardcode path for abigen.
    let out_dir_base = env::current_dir().unwrap();
    let out_dir = Path::new(&out_dir_base).join("build");
    if !out_dir.exists() {
        fs::create_dir(&out_dir).expect("Can't create out_dir.");
    }

    // // compile contracts
    // let solc_output = Solc::new("./contracts/**/*.sol")
    //     .build_raw()
    //     .expect("Compilation error.");

    // // write bytecode and abi's
    // for (name, output) in solc_output {
    //     // solc shouldn't ever give us an empty abi, though maybe one that is just []
    //     fs::write(&out_dir.join(name.clone() + ".abi"), &output.abi).expect("Can't write abi");

    //     // Interface contracts won't produce bytecode, so don't write a .bin file for them
    //     if !&output.bin.is_empty() {
    //         fs::write(&out_dir.join(name + ".bin"), &output.bin).expect("Can't write bytecode");
    //     }
    // }

    // Pass out_dir to env as SOLC_BUILD_DIR
    println!(
        "cargo:rustc-env=SOLC_BUILD_DIR={}",
        out_dir.into_os_string().into_string().unwrap()
    );

    // // Tell Cargo to rerun build script anytime contracts/** is updated
    // println!("cargo:rerun-if-changed=contracts/");
}
