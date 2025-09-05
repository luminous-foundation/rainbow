use crate::chunks::{DataIndex, RawType};

#[derive(Debug, Clone)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum RawInstruction {
    // just nop
    NOP,

    // variables
    VAR_T_N(RawType,   DataIndex),
    VAR_V_N(DataIndex, DataIndex),
    VAR_S_N(           DataIndex),
    VAR_T_V(RawType,   DataIndex),
    VAR_V_V(DataIndex, DataIndex),
    VAR_S_V(           DataIndex),
    VAR_T_S(RawType             ),
    VAR_V_S(DataIndex           ),
    VAR_S_S,
}
