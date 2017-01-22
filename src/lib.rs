//! Bindings to [ao][1].
//!
//! [1]: https://xiph.org/ao/

#![allow(non_camel_case_types)]

extern crate libc;

use libc::{c_char, c_int, uint32_t};

pub const AO_TYPE_LIVE: c_int = 1;
pub const AO_TYPE_FILE: c_int = 2;

pub const AO_ENODRIVER: c_int = 1;
pub const AO_ENOTFILE: c_int = 2;
pub const AO_ENOTLIVE: c_int = 3;
pub const AO_EBADOPTION: c_int = 4;
pub const AO_EOPENDEVICE: c_int = 5;
pub const AO_EOPENFILE: c_int = 6;
pub const AO_EFILEEXISTS: c_int = 7;
pub const AO_EBADFORMAT: c_int = 8;

pub const AO_EFAIL: c_int = 100;

pub const AO_FMT_LITTLE: c_int = 1;
pub const AO_FMT_BIG: c_int = 2;
pub const AO_FMT_NATIVE: c_int = 4;

pub enum ao_device {}

pub enum ao_functions {}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ao_info {
    pub _type: c_int,
    pub name: *mut c_char,
    pub short_name: *mut c_char,
    pub author: *mut c_char,
    pub comment: *mut c_char,
    pub preferred_byte_format: c_int,
    pub priority: c_int,
    pub options: *mut *mut c_char,
    pub option_count: c_int,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ao_option {
    pub key: *mut c_char,
    pub value: *mut c_char,
    pub next: *mut ao_option,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ao_sample_format {
    pub bits: c_int,
    pub rate: c_int,
    pub channels: c_int,
    pub byte_format: c_int,
    pub matrix: *mut c_char,
}

extern {
    pub fn ao_initialize();
    pub fn ao_shutdown();
    pub fn ao_append_global_option(key: *const c_char, value: *const c_char) -> c_int;
    pub fn ao_append_option(options: *mut *mut ao_option, key: *const c_char, value: *const c_char) -> c_int;
    pub fn ao_free_options(options: *mut ao_option);
    pub fn ao_open_live(driver_id: c_int, format: *mut ao_sample_format, option: *mut ao_option) -> *mut ao_device;
    pub fn ao_open_file(driver_id: c_int, filename: *const c_char, overwrite: c_int, format: *mut ao_sample_format, option: *mut ao_option) -> *mut ao_device;
    pub fn ao_play(device: *mut ao_device, output_samples: *mut c_char, num_bytes: uint32_t) -> c_int;
    pub fn ao_close(device: *mut ao_device) -> c_int;
    pub fn ao_driver_id(short_name: *const c_char) -> c_int;
    pub fn ao_default_driver_id() -> c_int;
    pub fn ao_driver_info(driver_id: c_int) -> *mut ao_info;
    pub fn ao_driver_info_list(driver_count: *mut c_int) -> *mut *mut ao_info;
    pub fn ao_file_extension(driver_id: c_int) -> *const c_char;
    pub fn ao_is_big_endian() -> c_int;
}
