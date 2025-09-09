use chunks::Chunk;
use ffi::{FFIArray, FFIOption};

use crate::chunks::RawChunk;

pub mod chunks;
pub mod ffi;
pub mod instructions;

#[derive(Debug)]
#[repr(C)]
pub struct Header {
    pub major_version: u16,
    pub minor_version: u16,
    pub patch_version: u16,

    pub checksum: u32,

    pub endianness: bool,

    pub compressed: bool,
    pub signed: bool,
    pub signature: FFIOption<()>, // TODO

    pub body_size: u64,
}

#[derive(Debug)]
#[repr(C)]
pub struct Program {
    pub header: Header,

    pub body: FFIArray<Chunk>,
    pub raw_body: FFIArray<RawChunk>,

    // TODO: write docs on this field
    pub extra: FFIArray<u8>, // exists to prevent DLL version hell
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModuleType {
    BEGINNING,
    MIDDLE,
    END
}
