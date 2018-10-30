extern crate vl53l0x_sys;
use vl53l0x_sys::*;

fn main() {
    let mut version = VL53L0X_Version_t::default();
    if unsafe { VL53L0X_GetVersion(&mut version) } != 0 {
        panic!("VL53L0X_GetVersion");
    }
    println!("{:#?}", version);

    let mut device = VL53L0X_Dev_t::default();
    if unsafe { VL53L0X_DataInit(&mut device) } != 0 {
        panic!("VL53L0X_DataInit");
    }

    println!("{:#?}", device);
}
