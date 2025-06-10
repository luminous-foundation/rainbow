use std::fmt::Display;

use crate::{FFIArray, FFIString};

#[derive(Debug)]
#[repr(C)]
pub struct CodeChunk {

}

#[derive(Debug)]
#[repr(C)]
pub struct ModuleChunk {

}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct DataIndex {
    pub chunk: usize,
    pub index: usize
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct RawFuncRef {
    pub module: Vec<DataIndex>,
    pub function: Vec<DataIndex>,
    pub name: DataIndex,
    pub is_extern: bool,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct RawStructRef {
    pub module: Vec<DataIndex>,
    pub function: Vec<DataIndex>,
    pub name: DataIndex,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum RawComplexType {
    uXX(DataIndex),
    iXX(DataIndex),
    fXX(DataIndex, DataIndex),
    Struct(DataIndex),
}

#[derive(Debug, Clone)]
#[repr(C)]
pub enum RawData {
    Number(Number),
    Text(String),
    Array(Vec<DataIndex>),
    FuncRef(RawFuncRef),
    StructRef(RawStructRef),
    ComplexType(RawComplexType),
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct RawDataChunk {
    pub data: Vec<RawData>,
}

#[derive(Debug)]
#[repr(C)]
pub enum Number {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    UXX(FFIArray<u8>),

    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    IXX(FFIArray<u8>),

    F8(u8),
    F16(u16), // just make it a u16 for now...
    F32(f32),
    F64(f64),
    FXX(FFIArray<u8>, u64, u64),
}

impl Clone for Number {
    fn clone(&self) -> Self {
        match self {
            Number::U8(n)  => Number::U8(*n),
            Number::U16(n) => Number::U16(*n),
            Number::U32(n) => Number::U32(*n),
            Number::U64(n) => Number::U64(*n),
            Number::UXX(n) => {
                let mut clone = Vec::new();
                for i in 0..n.len {
                    clone.push(unsafe { *n.data.add(i) });
                }
                Number::UXX(clone.into())
            }

            Number::I8(n)  => Number::I8(*n),
            Number::I16(n) => Number::I16(*n),
            Number::I32(n) => Number::I32(*n),
            Number::I64(n) => Number::I64(*n),
            Number::IXX(n) => {
                let mut clone = Vec::new();
                for i in 0..n.len {
                    clone.push(unsafe { *n.data.add(i) });
                }
                Number::IXX(clone.into())
            }

            Number::F8(n)  => Number::F8(*n),
            Number::F16(n) => Number::F16(*n),
            Number::F32(n) => Number::F32(*n),
            Number::F64(n) => Number::F64(*n),
            Number::FXX(n, e, m) => {
                let mut clone = Vec::new();
                for i in 0..n.len {
                    clone.push(unsafe { *n.data.add(i) });
                }
                Number::FXX(clone.into(), *e, *m)
            }
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::U8(n)  => write!(f, "u8 {n}"),
            Number::U16(n) => write!(f, "u16 {n}"),
            Number::U32(n) => write!(f, "u32 {n}"),
            Number::U64(n) => write!(f, "u64 {n}"),
            Number::UXX(n) => {
                write!(f, "uXX({}) 0x", n.len)?;

                for n in n {
                    write!(f, "{:X}", n)?;
                }

                Ok(())
            }

            Number::I8(n)  => write!(f, "i8 {n}"),
            Number::I16(n) => write!(f, "i16 {n}"),
            Number::I32(n) => write!(f, "i32 {n}"),
            Number::I64(n) => write!(f, "i64 {n}"),
            Number::IXX(n) => {
                write!(f, "iXX({}) 0x", n.len)?;

                for n in n {
                    write!(f, "{:X}", n)?;
                }

                Ok(())
            }

            Number::F8(n)  => write!(f, "f8 0x{:x}", n),
            Number::F16(n) => write!(f, "f16 0x{:x}", n),
            Number::F32(n) => write!(f, "f32 {n}"),
            Number::F64(n) => write!(f, "f64 {n}"),
            Number::FXX(n, e, m) => {
                write!(f, "fXX({e}, {m}) 0x")?;

                for n in n {
                    write!(f, "{:X}", n)?;
                }

                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct FuncRef {
    pub module: FFIArray<*const FFIString>,
    pub function: FFIArray<*const FFIString>,
    pub name: *const FFIString,
    pub is_extern: bool,
}

impl Display for FuncRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_extern {
            write!(f, "extern ")?;
        }

        for s in &self.module {
            write!(f, "{}.", unsafe { FFIString::to_string(s) })?;
        }

        for s in &self.function {
            write!(f, "{}.", unsafe { FFIString::to_string(s) })?;
        }

        write!(f, "{}", unsafe { FFIString::to_string(self.name) })
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct StructRef {
    pub module: FFIArray<*const FFIString>,
    pub function: FFIArray<*const FFIString>,
    pub name: *const FFIString,
}

impl Display for StructRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in &self.module {
            write!(f, "{}.", unsafe { FFIString::to_string(s) })?;
        }

        for s in &self.function {
            write!(f, "{}.", unsafe { FFIString::to_string(s) })?;
        }

        write!(f, "{}", unsafe { FFIString::to_string(self.name) })
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub enum Data {
    Number(Number),
    Text(*mut FFIString),
    Array(FFIArray<Data>),
    FuncRef(FuncRef),
    StructRef(StructRef),
    ComplexType,
}

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Number(n) => write!(f, "{n}"),
            Data::Text(s) => write!(f, "\"{}\"", unsafe { FFIString::to_string(*s as *const FFIString) }),
            Data::Array(d) => {
                write!(f, "[")?;

                for i in 0..d.len {
                    let val = &d[i];
                    write!(f, "{val}")?;

                    if i < d.len - 1 {
                        write!(f, ", ")?;
                    }
                }

                write!(f, "]")
            }
            Data::FuncRef(func) => write!(f, "{func}"),
            Data::StructRef(s) => write!(f, "{s}"),
            Data::ComplexType => write!(f, "complex type is todo :)"),
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct DataChunk {
    pub raw: RawDataChunk,
    pub data: FFIArray<Data>,
}

impl Display for DataChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        
        for i in 0..self.data.len {
            let val = &self.data[i];
            write!(f, "    {val}")?;

            if i < self.data.len - 1 {
                writeln!(f, ",")?;
            } else {
                writeln!(f, "")?;
            }
        }

        write!(f, "}}")
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct MetadataChunk {

}

#[derive(Debug)]
#[repr(C)]
pub struct TypeCastChunk {

}

#[derive(Debug)]
#[repr(C)]
pub struct ConditionalParsingChunk {

}

#[derive(Debug)]
#[repr(C)]
pub enum RuntimeConstant {
    Number(Number),
}

#[derive(Debug)]
#[repr(C)]
pub struct RuntimeConstantChunk {
    pub constants: FFIArray<RuntimeConstant>,
}

#[derive(Debug)]
#[repr(C)]
pub struct FileImportChunk {

}

