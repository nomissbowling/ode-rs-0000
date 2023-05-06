/*
  build.rs for ode-rs-0000

  (skip C/C++ Bridge)

  cc-rs bindgen
  and generate link option
*/

extern crate cc;
extern crate bindgen;

use std::path::PathBuf;

fn main() {
  let _mk_cc = |dname: &str, sname: &str, iname: &str, oname: &str| {
    let sd = PathBuf::from(dname);
    let fname = format!("{}", sd.join(sname).to_str().expect("invalid path"));
    println!("cargo:rerun-if-changed={}", fname);
    cc::Build::new()
      .file(fname)
      // .flag("-std=c++11") // gcc
      .flag("-std:c11") // cl
      // .flag("-std:c++14") // cl
      .include(iname)
      .compile(oname)
  };
  // mk_cc("./src", "bridge.cpp", "./include", "bridge");

  let _mk_bindings = |hdd: &str, header: &str, rsd: &str, rsfile: &str| {
    let hd = PathBuf::from(hdd);
    let hf = format!("{}", hd.join(header).to_str().expect("invalid path"));
    println!("cargo:rerun-if-changed={}", hf);
    let bindings = bindgen::Builder::default()
      .header(hf)
      .parse_callbacks(Box::new(bindgen::CargoCallbacks))
      .generate()
      .expect("Unable to generate bindings");
    let rs = PathBuf::from(rsd);
    bindings
      .write_to_file(rs.join(rsfile))
      .expect("Could not write bindings!");
  };
  // mk_bindings("./include", "bridge.hpp", "./include", "bridge_bindings.rs");
  // mk_bindings("./ode", "drawstuff.h", "./ode", "drawstuff_bindings.rs");
  // mk_bindings("./ode", "ode.hpp", "./ode", "ode_bindings.rs");

  println!("cargo:rustc-link-search=../oyk/ode/lib");
  println!("cargo:rustc-link-lib=drawstuff");
  println!("cargo:rustc-link-lib=ode");
}
