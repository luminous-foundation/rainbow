use std::fmt::{Debug, Display};

use crate::{ffi::{FFIArray, FFIString}, instructions::RawInstruction};

#[derive(Debug, Clone)]
#[repr(C)]
pub enum RawCodeBlock {
    Instructions(Vec<RawInstruction>),
    Scope(usize),
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct RawStructVar {
    pub typ: RawType,
    pub name: DataIndex,
    pub default: DataIndex,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct RawStruct {
    pub name: DataIndex,
    pub variables: FFIArray<RawStructVar>,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct RawArgument {
    pub typ: RawType,
    pub name: DataIndex,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct RawFunction {
    pub name: DataIndex,
    pub ret_type: RawType,
    pub args: FFIArray<RawArgument>,
    pub body: usize,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct RawCodeChunk {
    pub has_parent: bool,

    pub blocks: FFIArray<RawCodeBlock>,

    pub structs: FFIArray<RawStruct>,
    pub functions: FFIArray<RawFunction>,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub enum RawImport {
    FullImport {
        path: DataIndex,
        parent_modules: FFIArray<DataIndex>,
        name: DataIndex,
        as_name: DataIndex,
    },
    ItemImport {
        path: DataIndex,
        parent_modules: FFIArray<DataIndex>,
        name: DataIndex,
        item: DataIndex,
        as_name: DataIndex,
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct RawExtern {
    pub path: DataIndex,
    pub name: DataIndex,
    pub ret_type: RawType,
    pub arg_types:FFIArray<RawType>,
    pub as_name: DataIndex,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub enum RawModuleBlock {
    Submodule(FFIArray<usize>),
    Import(FFIArray<RawImport>),
    Export(FFIArray<DataIndex>),
    Extern(FFIArray<RawExtern>),
}

#[derive(Debug)]
#[repr(C)]
pub struct RawModuleChunk {
    pub name: DataIndex,
    pub has_parent: bool,
    pub code_chunk: usize,

    pub blocks: FFIArray<RawModuleBlock>,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct DataIndex {
    pub chunk: usize,
    pub index: usize
}

impl Display for DataIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.chunk, self.index)
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct RawFuncRef {
    pub module: FFIArray<DataIndex>,
    pub function: FFIArray<DataIndex>,
    pub name: DataIndex,
    pub is_extern: bool,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct RawStructRef {
    pub module: FFIArray<DataIndex>,
    pub function: FFIArray<DataIndex>,
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

#[derive(Clone)]
#[repr(C)]
pub enum RawData {
    Number(Number),
    Text(*const FFIString),
    Array(FFIArray<DataIndex>),
    FuncRef(RawFuncRef),
    StructRef(RawStructRef),
    ComplexType(RawComplexType),
}

impl Debug for RawData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RawData::Number(n)      => write!(f, "{:?}", n),
            RawData::Text(t)        => write!(f, "\"{}\"", unsafe { FFIString::to_string(*t) }),
            RawData::Array(a)       => write!(f, "{:#?}", a),
            RawData::FuncRef(r)     => write!(f, "{:?}", r),
            RawData::StructRef(s)   => write!(f, "{:?}", s),
            RawData::ComplexType(t) => write!(f, "{:?}", t),
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub enum RawMetadata {
    General(DataIndex, DataIndex),
    Byte(usize, usize, DataIndex),
    Element(usize, usize, DataIndex),
}

#[derive(Debug)]
#[repr(C)]
pub struct RawTypeCast {
    pub type_a: RawType,
    pub type_b: RawType,
    pub function: DataIndex,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub enum RawType {
    Void,
    U8,
    U16,
    U32,
    U64,
    UXX(DataIndex),
    I8,
    I16,
    I32,
    I64,
    IXX(DataIndex),
    F8,
    F16,
    F32,
    F64,
    FXX(DataIndex, DataIndex),
    Struct(DataIndex),
    Name,
    Type,
    FuncRef,
    StructRef,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct RawRuntimeConstant {
    pub name: DataIndex,
    pub typ: RawType,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct RawFileImport {
    pub path: DataIndex,
    pub internal_path: DataIndex,
}

#[derive(Debug)]
#[repr(C)]
pub enum RawChunk {
    Code(RawCodeChunk),
    Module(RawModuleChunk),
    Data(FFIArray<RawData>),
    Metadata(FFIArray<RawMetadata>),
    TypeCast(FFIArray<RawTypeCast>),
    RuntimeConstant(FFIArray<RawRuntimeConstant>),
    FileImport(FFIArray<RawFileImport>),
}

#[derive(Debug)]
#[repr(C)]
pub struct CodeChunk {

}

#[derive(Debug)]
#[repr(C)]
pub struct ModuleChunk {

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

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
#[repr(C)]
pub enum ComplexType {
    uXX(u64),
    iXX(u64),
    fXX(u64, u64),
    Struct(StructRef),
}

impl Display for ComplexType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComplexType::uXX(s)    => write!(f, "type uXX({s})"),
            ComplexType::iXX(s)    => write!(f, "type iXX({s})"),
            ComplexType::fXX(e, m) => write!(f, "type fXX({e}, {m})"),
            ComplexType::Struct(s) => write!(f, "type Struct({s})"),
        }
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub enum Data {
    Number(Number),
    Text(*const FFIString),
    Array(FFIArray<Data>),
    FuncRef(FuncRef),
    StructRef(StructRef),
    ComplexType(ComplexType),
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
            Data::ComplexType(t) => write!(f, "{t}"),
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct DataChunk {
    pub raw: FFIArray<RawData>,
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
