use std::{ffi::{c_char, CStr, CString}, mem, ops::{Index, IndexMut}};

use chunks::{CodeChunk, ConditionalParsingChunk, DataChunk, FileImportChunk, MetadataChunk, ModuleChunk, RuntimeConstantChunk, TypeCastChunk};

pub mod chunks;

#[derive(Debug)]
#[repr(C)]
pub struct FFIString {
    data: *mut c_char,
    len: usize,
}

impl FFIString {
    #[no_mangle]
    pub unsafe extern "C" fn from_str(str: *const c_char) -> *mut FFIString {
        let c_str = CStr::from_ptr(str);
        let c_string = c_str.to_owned();
        
        Box::into_raw(Box::new(FFIString {
            data: c_string.into_raw(),
            len: c_str.to_bytes().len(),
        }))
    }

    pub fn from_string(str: String) -> *mut FFIString {
        let len = str.len();
        let c_string = CString::new(str).expect("somehow failed to create string");
        
        Box::into_raw(Box::new(FFIString {
            data: c_string.into_raw(),
            len, 
        }))
    }
    
    #[no_mangle]
    pub extern "C" fn free_string(ptr: *mut FFIString) {
        if ptr.is_null() {
            return;
        }
        unsafe {
            let ffi_str = Box::from_raw(ptr);
            let _ = CString::from_raw(ffi_str.data);
        }
    }

    #[no_mangle]
    pub unsafe fn to_string(ptr: *const FFIString) -> String {
        let ffi_str = &*ptr;
        CStr::from_ptr(ffi_str.data).to_string_lossy().into_owned()
    }
}

#[derive(Debug)]
#[repr(C)]
pub enum FFIResult<T> {
    Ok(T),
    Error(*mut FFIString)
}

#[derive(Debug)]
#[repr(C)]
pub enum FFIOption<T> {
    Some(T),
    None,
}

#[derive(Debug)]
#[repr(C)]
pub struct FFIArray<T> {
    data: *mut T,
    pub len: usize, // in elements
}

impl<T: Clone> Clone for FFIArray<T> {
    fn clone(&self) -> Self {
        let mut res = Vec::new();

        for i in 0..self.len {
            res.push(unsafe { (*self.data.add(i)).clone() });
        }

        let data = res.as_mut_ptr();
        mem::forget(res);

        FFIArray { data, len: self.len }
    }
}

impl<'a, T: Clone + 'a> IntoIterator for &'a FFIArray<T> {
    type IntoIter = FFIIterator<'a, T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        FFIIterator::new(self)
    }
}

#[derive(Debug)]
pub struct FFIIterator<'a, T> {
    arr: &'a FFIArray<T>,
    i: usize
}

impl<'a> FFIIterator<'a, ()> {
    pub fn new<T>(arr: &'a FFIArray<T>) -> FFIIterator<'a, T> {
        FFIIterator::<'a, T> { arr, i: 0 }
    }
}

impl<'a, T: Clone> Iterator for FFIIterator<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.arr.len {
            return None;
        }
        let res = Some(self.arr[self.i].clone());
        self.i += 1;
        res
    }
}

impl<T> From<Vec<T>> for FFIArray<T> {
    fn from(mut vec: Vec<T>) -> Self {
        let res = FFIArray { data: vec.as_mut_ptr(), len: vec.len() };
        mem::forget(vec);
        res
    }
}

impl<T> Into<Vec<T>> for FFIArray<T> {
    fn into(self) -> Vec<T> {
        unsafe { Vec::from_raw_parts(self.data, self.len, self.len) }
    }
}

impl<T> Index<usize> for FFIArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len {
            panic!("FFIArray index out of bounds, got {index} expected value less than {}", self.len);
        }

        unsafe { self.data.add(index).as_ref().expect("could not convert FFIArray access to ref") }
    }
}

impl<T> IndexMut<usize> for FFIArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { self.data.add(index).as_mut().expect("could not convert FFIArray access to mut") }
    }
}

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
pub enum Chunk {
    Code(CodeChunk),
    Module(ModuleChunk),
    Data(DataChunk),
    Metadata(MetadataChunk),
    TypeCast(TypeCastChunk),
    ConditionalParsing(ConditionalParsingChunk),
    RuntimeConstant(RuntimeConstantChunk),
    FileImport(FileImportChunk),
}

#[derive(Debug)]
#[repr(C)]
pub struct Program {
    pub header: Header,

    pub body: FFIArray<Chunk>,

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
