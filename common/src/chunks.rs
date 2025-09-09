use std::{fmt::{Debug, Display}, hash::Hash};

use crate::{ffi::{FFIArray, FFIString}, instructions::{Instruction, RawInstruction}};

#[derive(Debug, Clone)]
#[repr(C)]
pub enum RawCodeBlock {
    Instructions(Vec<RawInstruction>),
    Scope(usize),
    Struct(RawStruct),
    Function(RawFunction),
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
    pub arg_types: FFIArray<RawType>,
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

#[derive(Debug, Clone)]
#[repr(C)]
pub enum RawMetadata {
    General(DataIndex, DataIndex),
    Byte(usize, usize, DataIndex),
    Element(usize, usize, DataIndex),
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct RawTypeCast {
    pub type_a: RawType,
    pub type_b: RawType,
    pub function: DataIndex,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
#[repr(C)]
pub enum Type {
    Void,
    U8,
    U16,
    U32,
    U64,
    UXX(u64),
    I8,
    I16,
    I32,
    I64,
    IXX(u64),
    F8,
    F16,
    F32,
    F64,
    FXX(u64, u64),
    Struct(StructRef),
    Name,
    Type,
    FuncRef,
    StructRef,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Void      => write!(f, "type void"),
            Type::U8        => write!(f, "type u8"),
            Type::U16       => write!(f, "type u16"),
            Type::U32       => write!(f, "type u32"),
            Type::U64       => write!(f, "type u64"),
            Type::UXX(s)    => write!(f, "type uXX({s})"),
            Type::I8        => write!(f, "type i8"),
            Type::I16       => write!(f, "type i16"),
            Type::I32       => write!(f, "type i32"),
            Type::I64       => write!(f, "type i64"),
            Type::IXX(s)    => write!(f, "type iXX({s})"),
            Type::F8        => write!(f, "type f8"),
            Type::F16       => write!(f, "type f16"),
            Type::F32       => write!(f, "type f32"),
            Type::F64       => write!(f, "type f64"),
            Type::FXX(e, m) => write!(f, "type fXX({e}, {m})"),
            Type::Struct(s) => write!(f, "type Struct({s})"),
            Type::Name      => write!(f, "type name"),
            Type::Type      => write!(f, "type type"),
            Type::FuncRef   => write!(f, "type funcref"),
            Type::StructRef => write!(f, "type structref"),
        }
    }
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
    FXX(DataIndex),
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
    pub default: DataIndex,
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

#[derive(Debug, Clone)]
#[repr(C)]
pub enum CodeBlock {
    Instructions(Vec<Instruction>),
    Scope(usize),
    Struct(Struct),
    Function(Function),
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct StructVar {
    pub typ: Type,
    pub name: *const FFIString,
    pub default: Data,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Struct {
    pub name: *const FFIString,
    pub variables: FFIArray<StructVar>,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Argument {
    pub typ: Type,
    pub name: *const FFIString,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Function {
    pub name: *const FFIString,
    pub ret_type: Type,
    pub args: FFIArray<Argument>,
    pub body: usize,
}

#[derive(Debug)]
#[repr(C)]
pub struct CodeChunk {
    /// Index of the raw version of this chunk
    pub raw_index: usize,

    pub has_parent: bool,
    pub blocks: FFIArray<CodeBlock>,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub enum Item {
    Function(FuncRef),
    Struct(StructRef),
    Variable(*const FFIString),
}

#[derive(Debug, Clone)]
#[repr(C)]
pub enum Import {
    FullImport {
        path: *const FFIString,
        parent_modules: FFIArray<*const FFIString>,
        name: *const FFIString,
        as_name: *const FFIString,
    },
    ItemImport {
        path: *const FFIString,
        parent_modules: FFIArray<*const FFIString>,
        name: *const FFIString,
        item: Item,
        as_name: *const FFIString,
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Extern {
    pub path: *const FFIString,
    pub name: *const FFIString,
    pub ret_type: Type,
    pub arg_types: FFIArray<Type>,
    pub as_name: *const FFIString,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub enum ModuleBlock {
    Submodule(FFIArray<usize>),
    Import(FFIArray<Import>),
    Export(FFIArray<Item>),
    Extern(FFIArray<Extern>),
}

#[derive(Debug)]
#[repr(C)]
pub struct ModuleChunk {
    /// Index of the raw version of this chunk
    pub raw_index: usize,

    pub name: *const FFIString,
    pub has_parent: bool,
    pub code_chunk: usize,

    pub blocks: FFIArray<ModuleBlock>,
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

#[derive(Debug, Clone, Eq)]
#[repr(C)]
pub struct StructRef {
    pub module: FFIArray<*const FFIString>,
    pub function: FFIArray<*const FFIString>,
    pub name: *const FFIString,
}

impl PartialEq for StructRef {
    fn eq(&self, other: &Self) -> bool {
        let self_module: Vec<String> = unsafe { std::slice::from_raw_parts(self.module.data, self.module.len) }.to_vec().iter().map(|s| unsafe { FFIString::to_string(*s) }).collect();
        let self_function: Vec<String> = unsafe { std::slice::from_raw_parts(self.function.data, self.function.len) }.to_vec().iter().map(|s| unsafe { FFIString::to_string(*s) }).collect();
        let self_name: String = unsafe { FFIString::to_string(self.name) };
        let other_module: Vec<String> = unsafe { std::slice::from_raw_parts(other.module.data, other.module.len) }.to_vec().iter().map(|s| unsafe { FFIString::to_string(*s) }).collect();
        let other_function: Vec<String> = unsafe { std::slice::from_raw_parts(other.function.data, other.function.len) }.to_vec().iter().map(|s| unsafe { FFIString::to_string(*s) }).collect();
        let other_name: String = unsafe { FFIString::to_string(other.name) };

        self_module == other_module && self_function == other_function && self_name == other_name
    }
}

impl Hash for StructRef {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let module: Vec<String> = unsafe { std::slice::from_raw_parts(self.module.data, self.module.len) }.to_vec().iter().map(|s| unsafe { FFIString::to_string(*s) }).collect();
        let function: Vec<String> = unsafe { std::slice::from_raw_parts(self.function.data, self.function.len) }.to_vec().iter().map(|s| unsafe { FFIString::to_string(*s) }).collect();
        let name: String = unsafe { FFIString::to_string(self.name) };
        module.hash(state);
        function.hash(state);
        name.hash(state);
    }
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
/// Data section chunk
///
/// Holds all constants, function references, struct references, and complex types in a program
pub struct DataChunk {
    /// Index of the raw version of this chunk
    pub raw_index: usize,

    /// Array of the data stored inside this chunk
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
pub enum Metadata {
    General(*const FFIString, *const FFIString),
    Byte(usize, usize, *const FFIString),
    Element(usize, usize, *const FFIString),
}

#[derive(Debug)]
#[repr(C)]
pub struct MetadataChunk {
    /// Index of the raw version of this chunk
    pub raw_index: usize,

    pub metadata: FFIArray<Metadata>,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct TypeCast {
    pub type_a: Type,
    pub type_b: Type,
    pub function: FuncRef,
}

#[derive(Debug)]
#[repr(C)]
pub struct TypeCastChunk {
    /// Index of the raw version of this chunk
    pub raw_index: usize,

    pub type_casts: FFIArray<TypeCast>,
}

#[derive(Debug)]
#[repr(C)]
pub struct ConditionalParsingChunk {
    /// Index of the raw version of this chunk
    pub raw_index: usize,

}

#[derive(Debug)]
#[repr(C)]
/// A runtime constant
///
/// Holds a value to be used for conditional parsing, and can be overriden through arguments passed to the execution module.
pub struct RuntimeConstant {
    /// Name of the runtime constant
    pub name: *const FFIString,
    /// Default value of the runtime constant
    pub default: Data,
}

#[derive(Debug)]
#[repr(C)]
/// Runtime constant chunk
///
/// Holds all runtime constants, which are used for conditional parsing
pub struct RuntimeConstantChunk {
    /// Index of the raw version of this chunk
    pub raw_index: usize,

    /// Array of the constants stored in this chunk
    pub constants: FFIArray<RuntimeConstant>,
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct FileImport {
    pub path: *const FFIString,
    pub internal_path: *const FFIString,
}

#[derive(Debug)]
#[repr(C)]
pub struct FileImportChunk {
    /// Index of the raw version of this chunk
    pub raw_index: usize,

    pub file_imports: FFIArray<FileImport>,
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
