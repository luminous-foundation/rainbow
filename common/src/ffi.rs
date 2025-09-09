use std::{ffi::{c_char, CStr, CString}, fmt::Debug, hash::Hash, mem, ops::{Index, IndexMut}, ptr::copy_nonoverlapping};

#[repr(C)]
pub struct FFIString {
    data: *mut c_char,
    len: usize,
}

impl Debug for FFIString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", unsafe { FFIString::to_string(self as *const FFIString) })
    }
}

impl Clone for FFIString {
    fn clone(&self) -> Self {
        let mut new_mem = Vec::with_capacity(self.len);
        let new_data = new_mem.as_mut_ptr();
        mem::forget(new_mem);

        unsafe { copy_nonoverlapping(self.data, new_data, self.len) };

        FFIString { data: new_data, len: self.len }
    }
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

#[repr(C)]
pub struct FFIArray<T> {
    pub data: *mut T,
    pub len: usize, // in elements
}

impl<T: Clone + Hash> Hash for FFIArray<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let elements: Vec<T> = unsafe { std::slice::from_raw_parts(self.data, self.len) }.to_vec();
        elements.hash(state)
    }
}

impl<T: Clone + Eq> Eq for FFIArray<T> {}

impl<T: Clone + PartialEq> PartialEq for FFIArray<T> {
    fn eq(&self, other: &Self) -> bool {
        let self_: Vec<T> = unsafe { std::slice::from_raw_parts(self.data, self.len) }.to_vec();
        let other: Vec<T> = unsafe { std::slice::from_raw_parts(other.data, other.len) }.to_vec();
        self_ == other
    }
}

impl<T: Debug> Debug for FFIArray<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(unsafe { std::slice::from_raw_parts(self.data, self.len) })
            .finish()
    }
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
