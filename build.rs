extern crate cc;

use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let src_path = env::var("VL53L0X_SRC_PATH").expect("VL53L0X_SRC_PATH must be set");
    let out_path = env::var("OUT_DIR").expect("OUT_DIR must be st");

    let api = Path::new(&src_path).join("Api");
    let inc = api.join("core").join("inc");
    let src = inc.join("vl53l0x_api.h");
    let out = Path::new(&out_path).join("bindings.rs");

    let plt = api.join("platform").join("inc");

    let status = Command::new("bindgen")
        .arg("--with-derive-default")
        .arg("--whitelist-function")
        .arg("VL53L0X_[A-Z].*")
        .arg("-o")
        .arg(&out)
        .arg(&src)
        .arg("--")
        .arg("-I")
        .arg(&inc)
        .arg("-I")
        .arg(&plt)
        .status()
        .expect("bindgen failed");
    assert!(status.success());

    let files = &[
        "core/src/vl53l0x_api.c",
        "core/src/vl53l0x_api_core.c",
        "core/src/vl53l0x_api_calibration.c",
        "core/src/vl53l0x_api_ranging.c",
        "core/src/vl53l0x_api_strings.c",
    ];

    let mut build = cc::Build::new();
    build.include(&inc).include(&plt).warnings(false);

    for file in files {
        build.file(api.join(file));
    }

    build.compile("capi");
}
