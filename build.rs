
extern crate cpp_build;
use std::io;
#[cfg(windows)] use winres::WindowsResource;

fn main() -> io::Result<()> {
    let include_path = "C:\\Git\\Shutdowner\\lib";
    let lib_path =  "C:\\Git\\Shutdowner\\lib";
    cpp_build::Config::new().include(include_path).build("src/volume.rs");
    println!("cargo:rustc-link-search={}", lib_path);
    println!("cargo:rustc-link-lib=SetVolume");
    println!("Check if win.... ");

    #[cfg(windows)] {
        print!("Packing");
        WindowsResource::new()
            // This path can be absolute, or relative to your crate root.
            .set_icon("assets/icon.ico")
            .compile()?;
    }

    Ok(())
}