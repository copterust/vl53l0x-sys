#![allow(unused)]

use crate::{VL53L0X_Error, VL53L0X_DEV};

#[no_mangle]
pub extern "C" fn VL53L0X_RdByte(Dev: VL53L0X_DEV, index: u8, data: *mut u8) -> VL53L0X_Error {
    0
}

#[no_mangle]
pub extern "C" fn VL53L0X_RdDWord(Dev: VL53L0X_DEV, index: u8, data: *mut u32) -> VL53L0X_Error {
    0
}

#[no_mangle]
pub extern "C" fn VL53L0X_RdWord(Dev: VL53L0X_DEV, index: u8, data: *mut u16) -> VL53L0X_Error {
    0
}

#[no_mangle]
pub extern "C" fn VL53L0X_ReadMulti(
    Dev: VL53L0X_DEV,
    index: u8,
    pdata: *mut u8,
    count: u32,
) -> VL53L0X_Error {
    0
}

#[no_mangle]
pub extern "C" fn VL53L0X_WrByte(Dev: VL53L0X_DEV, index: u8, data: u8) -> VL53L0X_Error {
    0
}

#[no_mangle]
pub extern "C" fn VL53L0X_WrDWord(Dev: VL53L0X_DEV, index: u8, data: u32) -> VL53L0X_Error {
    0
}

#[no_mangle]
pub extern "C" fn VL53L0X_WriteMulti(
    Dev: VL53L0X_DEV,
    index: u8,
    pdata: *mut u8,
    count: u32,
) -> VL53L0X_Error {
    0
}

#[no_mangle]
pub extern "C" fn VL53L0X_WrWord(Dev: VL53L0X_DEV, index: u8, data: u16) -> VL53L0X_Error {
    0
}

#[no_mangle]
pub extern "C" fn VL53L0X_UpdateByte(
    Dev: VL53L0X_DEV,
    index: u8,
    AndData: u8,
    OrData: u8,
) -> VL53L0X_Error {
    0
}
