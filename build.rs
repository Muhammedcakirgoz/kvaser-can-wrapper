fn main() {
	println!("cargo:rustc-link-lib=canlib");
	println!("cargo:rustc-link-search=/usr/lib");

	let bindings = bindgen::Builder::default()
		.header("/usr/include/canlib.h")
		.parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
		.generate()
		.expect("canlib.h'den bindings uretilemedi");
	
	let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
	bindings
		.write_to_file(out_path.join("bindings.rs"))
		.expect("bindings.rs yazilamadi");
}
