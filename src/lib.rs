//! This crate provides a way to access Device electronic signature
//! items on STM32 microcontrollers.
//!
//! You need to pass one of the features in order to use this crate:
//! * `stm32f0`
//! * `stm32f1`
//! * `stm32f3`
//! * `stm32f4`
//! * `stm32g0`
//! * `stm32l0`
//! * `stm32l4`

#![no_std]

use core::slice;
use cortex_m::interrupt;

#[cfg(any(feature = "stm32f0", feature = "stm32f3"))]
mod pointers {
    pub const DEVICE_ID_PTR: *const u8 = 0x1FFF_F7AC as _;
    pub const FLASH_SIZE_PTR: *const u16 = 0x1FFF_F7CC as _;
}

#[cfg(feature = "stm32f1")]
mod pointers {
    pub const DEVICE_ID_PTR: *const u8 = 0x1FFF_F7E8 as _;
    pub const FLASH_SIZE_PTR: *const u16 = 0x1FFF_F7E0 as _;
}

#[cfg(feature = "stm32f4")]
mod pointers {
    pub const DEVICE_ID_PTR: *const u8 = 0x1FFF_7A10 as _;
    pub const FLASH_SIZE_PTR: *const u16 = 0x1FFF_7A22 as _;
}

#[cfg(any(feature = "stm32g0", feature = "stm32l4"))]
mod pointers {
    pub const DEVICE_ID_PTR: *const u8 = 0x1FFF_7590 as _;
    pub const FLASH_SIZE_PTR: *const u16 = 0x1FFF_75E0 as _;
}

#[cfg(feature = "stm32l0")]
mod pointers {
    pub const DEVICE_ID_PTR: *const u8 = 0x1FF8_0050 as _;
    pub const FLASH_SIZE_PTR: *const u16 = 0x1FF8_007C as _;
}

use pointers::*;


/// Returns a 12-byte slice with a unique device ID
pub fn device_id() -> &'static [u8] {
    unsafe { slice::from_raw_parts(DEVICE_ID_PTR, 12) }
}


/// Returns a string with a hex-encoded unique device ID
pub fn device_id_hex() -> &'static str {
    static mut DEVICE_ID_STR: [u8; 24] = [0; 24];

    unsafe {
        if DEVICE_ID_STR.as_ptr().read_volatile() == 0 {
            interrupt::free(|_| {
                let hex = "0123456789abcdef".as_bytes();
                let device_id = slice::from_raw_parts(DEVICE_ID_PTR, 12);
                for i in 0..12 {
                    let lo = device_id[i] & 0xf;
                    let hi = (device_id[i] >> 4) & 0xf;
                    DEVICE_ID_STR[i*2] = hex[lo as usize];
                    DEVICE_ID_STR[i*2+1] = hex[hi as usize];
                }
            });
        }

        core::str::from_utf8_unchecked(&DEVICE_ID_STR)
    }
}


/// Returns the Flash memory size of the device in Kbytes
pub fn flash_size_kb() -> u16 {
    unsafe { *FLASH_SIZE_PTR }
}
