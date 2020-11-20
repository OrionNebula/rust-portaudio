// The MIT License (MIT)
//
// Copyright (c) 2013 Jeremy Letang (letang.jeremy@gmail.com)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

extern crate cmake;
use cmake::Config;

extern crate bindgen;
use bindgen::{Builder, CargoCallbacks};

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    build_lib();

    let bindings = Builder::default()
        .header("portaudio/include/portaudio.h")
        .whitelist_function("[pP]a.*")
        .whitelist_var("[pP]a.*")
        .whitelist_type("[pP]a.*")
        .constified_enum("[pP]a.*")
        .blacklist_type("PaStreamCallbackResult")
        .parse_callbacks(Box::new(CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}

fn build_lib() {
    // Current git revision of PortAudio
    let revision = git2::Repository::open("portaudio")
        .unwrap()
        .head()
        .unwrap()
        .target()
        .unwrap()
        .to_string();

    let dst = Config::new("portaudio")
        .cflag(format!("-DPA_GIT_REVISION={}", revision))
        .define("CMAKE_BUILD_TYPE", "RELEASE")
        .very_verbose(true)
        .build();

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=Carbon");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=AudioToolbox");
        println!("cargo:rustc-link-lib=framework=CoreAudio");
    }

    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=static=asound");
        todo!();
    }

    if cfg!(target_os = "windows") {
        todo!();
    }

    if cfg!(feature = "static") {
        println!(
            "cargo:rustc-link-search=native={}",
            dst.join("build").display()
        );
        println!("cargo:rustc-link-lib=static=portaudio_static");
    } else {
        todo!("Dynamic linking not yet supported")
    }
}
