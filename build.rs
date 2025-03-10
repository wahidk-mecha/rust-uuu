#![allow(improper_ctypes)]

use std::env;
use std::path::PathBuf;

use cmake::Config;

fn main() {
    let system = env::var("SYSTEM").unwrap();

    let dst = Config::new("./vendor/mfgtools/")
        .define("CMAKE_BUILD_TYPE", "Release")
        .build();

    println!("cargo:rustc-link-search={}/build/libuuu", dst.display());
    //println!("cargo:rustc-link-search=./vendor/mfgtools/build/libuuu");
    //println!("cargo:rustc-link-search=/usr/lib/gcc/x86_64-pc-linux-gnu/14.2.1");
    //println!("cargo:rustc-link-search=../msvc/x64/Release/");
    //println!("cargo:rustc-link-search=../libusb/build/v143/x64/Release/lib");
    //println!("cargo:rustc-link-search=../zstd/build/VS2010/bin/x64_Release");

    if system == "x86_64-darwin" || system == "aarch64-darwin" {
        println!("cargo:rustc-link-lib=c++");
    } else if system == "x86_64-linux" || system == "aarch64-linux" {
        println!("cargo:rustc-link-lib=stdc++");
        //println!("cargo:rustc-link-lib=static=gcc");
    }

    if system == "x86_64-windows" {
        println!("cargo:rustc-link-lib=libusb-1.0");
        println!("cargo:rustc-link-lib=bzip2");
        println!("cargo:rustc-link-lib=libuuu");
        println!("cargo:rustc-link-lib=tinyxml2");
        println!("cargo:rustc-link-lib=zlib");
        println!("cargo:rustc-link-lib=libzstd");
        println!("cargo:rustc-link-lib=libuuu");
    } else {
        println!("cargo:rustc-link-lib=dylib=usb-1.0");
        println!("cargo:rustc-link-lib=dylib=crypto");
        println!("cargo:rustc-link-lib=dylib=z");
        println!("cargo:rustc-link-lib=dylib=zstd");
        println!("cargo:rustc-link-lib=dylib=bz2");
        println!("cargo:rustc-link-lib=dylib=tinyxml2");
        println!("cargo:rustc-link-lib=dylib=ssl");
        println!("cargo:rustc-link-lib=static=uuc_s");
    }
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-x")
        .clang_arg("c++")
        //.clang_arg("-stdlib=libstdc++")
        //.clang_arg("-D_GLIBCXX_USE_CXX11_ABI=1")
        .clang_arg("-I./vendor/mfgtools/libuuu")
        .clang_arg("-I./vendor/mfgtools/libusb/libusb")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
