use std::{env, path::PathBuf};

macro_rules! warn {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main() {
	let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
	let static_folder = manifest_dir.join("../static");
	let out_path = manifest_dir.join("../build/_app/immutable/assets");

	let files = ["32px.png", "throbber.gif"];

	for file in files {
		std::fs::copy(static_folder.join(file), out_path.join(file)).expect("Failed to copy asset to output directory");
	}

	if let Ok(resourcelib_path) = env::var("DEP_RESOURCELIB_LIB_PATH") {
		println!("cargo:rustc-link-search={}", resourcelib_path);
		println!("cargo:rustc-link-arg=-Wl,-rpath={}", resourcelib_path);

		println!("cargo:rustc-link-lib=dylib=ResourceLib_HM2016");
		println!("cargo:rustc-link-lib=dylib=ResourceLib_HM2");
		println!("cargo:rustc-link-lib=dylib=ResourceLib_HM3");
	} else {
		warn!("Could not location ResourceLib", );
	}

	tauri_build::build();
}
