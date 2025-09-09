use crate::{chunks::{DataIndex, FuncRef, Number, RawType, StructRef, Type}, ffi::FFIString};

#[derive(Debug, Clone)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum RawInstruction {
    /// Does nothing.
    NOP,

    /// Moves A into B.
    MOV_C_V(DataIndex, DataIndex),
    MOV_V_V(DataIndex, DataIndex),

    /// Pushes A onto the stack.
    /// Can cause stack overflow if the stack runs out of space, halting execution with an error.
    PUSH_C(DataIndex),
    PUSH_V(DataIndex),

    /// Pops a value off of the stack and puts it in A.
    /// Expects types to match.
    /// If the stack is empty execution will halt with an error.
    POP(DataIndex),

    /// Duplicates the element on the top of the stack.
    /// If the stack is empty execution will halt with an error.
    DUP,

    /// Swaps the top two elements on the stack.
    /// If the stack is empty execution will halt with an error.
    SWAP,

    /// Removes the element on the top of the stack.
    /// If the stack is empty execution will halt with an error.
    DROP,

    /// Allocates memory on the stack of type A with B elements and outputs to C.
    /// Expects C to be a pointer to elements of type A.
    /// Can cause stack overflow if the stack runs out of space and the allocation is too big, halting execution with an error.
    BUFF_T_C_V(RawType, DataIndex, DataIndex),
    BUFF_V_C_V(DataIndex, DataIndex, DataIndex),
    BUFF_S_C_V(DataIndex, DataIndex),
    BUFF_T_V_V(RawType, DataIndex, DataIndex),
    BUFF_V_V_V(DataIndex, DataIndex, DataIndex),
    BUFF_S_V_V(DataIndex, DataIndex),
    BUFF_T_S_V(RawType, DataIndex),
    BUFF_V_S_V(DataIndex, DataIndex),
    BUFF_S_S_V(DataIndex),
    BUFF_T_C_S(RawType, DataIndex),
    BUFF_V_C_S(DataIndex, DataIndex),
    BUFF_S_C_S(DataIndex),
    BUFF_T_V_S(RawType, DataIndex),
    BUFF_V_V_S(DataIndex, DataIndex),
    BUFF_S_V_S(DataIndex),
    BUFF_T_S_S(RawType),
    BUFF_V_S_S(DataIndex),
    BUFF_S_S_S,

    /// Copies A to B with size C.
    /// Size is in elements.
    /// Expects A and B to both point to the same type.
    MEMCOPY_V_V_C(DataIndex, DataIndex, DataIndex),
    MEMCOPY_S_V_C(DataIndex, DataIndex),
    MEMCOPY_V_S_C(DataIndex, DataIndex),
    MEMCOPY_S_S_C(DataIndex),
    MEMCOPY_V_V_S(DataIndex, DataIndex),
    MEMCOPY_S_V_S(DataIndex),
    MEMCOPY_V_S_S(DataIndex),
    MEMCOPY_S_S_S,

    /// Copies the value at A and outputs to B.
    /// Expects B to have the type that A points to.
    /// Expects A to be a pointer.
    DEREF_V_V(DataIndex, DataIndex),
    DEREF_S_V(DataIndex),
    DEREF_V_S(DataIndex),
    DEREF_S_S,

    /// Gets the address of A and outputs to B.
    /// Expects B to be a pointer to elements of type A.
    /// When [stack] is used it gets the current stack pointer.
    REF_V_V(DataIndex, DataIndex),
    REF_S_V(DataIndex),

    /// Moves A into the memory at B with offset C. (B[C] = A)
    /// C is in offset of elements of the type that B points to, as opposed to bytes.
    /// If you want to access something byte-wise cast B to a `void*` or `u8*`.
    /// Expects A to have the type that B points to.
    PMOV_C_V_C(DataIndex, DataIndex, DataIndex),
    PMOV_V_V_C(DataIndex, DataIndex, DataIndex),
    PMOV_S_V_C(DataIndex, DataIndex),
    PMOV_C_S_C(DataIndex, DataIndex),
    PMOV_V_S_C(DataIndex, DataIndex),
    PMOV_S_S_C(DataIndex),
    PMOV_C_V_V(DataIndex, DataIndex, DataIndex),
    PMOV_V_V_V(DataIndex, DataIndex, DataIndex),
    PMOV_S_V_V(DataIndex, DataIndex),
    PMOV_C_S_V(DataIndex, DataIndex),
    PMOV_V_S_V(DataIndex, DataIndex),
    PMOV_S_S_V(DataIndex),
    PMOV_C_V_S(DataIndex, DataIndex),
    PMOV_V_V_S(DataIndex, DataIndex),
    PMOV_S_V_S(DataIndex),
    PMOV_C_S_S(DataIndex),
    PMOV_V_S_S(DataIndex),
    PMOV_S_S_S,

    /// Gets a constant from the data section from chunk A at index B and outputs to C.
    /// Expects C to have the type of the constant.
    /// Different from 'MOV' as it can access any arbitrary value from the data section with the indices not known at parse time.
    DATA_C_C_V(DataIndex, DataIndex, DataIndex),
    DATA_V_C_V(DataIndex, DataIndex, DataIndex),
    DATA_S_C_V(DataIndex, DataIndex),
    DATA_C_V_V(DataIndex, DataIndex, DataIndex),
    DATA_V_V_V(DataIndex, DataIndex, DataIndex),
    DATA_S_V_V(DataIndex, DataIndex),
    DATA_C_S_V(DataIndex, DataIndex),
    DATA_V_S_V(DataIndex, DataIndex),
    DATA_S_S_V(DataIndex),
    DATA_C_C_S(DataIndex, DataIndex),
    DATA_V_C_S(DataIndex, DataIndex),
    DATA_S_C_S(DataIndex),
    DATA_C_V_S(DataIndex, DataIndex),
    DATA_V_V_S(DataIndex, DataIndex),
    DATA_S_V_S(DataIndex),
    DATA_C_S_S(DataIndex),
    DATA_V_S_S(DataIndex),
    DATA_S_S_S,

    /// Performs B + A and stores the output in C.
    /// Expects all types to match.
    ADD_C_C_V(DataIndex, DataIndex, DataIndex),
    ADD_V_C_V(DataIndex, DataIndex, DataIndex),
    ADD_S_C_V(DataIndex, DataIndex),
    ADD_C_V_V(DataIndex, DataIndex, DataIndex),
    ADD_V_V_V(DataIndex, DataIndex, DataIndex),
    ADD_S_V_V(DataIndex, DataIndex),
    ADD_C_S_V(DataIndex, DataIndex),
    ADD_V_S_V(DataIndex, DataIndex),
    ADD_S_S_V(DataIndex),
    ADD_C_C_S(DataIndex, DataIndex),
    ADD_V_C_S(DataIndex, DataIndex),
    ADD_S_C_S(DataIndex),
    ADD_C_V_S(DataIndex, DataIndex),
    ADD_V_V_S(DataIndex, DataIndex),
    ADD_S_V_S(DataIndex),
    ADD_C_S_S(DataIndex),
    ADD_V_S_S(DataIndex),
    ADD_S_S_S,

    /// Performs B - A and stores the output in C.
    /// Expects all types to match.
    SUB_C_C_V(DataIndex, DataIndex, DataIndex),
    SUB_V_C_V(DataIndex, DataIndex, DataIndex),
    SUB_S_C_V(DataIndex, DataIndex),
    SUB_C_V_V(DataIndex, DataIndex, DataIndex),
    SUB_V_V_V(DataIndex, DataIndex, DataIndex),
    SUB_S_V_V(DataIndex, DataIndex),
    SUB_C_S_V(DataIndex, DataIndex),
    SUB_V_S_V(DataIndex, DataIndex),
    SUB_S_S_V(DataIndex),
    SUB_C_C_S(DataIndex, DataIndex),
    SUB_V_C_S(DataIndex, DataIndex),
    SUB_S_C_S(DataIndex),
    SUB_C_V_S(DataIndex, DataIndex),
    SUB_V_V_S(DataIndex, DataIndex),
    SUB_S_V_S(DataIndex),
    SUB_C_S_S(DataIndex),
    SUB_V_S_S(DataIndex),
    SUB_S_S_S,

    /// Performs B * A and stores the output in C.
    /// Expects all types to match.
    MUL_C_C_V(DataIndex, DataIndex, DataIndex),
    MUL_V_C_V(DataIndex, DataIndex, DataIndex),
    MUL_S_C_V(DataIndex, DataIndex),
    MUL_C_V_V(DataIndex, DataIndex, DataIndex),
    MUL_V_V_V(DataIndex, DataIndex, DataIndex),
    MUL_S_V_V(DataIndex, DataIndex),
    MUL_C_S_V(DataIndex, DataIndex),
    MUL_V_S_V(DataIndex, DataIndex),
    MUL_S_S_V(DataIndex),
    MUL_C_C_S(DataIndex, DataIndex),
    MUL_V_C_S(DataIndex, DataIndex),
    MUL_S_C_S(DataIndex),
    MUL_C_V_S(DataIndex, DataIndex),
    MUL_V_V_S(DataIndex, DataIndex),
    MUL_S_V_S(DataIndex),
    MUL_C_S_S(DataIndex),
    MUL_V_S_S(DataIndex),
    MUL_S_S_S,

    /// Performs B / A and stores the output in C.
    /// Expects all types to match.
    DIV_C_C_V(DataIndex, DataIndex, DataIndex),
    DIV_V_C_V(DataIndex, DataIndex, DataIndex),
    DIV_S_C_V(DataIndex, DataIndex),
    DIV_C_V_V(DataIndex, DataIndex, DataIndex),
    DIV_V_V_V(DataIndex, DataIndex, DataIndex),
    DIV_S_V_V(DataIndex, DataIndex),
    DIV_C_S_V(DataIndex, DataIndex),
    DIV_V_S_V(DataIndex, DataIndex),
    DIV_S_S_V(DataIndex),
    DIV_C_C_S(DataIndex, DataIndex),
    DIV_V_C_S(DataIndex, DataIndex),
    DIV_S_C_S(DataIndex),
    DIV_C_V_S(DataIndex, DataIndex),
    DIV_V_V_S(DataIndex, DataIndex),
    DIV_S_V_S(DataIndex),
    DIV_C_S_S(DataIndex),
    DIV_V_S_S(DataIndex),
    DIV_S_S_S,

    /// Performs B % A and stores the output in C.
    /// Expects all types to match.
    /// Modulo on floating point numbers performed to IEEE 754 standards.
    MOD_C_C_V(DataIndex, DataIndex, DataIndex),
    MOD_V_C_V(DataIndex, DataIndex, DataIndex),
    MOD_S_C_V(DataIndex, DataIndex),
    MOD_C_V_V(DataIndex, DataIndex, DataIndex),
    MOD_V_V_V(DataIndex, DataIndex, DataIndex),
    MOD_S_V_V(DataIndex, DataIndex),
    MOD_C_S_V(DataIndex, DataIndex),
    MOD_V_S_V(DataIndex, DataIndex),
    MOD_S_S_V(DataIndex),
    MOD_C_C_S(DataIndex, DataIndex),
    MOD_V_C_S(DataIndex, DataIndex),
    MOD_S_C_S(DataIndex),
    MOD_C_V_S(DataIndex, DataIndex),
    MOD_V_V_S(DataIndex, DataIndex),
    MOD_S_V_S(DataIndex),
    MOD_C_S_S(DataIndex),
    MOD_V_S_S(DataIndex),
    MOD_S_S_S,

    /// Performs B & A and stores the output in C.
    /// Expects all types to be the same.
    AND_C_C_V(DataIndex, DataIndex, DataIndex),
    AND_V_C_V(DataIndex, DataIndex, DataIndex),
    AND_S_C_V(DataIndex, DataIndex),
    AND_C_V_V(DataIndex, DataIndex, DataIndex),
    AND_V_V_V(DataIndex, DataIndex, DataIndex),
    AND_S_V_V(DataIndex, DataIndex),
    AND_C_S_V(DataIndex, DataIndex),
    AND_V_S_V(DataIndex, DataIndex),
    AND_S_S_V(DataIndex),
    AND_C_C_S(DataIndex, DataIndex),
    AND_V_C_S(DataIndex, DataIndex),
    AND_S_C_S(DataIndex),
    AND_C_V_S(DataIndex, DataIndex),
    AND_V_V_S(DataIndex, DataIndex),
    AND_S_V_S(DataIndex),
    AND_C_S_S(DataIndex),
    AND_V_S_S(DataIndex),
    AND_S_S_S,

    /// Performs B & A and stores the output in C.
    /// Expects all types to be the same.
    OR_C_C_V(DataIndex, DataIndex, DataIndex),
    OR_V_C_V(DataIndex, DataIndex, DataIndex),
    OR_S_C_V(DataIndex, DataIndex),
    OR_C_V_V(DataIndex, DataIndex, DataIndex),
    OR_V_V_V(DataIndex, DataIndex, DataIndex),
    OR_S_V_V(DataIndex, DataIndex),
    OR_C_S_V(DataIndex, DataIndex),
    OR_V_S_V(DataIndex, DataIndex),
    OR_S_S_V(DataIndex),
    OR_C_C_S(DataIndex, DataIndex),
    OR_V_C_S(DataIndex, DataIndex),
    OR_S_C_S(DataIndex),
    OR_C_V_S(DataIndex, DataIndex),
    OR_V_V_S(DataIndex, DataIndex),
    OR_S_V_S(DataIndex),
    OR_C_S_S(DataIndex),
    OR_V_S_S(DataIndex),
    OR_S_S_S,

    /// Performs ~A and stores the output in B.
    /// Expects types to be the same.
    NOT_C_V(DataIndex, DataIndex),
    NOT_V_V(DataIndex, DataIndex),
    NOT_S_V(DataIndex),
    NOT_C_S(DataIndex),
    NOT_V_S(DataIndex),
    NOT_S_S,

    /// Performs B & A and stores the output in C.
    /// Expects all types to be the same.
    XOR_C_C_V(DataIndex, DataIndex, DataIndex),
    XOR_V_C_V(DataIndex, DataIndex, DataIndex),
    XOR_S_C_V(DataIndex, DataIndex),
    XOR_C_V_V(DataIndex, DataIndex, DataIndex),
    XOR_V_V_V(DataIndex, DataIndex, DataIndex),
    XOR_S_V_V(DataIndex, DataIndex),
    XOR_C_S_V(DataIndex, DataIndex),
    XOR_V_S_V(DataIndex, DataIndex),
    XOR_S_S_V(DataIndex),
    XOR_C_C_S(DataIndex, DataIndex),
    XOR_V_C_S(DataIndex, DataIndex),
    XOR_S_C_S(DataIndex),
    XOR_C_V_S(DataIndex, DataIndex),
    XOR_V_V_S(DataIndex, DataIndex),
    XOR_S_V_S(DataIndex),
    XOR_C_S_S(DataIndex),
    XOR_V_S_S(DataIndex),
    XOR_S_S_S,

    /// Performs B & A and stores the output in C.
    /// Expects all types to be the same.
    LSH_C_C_V(DataIndex, DataIndex, DataIndex),
    LSH_V_C_V(DataIndex, DataIndex, DataIndex),
    LSH_S_C_V(DataIndex, DataIndex),
    LSH_C_V_V(DataIndex, DataIndex, DataIndex),
    LSH_V_V_V(DataIndex, DataIndex, DataIndex),
    LSH_S_V_V(DataIndex, DataIndex),
    LSH_C_S_V(DataIndex, DataIndex),
    LSH_V_S_V(DataIndex, DataIndex),
    LSH_S_S_V(DataIndex),
    LSH_C_C_S(DataIndex, DataIndex),
    LSH_V_C_S(DataIndex, DataIndex),
    LSH_S_C_S(DataIndex),
    LSH_C_V_S(DataIndex, DataIndex),
    LSH_V_V_S(DataIndex, DataIndex),
    LSH_S_V_S(DataIndex),
    LSH_C_S_S(DataIndex),
    LSH_V_S_S(DataIndex),
    LSH_S_S_S,

    /// Performs B & A and stores the output in C.
    /// Expects all types to be the same.
    RSH_C_C_V(DataIndex, DataIndex, DataIndex),
    RSH_V_C_V(DataIndex, DataIndex, DataIndex),
    RSH_S_C_V(DataIndex, DataIndex),
    RSH_C_V_V(DataIndex, DataIndex, DataIndex),
    RSH_V_V_V(DataIndex, DataIndex, DataIndex),
    RSH_S_V_V(DataIndex, DataIndex),
    RSH_C_S_V(DataIndex, DataIndex),
    RSH_V_S_V(DataIndex, DataIndex),
    RSH_S_S_V(DataIndex),
    RSH_C_C_S(DataIndex, DataIndex),
    RSH_V_C_S(DataIndex, DataIndex),
    RSH_S_C_S(DataIndex),
    RSH_C_V_S(DataIndex, DataIndex),
    RSH_V_V_S(DataIndex, DataIndex),
    RSH_S_V_S(DataIndex),
    RSH_C_S_S(DataIndex),
    RSH_V_S_S(DataIndex),
    RSH_S_S_S,

    /// Adds an element offset B to pointer A and outputs to C.
    /// Expects B to be an integer, and for A and C to point to the same elements.
    /// Expects pointer alignment to match.
    PADD_C_C_V(DataIndex, DataIndex, DataIndex),
    PADD_V_C_V(DataIndex, DataIndex, DataIndex),
    PADD_S_C_V(DataIndex, DataIndex),
    PADD_C_V_V(DataIndex, DataIndex, DataIndex),
    PADD_V_V_V(DataIndex, DataIndex, DataIndex),
    PADD_S_V_V(DataIndex, DataIndex),
    PADD_C_S_V(DataIndex, DataIndex),
    PADD_V_S_V(DataIndex, DataIndex),
    PADD_S_S_V(DataIndex),
    PADD_C_C_S(DataIndex, DataIndex),
    PADD_V_C_S(DataIndex, DataIndex),
    PADD_S_C_S(DataIndex),
    PADD_C_V_S(DataIndex, DataIndex),
    PADD_V_V_S(DataIndex, DataIndex),
    PADD_S_V_S(DataIndex),
    PADD_C_S_S(DataIndex),
    PADD_V_S_S(DataIndex),
    PADD_S_S_S,

    /// Subtracts an element offset B to pointer A and outputs to C.
    /// Expects B to be an integer, and for A and C to point to the same elements.
    /// Expects pointer alignment to match.
    PSUB_C_C_V(DataIndex, DataIndex, DataIndex),
    PSUB_V_C_V(DataIndex, DataIndex, DataIndex),
    PSUB_S_C_V(DataIndex, DataIndex),
    PSUB_C_V_V(DataIndex, DataIndex, DataIndex),
    PSUB_V_V_V(DataIndex, DataIndex, DataIndex),
    PSUB_S_V_V(DataIndex, DataIndex),
    PSUB_C_S_V(DataIndex, DataIndex),
    PSUB_V_S_V(DataIndex, DataIndex),
    PSUB_S_S_V(DataIndex),
    PSUB_C_C_S(DataIndex, DataIndex),
    PSUB_V_C_S(DataIndex, DataIndex),
    PSUB_S_C_S(DataIndex),
    PSUB_C_V_S(DataIndex, DataIndex),
    PSUB_V_V_S(DataIndex, DataIndex),
    PSUB_S_V_S(DataIndex),
    PSUB_C_S_S(DataIndex),
    PSUB_V_S_S(DataIndex),
    PSUB_S_S_S,

    /// Gets the element difference of pointer A from pointer B and outputs to C. (B - A)
    /// Expects C to be an integer, and for A and C to point to the same elements.
    /// Expects pointer alignment to match.
    PDIFF_C_C_V(DataIndex, DataIndex, DataIndex),
    PDIFF_V_C_V(DataIndex, DataIndex, DataIndex),
    PDIFF_S_C_V(DataIndex, DataIndex),
    PDIFF_C_V_V(DataIndex, DataIndex, DataIndex),
    PDIFF_V_V_V(DataIndex, DataIndex, DataIndex),
    PDIFF_S_V_V(DataIndex, DataIndex),
    PDIFF_C_S_V(DataIndex, DataIndex),
    PDIFF_V_S_V(DataIndex, DataIndex),
    PDIFF_S_S_V(DataIndex),
    PDIFF_C_C_S(DataIndex, DataIndex),
    PDIFF_V_C_S(DataIndex, DataIndex),
    PDIFF_S_C_S(DataIndex),
    PDIFF_C_V_S(DataIndex, DataIndex),
    PDIFF_V_V_S(DataIndex, DataIndex),
    PDIFF_S_V_S(DataIndex),
    PDIFF_C_S_S(DataIndex),
    PDIFF_V_S_S(DataIndex),
    PDIFF_S_S_S,

    /// Creates a variable with `type` A and `name` B.
    /// Allocates onto the stack.
    /// Variables are automatically dropped once the scope they are created in ends.
    VAR_T_N(RawType, DataIndex),
    VAR_V_N(DataIndex, DataIndex),
    VAR_S_N(DataIndex),
    VAR_T_V(RawType, DataIndex),
    VAR_V_V(DataIndex, DataIndex),
    VAR_S_V(DataIndex),
    VAR_T_S(RawType),
    VAR_V_S(DataIndex),
    VAR_S_S,

    /// Checks if variable with `name` A exists. Sets the EQ flag to 1 if it does exist and to 0 if it doesn't.
    /// Useful for dynamically named variables.
    VAREXISTS_N(DataIndex),
    VAREXISTS_V(DataIndex),
    VAREXISTS_S,

    /// No description found.
    GETFIELD_INDEX_N_D_V,
    GETFIELD_INDEX_V_D_V,
    GETFIELD_INDEX_S_D_V,
    GETFIELD_INDEX_N_V_V,
    GETFIELD_INDEX_V_V_V,
    GETFIELD_INDEX_S_V_V,
    GETFIELD_INDEX_N_S_V,
    GETFIELD_INDEX_V_S_V,
    GETFIELD_INDEX_S_S_V,
    GETFIELD_INDEX_N_D_S,
    GETFIELD_INDEX_V_D_S,
    GETFIELD_INDEX_S_D_S,
    GETFIELD_INDEX_N_V_S,
    GETFIELD_INDEX_V_V_S,
    GETFIELD_INDEX_S_V_S,
    GETFIELD_INDEX_N_S_S,
    GETFIELD_INDEX_V_S_S,
    GETFIELD_INDEX_S_S_S,
    GETFIELD_VALUE_C_V_V,
    GETFIELD_VALUE_V_V_V,
    GETFIELD_VALUE_S_V_V,
    GETFIELD_VALUE_C_S_V,
    GETFIELD_VALUE_V_S_V,
    GETFIELD_VALUE_S_S_V,
    GETFIELD_VALUE_C_V_S,
    GETFIELD_VALUE_V_V_S,
    GETFIELD_VALUE_S_V_S,
    GETFIELD_VALUE_C_S_S,
    GETFIELD_VALUE_V_S_S,
    GETFIELD_VALUE_S_S_S,
    GETFIELD_OFFSET_N_D_V(DataIndex, DataIndex, DataIndex),
    GETFIELD_OFFSET_V_D_V(DataIndex, DataIndex, DataIndex),
    GETFIELD_OFFSET_S_D_V(DataIndex, DataIndex),
    GETFIELD_OFFSET_N_V_V(DataIndex, DataIndex, DataIndex),
    GETFIELD_OFFSET_V_V_V(DataIndex, DataIndex, DataIndex),
    GETFIELD_OFFSET_S_V_V(DataIndex, DataIndex),
    GETFIELD_OFFSET_N_S_V(DataIndex, DataIndex),
    GETFIELD_OFFSET_V_S_V(DataIndex, DataIndex),
    GETFIELD_OFFSET_S_S_V(DataIndex),
    GETFIELD_OFFSET_N_D_S(DataIndex, DataIndex),
    GETFIELD_OFFSET_V_D_S(DataIndex, DataIndex),
    GETFIELD_OFFSET_S_D_S(DataIndex),
    GETFIELD_OFFSET_N_V_S(DataIndex, DataIndex),
    GETFIELD_OFFSET_V_V_S(DataIndex, DataIndex),
    GETFIELD_OFFSET_S_V_S(DataIndex),
    GETFIELD_OFFSET_N_S_S(DataIndex),
    GETFIELD_OFFSET_V_S_S(DataIndex),
    GETFIELD_OFFSET_S_S_S,

    /// No description found.
    SETFIELD_C_V_C(DataIndex, DataIndex, DataIndex),
    SETFIELD_V_V_C(DataIndex, DataIndex, DataIndex),
    SETFIELD_S_V_C(DataIndex, DataIndex),
    SETFIELD_C_S_C(DataIndex, DataIndex),
    SETFIELD_V_S_C(DataIndex, DataIndex),
    SETFIELD_S_S_C(DataIndex),
    SETFIELD_C_V_V(DataIndex, DataIndex, DataIndex),
    SETFIELD_V_V_V(DataIndex, DataIndex, DataIndex),
    SETFIELD_S_V_V(DataIndex, DataIndex),
    SETFIELD_C_S_V(DataIndex, DataIndex),
    SETFIELD_V_S_V(DataIndex, DataIndex),
    SETFIELD_S_S_V(DataIndex),
    SETFIELD_C_V_S(DataIndex, DataIndex),
    SETFIELD_V_V_S(DataIndex, DataIndex),
    SETFIELD_S_V_S(DataIndex),
    SETFIELD_C_S_S(DataIndex),
    SETFIELD_V_S_S(DataIndex),
    SETFIELD_S_S_S,

    /// Calls function A.
    /// If a variable is passed, it expects it to be of type `funcref`.
    /// Externs are treated like normal functions, however the `funcref` must have the `extern` flag set.
    /// All arguments are passed through the stack.
    CALL_F(DataIndex),
    CALL_V(DataIndex),
    CALL_S,

    /// Calls syscall with id A.
    /// Arguments are fetched from the stack.
    SYSCALL_C(DataIndex),
    SYSCALL_V(DataIndex),
    SYSCALL_S,

    /// Returns value A, pushing it on to the stack of the function caller.
    /// If not inside of a function, return the value to the host.
    /// Expects A to match the return type of the parent function.
    RET,
    RET_C(DataIndex),
    RET_V(DataIndex),
    RET_S,

    /// Jumps to instruction B in code block A.
    /// Expects integer types.
    JMP_C_C(DataIndex, DataIndex),
    JMP_V_C(DataIndex, DataIndex),
    JMP_S_C(DataIndex),
    JMP_C_V(DataIndex, DataIndex),
    JMP_V_V(DataIndex, DataIndex),
    JMP_S_V(DataIndex),
    JMP_S,

    /// Compares A with B and sets flags accordingly.
    /// Expects types to match and be numeral.
    /// Comparisons for floating point numbers are done according to IEEE 754.
    CMP_C_C(DataIndex, DataIndex),
    CMP_V_C(DataIndex, DataIndex),
    CMP_S_C(DataIndex),
    CMP_C_V(DataIndex, DataIndex),
    CMP_V_V(DataIndex, DataIndex),
    CMP_S_V(DataIndex),
    CMP_C_S(DataIndex),
    CMP_V_S(DataIndex),
    CMP_S_S,

    /// Clears all flags set by CMP.
    CLR,

    /// Jumps to instruction B in code block A if the last CMP instruction resulted in EQ.
    /// Expects integer types.
    JE_C_C(DataIndex, DataIndex),
    JE_V_C(DataIndex, DataIndex),
    JE_S_C(DataIndex),
    JE_C_V(DataIndex, DataIndex),
    JE_V_V(DataIndex, DataIndex),
    JE_S_V(DataIndex),
    JE_C_S(DataIndex),
    JE_V_S(DataIndex),
    JE_S_S,

    /// Jumps to instruction B in code block A if the last CMP instruction resulted in NE.
    /// Expects integer types.
    JNE_C_C(DataIndex, DataIndex),
    JNE_V_C(DataIndex, DataIndex),
    JNE_S_C(DataIndex),
    JNE_C_V(DataIndex, DataIndex),
    JNE_V_V(DataIndex, DataIndex),
    JNE_S_V(DataIndex),
    JNE_C_S(DataIndex),
    JNE_V_S(DataIndex),
    JNE_S_S,

    /// Jumps to instruction B in code block A if the last CMP instruction resulted in LT.
    /// Expects integer types.
    JL_C_C(DataIndex, DataIndex),
    JL_V_C(DataIndex, DataIndex),
    JL_S_C(DataIndex),
    JL_C_V(DataIndex, DataIndex),
    JL_V_V(DataIndex, DataIndex),
    JL_S_V(DataIndex),
    JL_C_S(DataIndex),
    JL_V_S(DataIndex),
    JL_S_S,

    /// Jumps to instruction B in code block A if the last CMP instruction resulted in LE.
    /// Expects integer types.
    JLE_C_C(DataIndex, DataIndex),
    JLE_V_C(DataIndex, DataIndex),
    JLE_S_C(DataIndex),
    JLE_C_V(DataIndex, DataIndex),
    JLE_V_V(DataIndex, DataIndex),
    JLE_S_V(DataIndex),
    JLE_C_S(DataIndex),
    JLE_V_S(DataIndex),
    JLE_S_S,

    /// Jumps to instruction B in code block A if the last CMP instruction resulted in GT.
    /// Expects integer types.
    JG_C_C(DataIndex, DataIndex),
    JG_V_C(DataIndex, DataIndex),
    JG_S_C(DataIndex),
    JG_C_V(DataIndex, DataIndex),
    JG_V_V(DataIndex, DataIndex),
    JG_S_V(DataIndex),
    JG_C_S(DataIndex),
    JG_V_S(DataIndex),
    JG_S_S,

    /// Jumps to instruction B in code block A if the last CMP instruction resulted in GE.
    /// Expects integer types.
    JGE_C_C(DataIndex, DataIndex),
    JGE_V_C(DataIndex, DataIndex),
    JGE_S_C(DataIndex),
    JGE_C_V(DataIndex, DataIndex),
    JGE_V_V(DataIndex, DataIndex),
    JGE_S_V(DataIndex),
    JGE_C_S(DataIndex),
    JGE_V_S(DataIndex),
    JGE_S_S,

    /// Checks if A is 0 or 1. If A is 0, execution is halted and message B is displayed. If A is 1, execution continues normally.
    /// B is a constant containing an index to the data section containing UTF-8 encoded text.
    ASSERT_C_C(DataIndex, DataIndex),
    ASSERT_V_C(DataIndex, DataIndex),
    ASSERT_P_C(DataIndex),

    /// Casts B into type A and outputs into C.
    /// Expects C to have type A.
    /// Casts based on the Type Cast Table defined above.
    /// [var] and [pop] for type A expect a variable containing a `type` value.
    /// If the attempted cast is not present in the type casting table execution will halt with an error.
    CAST_T_V_V(RawType, DataIndex, DataIndex),
    CAST_V_V_V(DataIndex, DataIndex, DataIndex),
    CAST_S_V_V(DataIndex, DataIndex),
    CAST_T_S_V(RawType, DataIndex),
    CAST_V_S_V(DataIndex, DataIndex),
    CAST_S_S_V(DataIndex),
    CAST_T_V_S(RawType, DataIndex),
    CAST_V_V_S(DataIndex, DataIndex),
    CAST_S_V_S(DataIndex),
    CAST_T_S_S(RawType),
    CAST_V_S_S(DataIndex),
    CAST_S_S_S,

    /// Gets the type of A and outputs to B.
    /// Expects B to have the `type` type.
    TYPEOF_V_V(DataIndex, DataIndex),
    TYPEOF_S_V(DataIndex),
    TYPEOF_V_S(DataIndex),
    TYPEOF_S_S,

    /// No description found.
    TYPECMP_STRICT_C_C(DataIndex, DataIndex),
    TYPECMP_STRICT_V_C(DataIndex, DataIndex),
    TYPECMP_STRICT_S_C(DataIndex),
    TYPECMP_STRICT_C_V(DataIndex, DataIndex),
    TYPECMP_STRICT_V_V(DataIndex, DataIndex),
    TYPECMP_STRICT_S_V(DataIndex),
    TYPECMP_STRICT_C_S(DataIndex),
    TYPECMP_STRICT_V_S(DataIndex),
    TYPECMP_STRICT_S_S,
    TYPECMP_STRUCT_C_C(DataIndex, DataIndex),
    TYPECMP_STRUCT_V_C(DataIndex, DataIndex),
    TYPECMP_STRUCT_S_C(DataIndex),
    TYPECMP_STRUCT_C_V(DataIndex, DataIndex),
    TYPECMP_STRUCT_V_V(DataIndex, DataIndex),
    TYPECMP_STRUCT_S_V(DataIndex),
    TYPECMP_STRUCT_C_S(DataIndex),
    TYPECMP_STRUCT_V_S(DataIndex),
    TYPECMP_STRUCT_S_S,
    TYPECMP_LOOSE_C_C,
    TYPECMP_LOOSE_V_C,
    TYPECMP_LOOSE_S_C,
    TYPECMP_LOOSE_C_V,
    TYPECMP_LOOSE_V_V,
    TYPECMP_LOOSE_S_V,
    TYPECMP_LOOSE_C_S,
    TYPECMP_LOOSE_V_S,
    TYPECMP_LOOSE_S_S,

    /// No description found.
    SIZEOF_TYPE_T_V(RawType, DataIndex),
    SIZEOF_TYPE_V_V(DataIndex, DataIndex),
    SIZEOF_TYPE_S_V(DataIndex),
    SIZEOF_TYPE_T_S(RawType),
    SIZEOF_TYPE_V_S(DataIndex),
    SIZEOF_TYPE_S_S,
    SIZEOF_VAR_V_V,
    SIZEOF_VAR_S_V,
    SIZEOF_VAR_V_S,
    SIZEOF_VAR_S_S,

    /// No description found.
    GENTYPE_CREATE_C_V,
    GENTYPE_CREATE_V_V,
    GENTYPE_CREATE_S_V,
    GENTYPE_CREATE_C_S,
    GENTYPE_CREATE_V_S,
    GENTYPE_CREATE_S_S,
    GENTYPE_MODIFY_C_C_V(u8, DataIndex, DataIndex),
    GENTYPE_MODIFY_V_C_V(DataIndex, DataIndex, DataIndex),
    GENTYPE_MODIFY_S_C_V(DataIndex, DataIndex),
    GENTYPE_MODIFY_C_V_V(u8, DataIndex, DataIndex),
    GENTYPE_MODIFY_V_V_V(DataIndex, DataIndex, DataIndex),
    GENTYPE_MODIFY_S_V_V(DataIndex, DataIndex),
    GENTYPE_MODIFY_C_S_V(u8, DataIndex),
    GENTYPE_MODIFY_V_S_V(DataIndex, DataIndex),
    GENTYPE_MODIFY_S_S_V(DataIndex),
    GENTYPE_MODIFY_C_C_S(u8, DataIndex),
    GENTYPE_MODIFY_V_C_S(DataIndex, DataIndex),
    GENTYPE_MODIFY_S_C_S(DataIndex),
    GENTYPE_MODIFY_C_V_S(u8, DataIndex),
    GENTYPE_MODIFY_V_V_S(DataIndex, DataIndex),
    GENTYPE_MODIFY_S_V_S(DataIndex),
    GENTYPE_MODIFY_C_S_S(u8),
    GENTYPE_MODIFY_V_S_S(DataIndex),
    GENTYPE_MODIFY_S_S_S,
}

#[derive(Debug, Clone)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub enum Instruction {
    /// Does nothing.
    NOP,

    /// Moves A into B.
    MOV_C_V(Number, *const FFIString),
    MOV_V_V(*const FFIString, *const FFIString),

    /// Pushes A onto the stack.
    /// Can cause stack overflow if the stack runs out of space, halting execution with an error.
    PUSH_C(Number),
    PUSH_V(*const FFIString),

    /// Pops a value off of the stack and puts it in A.
    /// Expects types to match.
    /// If the stack is empty execution will halt with an error.
    POP(*const FFIString),

    /// Duplicates the element on the top of the stack.
    /// If the stack is empty execution will halt with an error.
    DUP,

    /// Swaps the top two elements on the stack.
    /// If the stack is empty execution will halt with an error.
    SWAP,

    /// Removes the element on the top of the stack.
    /// If the stack is empty execution will halt with an error.
    DROP,

    /// Allocates memory on the stack of type A with B elements and outputs to C.
    /// Expects C to be a pointer to elements of type A.
    /// Can cause stack overflow if the stack runs out of space and the allocation is too big, halting execution with an error.
    BUFF_T_C_V(Type, Number, *const FFIString),
    BUFF_V_C_V(*const FFIString, Number, *const FFIString),
    BUFF_S_C_V(Number, *const FFIString),
    BUFF_T_V_V(Type, *const FFIString, *const FFIString),
    BUFF_V_V_V(*const FFIString, *const FFIString, *const FFIString),
    BUFF_S_V_V(*const FFIString, *const FFIString),
    BUFF_T_S_V(Type, *const FFIString),
    BUFF_V_S_V(*const FFIString, *const FFIString),
    BUFF_S_S_V(*const FFIString),
    BUFF_T_C_S(Type, Number),
    BUFF_V_C_S(*const FFIString, Number),
    BUFF_S_C_S(Number),
    BUFF_T_V_S(Type, *const FFIString),
    BUFF_V_V_S(*const FFIString, *const FFIString),
    BUFF_S_V_S(*const FFIString),
    BUFF_T_S_S(Type),
    BUFF_V_S_S(*const FFIString),
    BUFF_S_S_S,

    /// Copies A to B with size C.
    /// Size is in elements.
    /// Expects A and B to both point to the same type.
    MEMCOPY_V_V_C(*const FFIString, *const FFIString, Number),
    MEMCOPY_S_V_C(*const FFIString, Number),
    MEMCOPY_V_S_C(*const FFIString, Number),
    MEMCOPY_S_S_C(Number),
    MEMCOPY_V_V_S(*const FFIString, *const FFIString),
    MEMCOPY_S_V_S(*const FFIString),
    MEMCOPY_V_S_S(*const FFIString),
    MEMCOPY_S_S_S,

    /// Copies the value at A and outputs to B.
    /// Expects B to have the type that A points to.
    /// Expects A to be a pointer.
    DEREF_V_V(*const FFIString, *const FFIString),
    DEREF_S_V(*const FFIString),
    DEREF_V_S(*const FFIString),
    DEREF_S_S,

    /// Gets the address of A and outputs to B.
    /// Expects B to be a pointer to elements of type A.
    /// When [stack] is used it gets the current stack pointer.
    REF_V_V(*const FFIString, *const FFIString),
    REF_S_V(*const FFIString),

    /// Moves A into the memory at B with offset C. (B[C] = A)
    /// C is in offset of elements of the type that B points to, as opposed to bytes.
    /// If you want to access something byte-wise cast B to a `void*` or `u8*`.
    /// Expects A to have the type that B points to.
    PMOV_C_V_C(Number, *const FFIString, Number),
    PMOV_V_V_C(*const FFIString, *const FFIString, Number),
    PMOV_S_V_C(*const FFIString, Number),
    PMOV_C_S_C(Number, Number),
    PMOV_V_S_C(*const FFIString, Number),
    PMOV_S_S_C(Number),
    PMOV_C_V_V(Number, *const FFIString, *const FFIString),
    PMOV_V_V_V(*const FFIString, *const FFIString, *const FFIString),
    PMOV_S_V_V(*const FFIString, *const FFIString),
    PMOV_C_S_V(Number, *const FFIString),
    PMOV_V_S_V(*const FFIString, *const FFIString),
    PMOV_S_S_V(*const FFIString),
    PMOV_C_V_S(Number, *const FFIString),
    PMOV_V_V_S(*const FFIString, *const FFIString),
    PMOV_S_V_S(*const FFIString),
    PMOV_C_S_S(Number),
    PMOV_V_S_S(*const FFIString),
    PMOV_S_S_S,

    /// Gets a constant from the data section from chunk A at index B and outputs to C.
    /// Expects C to have the type of the constant.
    /// Different from 'MOV' as it can access any arbitrary value from the data section with the indices not known at parse time.
    DATA_C_C_V(Number, Number, *const FFIString),
    DATA_V_C_V(*const FFIString, Number, *const FFIString),
    DATA_S_C_V(Number, *const FFIString),
    DATA_C_V_V(Number, *const FFIString, *const FFIString),
    DATA_V_V_V(*const FFIString, *const FFIString, *const FFIString),
    DATA_S_V_V(*const FFIString, *const FFIString),
    DATA_C_S_V(Number, *const FFIString),
    DATA_V_S_V(*const FFIString, *const FFIString),
    DATA_S_S_V(*const FFIString),
    DATA_C_C_S(Number, Number),
    DATA_V_C_S(*const FFIString, Number),
    DATA_S_C_S(Number),
    DATA_C_V_S(Number, *const FFIString),
    DATA_V_V_S(*const FFIString, *const FFIString),
    DATA_S_V_S(*const FFIString),
    DATA_C_S_S(Number),
    DATA_V_S_S(*const FFIString),
    DATA_S_S_S,

    /// Performs B + A and stores the output in C.
    /// Expects all types to match.
    ADD_C_C_V(Number, Number, *const FFIString),
    ADD_V_C_V(*const FFIString, Number, *const FFIString),
    ADD_S_C_V(Number, *const FFIString),
    ADD_C_V_V(Number, *const FFIString, *const FFIString),
    ADD_V_V_V(*const FFIString, *const FFIString, *const FFIString),
    ADD_S_V_V(*const FFIString, *const FFIString),
    ADD_C_S_V(Number, *const FFIString),
    ADD_V_S_V(*const FFIString, *const FFIString),
    ADD_S_S_V(*const FFIString),
    ADD_C_C_S(Number, Number),
    ADD_V_C_S(*const FFIString, Number),
    ADD_S_C_S(Number),
    ADD_C_V_S(Number, *const FFIString),
    ADD_V_V_S(*const FFIString, *const FFIString),
    ADD_S_V_S(*const FFIString),
    ADD_C_S_S(Number),
    ADD_V_S_S(*const FFIString),
    ADD_S_S_S,

    /// Performs B - A and stores the output in C.
    /// Expects all types to match.
    SUB_C_C_V(Number, Number, *const FFIString),
    SUB_V_C_V(*const FFIString, Number, *const FFIString),
    SUB_S_C_V(Number, *const FFIString),
    SUB_C_V_V(Number, *const FFIString, *const FFIString),
    SUB_V_V_V(*const FFIString, *const FFIString, *const FFIString),
    SUB_S_V_V(*const FFIString, *const FFIString),
    SUB_C_S_V(Number, *const FFIString),
    SUB_V_S_V(*const FFIString, *const FFIString),
    SUB_S_S_V(*const FFIString),
    SUB_C_C_S(Number, Number),
    SUB_V_C_S(*const FFIString, Number),
    SUB_S_C_S(Number),
    SUB_C_V_S(Number, *const FFIString),
    SUB_V_V_S(*const FFIString, *const FFIString),
    SUB_S_V_S(*const FFIString),
    SUB_C_S_S(Number),
    SUB_V_S_S(*const FFIString),
    SUB_S_S_S,

    /// Performs B * A and stores the output in C.
    /// Expects all types to match.
    MUL_C_C_V(Number, Number, *const FFIString),
    MUL_V_C_V(*const FFIString, Number, *const FFIString),
    MUL_S_C_V(Number, *const FFIString),
    MUL_C_V_V(Number, *const FFIString, *const FFIString),
    MUL_V_V_V(*const FFIString, *const FFIString, *const FFIString),
    MUL_S_V_V(*const FFIString, *const FFIString),
    MUL_C_S_V(Number, *const FFIString),
    MUL_V_S_V(*const FFIString, *const FFIString),
    MUL_S_S_V(*const FFIString),
    MUL_C_C_S(Number, Number),
    MUL_V_C_S(*const FFIString, Number),
    MUL_S_C_S(Number),
    MUL_C_V_S(Number, *const FFIString),
    MUL_V_V_S(*const FFIString, *const FFIString),
    MUL_S_V_S(*const FFIString),
    MUL_C_S_S(Number),
    MUL_V_S_S(*const FFIString),
    MUL_S_S_S,

    /// Performs B / A and stores the output in C.
    /// Expects all types to match.
    DIV_C_C_V(Number, Number, *const FFIString),
    DIV_V_C_V(*const FFIString, Number, *const FFIString),
    DIV_S_C_V(Number, *const FFIString),
    DIV_C_V_V(Number, *const FFIString, *const FFIString),
    DIV_V_V_V(*const FFIString, *const FFIString, *const FFIString),
    DIV_S_V_V(*const FFIString, *const FFIString),
    DIV_C_S_V(Number, *const FFIString),
    DIV_V_S_V(*const FFIString, *const FFIString),
    DIV_S_S_V(*const FFIString),
    DIV_C_C_S(Number, Number),
    DIV_V_C_S(*const FFIString, Number),
    DIV_S_C_S(Number),
    DIV_C_V_S(Number, *const FFIString),
    DIV_V_V_S(*const FFIString, *const FFIString),
    DIV_S_V_S(*const FFIString),
    DIV_C_S_S(Number),
    DIV_V_S_S(*const FFIString),
    DIV_S_S_S,

    /// Performs B % A and stores the output in C.
    /// Expects all types to match.
    /// Modulo on floating point numbers performed to IEEE 754 standards.
    MOD_C_C_V(Number, Number, *const FFIString),
    MOD_V_C_V(*const FFIString, Number, *const FFIString),
    MOD_S_C_V(Number, *const FFIString),
    MOD_C_V_V(Number, *const FFIString, *const FFIString),
    MOD_V_V_V(*const FFIString, *const FFIString, *const FFIString),
    MOD_S_V_V(*const FFIString, *const FFIString),
    MOD_C_S_V(Number, *const FFIString),
    MOD_V_S_V(*const FFIString, *const FFIString),
    MOD_S_S_V(*const FFIString),
    MOD_C_C_S(Number, Number),
    MOD_V_C_S(*const FFIString, Number),
    MOD_S_C_S(Number),
    MOD_C_V_S(Number, *const FFIString),
    MOD_V_V_S(*const FFIString, *const FFIString),
    MOD_S_V_S(*const FFIString),
    MOD_C_S_S(Number),
    MOD_V_S_S(*const FFIString),
    MOD_S_S_S,

    /// Performs B & A and stores the output in C.
    /// Expects all types to be the same.
    AND_C_C_V(Number, Number, *const FFIString),
    AND_V_C_V(*const FFIString, Number, *const FFIString),
    AND_S_C_V(Number, *const FFIString),
    AND_C_V_V(Number, *const FFIString, *const FFIString),
    AND_V_V_V(*const FFIString, *const FFIString, *const FFIString),
    AND_S_V_V(*const FFIString, *const FFIString),
    AND_C_S_V(Number, *const FFIString),
    AND_V_S_V(*const FFIString, *const FFIString),
    AND_S_S_V(*const FFIString),
    AND_C_C_S(Number, Number),
    AND_V_C_S(*const FFIString, Number),
    AND_S_C_S(Number),
    AND_C_V_S(Number, *const FFIString),
    AND_V_V_S(*const FFIString, *const FFIString),
    AND_S_V_S(*const FFIString),
    AND_C_S_S(Number),
    AND_V_S_S(*const FFIString),
    AND_S_S_S,

    /// Performs B & A and stores the output in C.
    /// Expects all types to be the same.
    OR_C_C_V(Number, Number, *const FFIString),
    OR_V_C_V(*const FFIString, Number, *const FFIString),
    OR_S_C_V(Number, *const FFIString),
    OR_C_V_V(Number, *const FFIString, *const FFIString),
    OR_V_V_V(*const FFIString, *const FFIString, *const FFIString),
    OR_S_V_V(*const FFIString, *const FFIString),
    OR_C_S_V(Number, *const FFIString),
    OR_V_S_V(*const FFIString, *const FFIString),
    OR_S_S_V(*const FFIString),
    OR_C_C_S(Number, Number),
    OR_V_C_S(*const FFIString, Number),
    OR_S_C_S(Number),
    OR_C_V_S(Number, *const FFIString),
    OR_V_V_S(*const FFIString, *const FFIString),
    OR_S_V_S(*const FFIString),
    OR_C_S_S(Number),
    OR_V_S_S(*const FFIString),
    OR_S_S_S,

    /// Performs ~A and stores the output in B.
    /// Expects types to be the same.
    NOT_C_V(Number, *const FFIString),
    NOT_V_V(*const FFIString, *const FFIString),
    NOT_S_V(*const FFIString),
    NOT_C_S(Number),
    NOT_V_S(*const FFIString),
    NOT_S_S,

    /// Performs B & A and stores the output in C.
    /// Expects all types to be the same.
    XOR_C_C_V(Number, Number, *const FFIString),
    XOR_V_C_V(*const FFIString, Number, *const FFIString),
    XOR_S_C_V(Number, *const FFIString),
    XOR_C_V_V(Number, *const FFIString, *const FFIString),
    XOR_V_V_V(*const FFIString, *const FFIString, *const FFIString),
    XOR_S_V_V(*const FFIString, *const FFIString),
    XOR_C_S_V(Number, *const FFIString),
    XOR_V_S_V(*const FFIString, *const FFIString),
    XOR_S_S_V(*const FFIString),
    XOR_C_C_S(Number, Number),
    XOR_V_C_S(*const FFIString, Number),
    XOR_S_C_S(Number),
    XOR_C_V_S(Number, *const FFIString),
    XOR_V_V_S(*const FFIString, *const FFIString),
    XOR_S_V_S(*const FFIString),
    XOR_C_S_S(Number),
    XOR_V_S_S(*const FFIString),
    XOR_S_S_S,

    /// Performs B & A and stores the output in C.
    /// Expects all types to be the same.
    LSH_C_C_V(Number, Number, *const FFIString),
    LSH_V_C_V(*const FFIString, Number, *const FFIString),
    LSH_S_C_V(Number, *const FFIString),
    LSH_C_V_V(Number, *const FFIString, *const FFIString),
    LSH_V_V_V(*const FFIString, *const FFIString, *const FFIString),
    LSH_S_V_V(*const FFIString, *const FFIString),
    LSH_C_S_V(Number, *const FFIString),
    LSH_V_S_V(*const FFIString, *const FFIString),
    LSH_S_S_V(*const FFIString),
    LSH_C_C_S(Number, Number),
    LSH_V_C_S(*const FFIString, Number),
    LSH_S_C_S(Number),
    LSH_C_V_S(Number, *const FFIString),
    LSH_V_V_S(*const FFIString, *const FFIString),
    LSH_S_V_S(*const FFIString),
    LSH_C_S_S(Number),
    LSH_V_S_S(*const FFIString),
    LSH_S_S_S,

    /// Performs B & A and stores the output in C.
    /// Expects all types to be the same.
    RSH_C_C_V(Number, Number, *const FFIString),
    RSH_V_C_V(*const FFIString, Number, *const FFIString),
    RSH_S_C_V(Number, *const FFIString),
    RSH_C_V_V(Number, *const FFIString, *const FFIString),
    RSH_V_V_V(*const FFIString, *const FFIString, *const FFIString),
    RSH_S_V_V(*const FFIString, *const FFIString),
    RSH_C_S_V(Number, *const FFIString),
    RSH_V_S_V(*const FFIString, *const FFIString),
    RSH_S_S_V(*const FFIString),
    RSH_C_C_S(Number, Number),
    RSH_V_C_S(*const FFIString, Number),
    RSH_S_C_S(Number),
    RSH_C_V_S(Number, *const FFIString),
    RSH_V_V_S(*const FFIString, *const FFIString),
    RSH_S_V_S(*const FFIString),
    RSH_C_S_S(Number),
    RSH_V_S_S(*const FFIString),
    RSH_S_S_S,

    /// Adds an element offset B to pointer A and outputs to C.
    /// Expects B to be an integer, and for A and C to point to the same elements.
    /// Expects pointer alignment to match.
    PADD_C_C_V(Number, Number, *const FFIString),
    PADD_V_C_V(*const FFIString, Number, *const FFIString),
    PADD_S_C_V(Number, *const FFIString),
    PADD_C_V_V(Number, *const FFIString, *const FFIString),
    PADD_V_V_V(*const FFIString, *const FFIString, *const FFIString),
    PADD_S_V_V(*const FFIString, *const FFIString),
    PADD_C_S_V(Number, *const FFIString),
    PADD_V_S_V(*const FFIString, *const FFIString),
    PADD_S_S_V(*const FFIString),
    PADD_C_C_S(Number, Number),
    PADD_V_C_S(*const FFIString, Number),
    PADD_S_C_S(Number),
    PADD_C_V_S(Number, *const FFIString),
    PADD_V_V_S(*const FFIString, *const FFIString),
    PADD_S_V_S(*const FFIString),
    PADD_C_S_S(Number),
    PADD_V_S_S(*const FFIString),
    PADD_S_S_S,

    /// Subtracts an element offset B to pointer A and outputs to C.
    /// Expects B to be an integer, and for A and C to point to the same elements.
    /// Expects pointer alignment to match.
    PSUB_C_C_V(Number, Number, *const FFIString),
    PSUB_V_C_V(*const FFIString, Number, *const FFIString),
    PSUB_S_C_V(Number, *const FFIString),
    PSUB_C_V_V(Number, *const FFIString, *const FFIString),
    PSUB_V_V_V(*const FFIString, *const FFIString, *const FFIString),
    PSUB_S_V_V(*const FFIString, *const FFIString),
    PSUB_C_S_V(Number, *const FFIString),
    PSUB_V_S_V(*const FFIString, *const FFIString),
    PSUB_S_S_V(*const FFIString),
    PSUB_C_C_S(Number, Number),
    PSUB_V_C_S(*const FFIString, Number),
    PSUB_S_C_S(Number),
    PSUB_C_V_S(Number, *const FFIString),
    PSUB_V_V_S(*const FFIString, *const FFIString),
    PSUB_S_V_S(*const FFIString),
    PSUB_C_S_S(Number),
    PSUB_V_S_S(*const FFIString),
    PSUB_S_S_S,

    /// Gets the element difference of pointer A from pointer B and outputs to C. (B - A)
    /// Expects C to be an integer, and for A and C to point to the same elements.
    /// Expects pointer alignment to match.
    PDIFF_C_C_V(Number, Number, *const FFIString),
    PDIFF_V_C_V(*const FFIString, Number, *const FFIString),
    PDIFF_S_C_V(Number, *const FFIString),
    PDIFF_C_V_V(Number, *const FFIString, *const FFIString),
    PDIFF_V_V_V(*const FFIString, *const FFIString, *const FFIString),
    PDIFF_S_V_V(*const FFIString, *const FFIString),
    PDIFF_C_S_V(Number, *const FFIString),
    PDIFF_V_S_V(*const FFIString, *const FFIString),
    PDIFF_S_S_V(*const FFIString),
    PDIFF_C_C_S(Number, Number),
    PDIFF_V_C_S(*const FFIString, Number),
    PDIFF_S_C_S(Number),
    PDIFF_C_V_S(Number, *const FFIString),
    PDIFF_V_V_S(*const FFIString, *const FFIString),
    PDIFF_S_V_S(*const FFIString),
    PDIFF_C_S_S(Number),
    PDIFF_V_S_S(*const FFIString),
    PDIFF_S_S_S,

    /// Creates a variable with `type` A and `name` B.
    /// Allocates onto the stack.
    /// Variables are automatically dropped once the scope they are created in ends.
    VAR_T_N(Type, *const FFIString),
    VAR_V_N(*const FFIString, *const FFIString),
    VAR_S_N(*const FFIString),
    VAR_T_V(Type, *const FFIString),
    VAR_V_V(*const FFIString, *const FFIString),
    VAR_S_V(*const FFIString),
    VAR_T_S(Type),
    VAR_V_S(*const FFIString),
    VAR_S_S,

    /// Checks if variable with `name` A exists. Sets the EQ flag to 1 if it does exist and to 0 if it doesn't.
    /// Useful for dynamically named variables.
    VAREXISTS_N(*const FFIString),
    VAREXISTS_V(*const FFIString),
    VAREXISTS_S,

    /// No description found.
    GETFIELD_INDEX_N_D_V,
    GETFIELD_INDEX_V_D_V,
    GETFIELD_INDEX_S_D_V,
    GETFIELD_INDEX_N_V_V,
    GETFIELD_INDEX_V_V_V,
    GETFIELD_INDEX_S_V_V,
    GETFIELD_INDEX_N_S_V,
    GETFIELD_INDEX_V_S_V,
    GETFIELD_INDEX_S_S_V,
    GETFIELD_INDEX_N_D_S,
    GETFIELD_INDEX_V_D_S,
    GETFIELD_INDEX_S_D_S,
    GETFIELD_INDEX_N_V_S,
    GETFIELD_INDEX_V_V_S,
    GETFIELD_INDEX_S_V_S,
    GETFIELD_INDEX_N_S_S,
    GETFIELD_INDEX_V_S_S,
    GETFIELD_INDEX_S_S_S,
    GETFIELD_VALUE_C_V_V,
    GETFIELD_VALUE_V_V_V,
    GETFIELD_VALUE_S_V_V,
    GETFIELD_VALUE_C_S_V,
    GETFIELD_VALUE_V_S_V,
    GETFIELD_VALUE_S_S_V,
    GETFIELD_VALUE_C_V_S,
    GETFIELD_VALUE_V_V_S,
    GETFIELD_VALUE_S_V_S,
    GETFIELD_VALUE_C_S_S,
    GETFIELD_VALUE_V_S_S,
    GETFIELD_VALUE_S_S_S,
    GETFIELD_OFFSET_N_D_V(*const FFIString, StructRef, *const FFIString),
    GETFIELD_OFFSET_V_D_V(*const FFIString, StructRef, *const FFIString),
    GETFIELD_OFFSET_S_D_V(StructRef, *const FFIString),
    GETFIELD_OFFSET_N_V_V(*const FFIString, *const FFIString, *const FFIString),
    GETFIELD_OFFSET_V_V_V(*const FFIString, *const FFIString, *const FFIString),
    GETFIELD_OFFSET_S_V_V(*const FFIString, *const FFIString),
    GETFIELD_OFFSET_N_S_V(*const FFIString, *const FFIString),
    GETFIELD_OFFSET_V_S_V(*const FFIString, *const FFIString),
    GETFIELD_OFFSET_S_S_V(*const FFIString),
    GETFIELD_OFFSET_N_D_S(*const FFIString, StructRef),
    GETFIELD_OFFSET_V_D_S(*const FFIString, StructRef),
    GETFIELD_OFFSET_S_D_S(StructRef),
    GETFIELD_OFFSET_N_V_S(*const FFIString, *const FFIString),
    GETFIELD_OFFSET_V_V_S(*const FFIString, *const FFIString),
    GETFIELD_OFFSET_S_V_S(*const FFIString),
    GETFIELD_OFFSET_N_S_S(*const FFIString),
    GETFIELD_OFFSET_V_S_S(*const FFIString),
    GETFIELD_OFFSET_S_S_S,

    /// No description found.
    SETFIELD_C_V_C(Number, *const FFIString, Number),
    SETFIELD_V_V_C(*const FFIString, *const FFIString, Number),
    SETFIELD_S_V_C(*const FFIString, Number),
    SETFIELD_C_S_C(Number, Number),
    SETFIELD_V_S_C(*const FFIString, Number),
    SETFIELD_S_S_C(Number),
    SETFIELD_C_V_V(Number, *const FFIString, *const FFIString),
    SETFIELD_V_V_V(*const FFIString, *const FFIString, *const FFIString),
    SETFIELD_S_V_V(*const FFIString, *const FFIString),
    SETFIELD_C_S_V(Number, *const FFIString),
    SETFIELD_V_S_V(*const FFIString, *const FFIString),
    SETFIELD_S_S_V(*const FFIString),
    SETFIELD_C_V_S(Number, *const FFIString),
    SETFIELD_V_V_S(*const FFIString, *const FFIString),
    SETFIELD_S_V_S(*const FFIString),
    SETFIELD_C_S_S(Number),
    SETFIELD_V_S_S(*const FFIString),
    SETFIELD_S_S_S,

    /// Calls function A.
    /// If a variable is passed, it expects it to be of type `funcref`.
    /// Externs are treated like normal functions, however the `funcref` must have the `extern` flag set.
    /// All arguments are passed through the stack.
    CALL_F(FuncRef),
    CALL_V(*const FFIString),
    CALL_S,

    /// Calls syscall with id A.
    /// Arguments are fetched from the stack.
    SYSCALL_C(Number),
    SYSCALL_V(*const FFIString),
    SYSCALL_S,

    /// Returns value A, pushing it on to the stack of the function caller.
    /// If not inside of a function, return the value to the host.
    /// Expects A to match the return type of the parent function.
    RET,
    RET_C(Number),
    RET_V(*const FFIString),
    RET_S,

    /// Jumps to instruction B in code block A.
    /// Expects integer types.
    JMP_C_C(Number, Number),
    JMP_V_C(*const FFIString, Number),
    JMP_S_C(Number),
    JMP_C_V(Number, *const FFIString),
    JMP_V_V(*const FFIString, *const FFIString),
    JMP_S_V(*const FFIString),
    JMP_S,

    /// Compares A with B and sets flags accordingly.
    /// Expects types to match and be numeral.
    /// Comparisons for floating point numbers are done according to IEEE 754.
    CMP_C_C(Number, Number),
    CMP_V_C(*const FFIString, Number),
    CMP_S_C(Number),
    CMP_C_V(Number, *const FFIString),
    CMP_V_V(*const FFIString, *const FFIString),
    CMP_S_V(*const FFIString),
    CMP_C_S(Number),
    CMP_V_S(*const FFIString),
    CMP_S_S,

    /// Clears all flags set by CMP.
    CLR,

    /// Jumps to instruction B in code block A if the last CMP instruction resulted in EQ.
    /// Expects integer types.
    JE_C_C(Number, Number),
    JE_V_C(*const FFIString, Number),
    JE_S_C(Number),
    JE_C_V(Number, *const FFIString),
    JE_V_V(*const FFIString, *const FFIString),
    JE_S_V(*const FFIString),
    JE_C_S(Number),
    JE_V_S(*const FFIString),
    JE_S_S,

    /// Jumps to instruction B in code block A if the last CMP instruction resulted in NE.
    /// Expects integer types.
    JNE_C_C(Number, Number),
    JNE_V_C(*const FFIString, Number),
    JNE_S_C(Number),
    JNE_C_V(Number, *const FFIString),
    JNE_V_V(*const FFIString, *const FFIString),
    JNE_S_V(*const FFIString),
    JNE_C_S(Number),
    JNE_V_S(*const FFIString),
    JNE_S_S,

    /// Jumps to instruction B in code block A if the last CMP instruction resulted in LT.
    /// Expects integer types.
    JL_C_C(Number, Number),
    JL_V_C(*const FFIString, Number),
    JL_S_C(Number),
    JL_C_V(Number, *const FFIString),
    JL_V_V(*const FFIString, *const FFIString),
    JL_S_V(*const FFIString),
    JL_C_S(Number),
    JL_V_S(*const FFIString),
    JL_S_S,

    /// Jumps to instruction B in code block A if the last CMP instruction resulted in LE.
    /// Expects integer types.
    JLE_C_C(Number, Number),
    JLE_V_C(*const FFIString, Number),
    JLE_S_C(Number),
    JLE_C_V(Number, *const FFIString),
    JLE_V_V(*const FFIString, *const FFIString),
    JLE_S_V(*const FFIString),
    JLE_C_S(Number),
    JLE_V_S(*const FFIString),
    JLE_S_S,

    /// Jumps to instruction B in code block A if the last CMP instruction resulted in GT.
    /// Expects integer types.
    JG_C_C(Number, Number),
    JG_V_C(*const FFIString, Number),
    JG_S_C(Number),
    JG_C_V(Number, *const FFIString),
    JG_V_V(*const FFIString, *const FFIString),
    JG_S_V(*const FFIString),
    JG_C_S(Number),
    JG_V_S(*const FFIString),
    JG_S_S,

    /// Jumps to instruction B in code block A if the last CMP instruction resulted in GE.
    /// Expects integer types.
    JGE_C_C(Number, Number),
    JGE_V_C(*const FFIString, Number),
    JGE_S_C(Number),
    JGE_C_V(Number, *const FFIString),
    JGE_V_V(*const FFIString, *const FFIString),
    JGE_S_V(*const FFIString),
    JGE_C_S(Number),
    JGE_V_S(*const FFIString),
    JGE_S_S,

    /// Checks if A is 0 or 1. If A is 0, execution is halted and message B is displayed. If A is 1, execution continues normally.
    /// B is a constant containing an index to the data section containing UTF-8 encoded text.
    ASSERT_C_C(Number, *const FFIString),
    ASSERT_V_C(*const FFIString, *const FFIString),
    ASSERT_P_C(*const FFIString),

    /// Casts B into type A and outputs into C.
    /// Expects C to have type A.
    /// Casts based on the Type Cast Table defined above.
    /// [var] and [pop] for type A expect a variable containing a `type` value.
    /// If the attempted cast is not present in the type casting table execution will halt with an error.
    CAST_T_V_V(Type, *const FFIString, *const FFIString),
    CAST_V_V_V(*const FFIString, *const FFIString, *const FFIString),
    CAST_S_V_V(*const FFIString, *const FFIString),
    CAST_T_S_V(Type, *const FFIString),
    CAST_V_S_V(*const FFIString, *const FFIString),
    CAST_S_S_V(*const FFIString),
    CAST_T_V_S(Type, *const FFIString),
    CAST_V_V_S(*const FFIString, *const FFIString),
    CAST_S_V_S(*const FFIString),
    CAST_T_S_S(Type),
    CAST_V_S_S(*const FFIString),
    CAST_S_S_S,

    /// Gets the type of A and outputs to B.
    /// Expects B to have the `type` type.
    TYPEOF_V_V(*const FFIString, *const FFIString),
    TYPEOF_S_V(*const FFIString),
    TYPEOF_V_S(*const FFIString),
    TYPEOF_S_S,

    /// No description found.
    TYPECMP_STRICT_C_C(Number, Number),
    TYPECMP_STRICT_V_C(*const FFIString, Number),
    TYPECMP_STRICT_S_C(Number),
    TYPECMP_STRICT_C_V(Number, *const FFIString),
    TYPECMP_STRICT_V_V(*const FFIString, *const FFIString),
    TYPECMP_STRICT_S_V(*const FFIString),
    TYPECMP_STRICT_C_S(Number),
    TYPECMP_STRICT_V_S(*const FFIString),
    TYPECMP_STRICT_S_S,
    TYPECMP_STRUCT_C_C(Number, Number),
    TYPECMP_STRUCT_V_C(*const FFIString, Number),
    TYPECMP_STRUCT_S_C(Number),
    TYPECMP_STRUCT_C_V(Number, *const FFIString),
    TYPECMP_STRUCT_V_V(*const FFIString, *const FFIString),
    TYPECMP_STRUCT_S_V(*const FFIString),
    TYPECMP_STRUCT_C_S(Number),
    TYPECMP_STRUCT_V_S(*const FFIString),
    TYPECMP_STRUCT_S_S,
    TYPECMP_LOOSE_C_C,
    TYPECMP_LOOSE_V_C,
    TYPECMP_LOOSE_S_C,
    TYPECMP_LOOSE_C_V,
    TYPECMP_LOOSE_V_V,
    TYPECMP_LOOSE_S_V,
    TYPECMP_LOOSE_C_S,
    TYPECMP_LOOSE_V_S,
    TYPECMP_LOOSE_S_S,

    /// No description found.
    SIZEOF_TYPE_T_V(Type, *const FFIString),
    SIZEOF_TYPE_V_V(*const FFIString, *const FFIString),
    SIZEOF_TYPE_S_V(*const FFIString),
    SIZEOF_TYPE_T_S(Type),
    SIZEOF_TYPE_V_S(*const FFIString),
    SIZEOF_TYPE_S_S,
    SIZEOF_VAR_V_V,
    SIZEOF_VAR_S_V,
    SIZEOF_VAR_V_S,
    SIZEOF_VAR_S_S,

    /// No description found.
    GENTYPE_CREATE_C_V,
    GENTYPE_CREATE_V_V,
    GENTYPE_CREATE_S_V,
    GENTYPE_CREATE_C_S,
    GENTYPE_CREATE_V_S,
    GENTYPE_CREATE_S_S,
    GENTYPE_MODIFY_C_C_V(u8, Number, *const FFIString),
    GENTYPE_MODIFY_V_C_V(*const FFIString, Number, *const FFIString),
    GENTYPE_MODIFY_S_C_V(Number, *const FFIString),
    GENTYPE_MODIFY_C_V_V(u8, *const FFIString, *const FFIString),
    GENTYPE_MODIFY_V_V_V(*const FFIString, *const FFIString, *const FFIString),
    GENTYPE_MODIFY_S_V_V(*const FFIString, *const FFIString),
    GENTYPE_MODIFY_C_S_V(u8, *const FFIString),
    GENTYPE_MODIFY_V_S_V(*const FFIString, *const FFIString),
    GENTYPE_MODIFY_S_S_V(*const FFIString),
    GENTYPE_MODIFY_C_C_S(u8, Number),
    GENTYPE_MODIFY_V_C_S(*const FFIString, Number),
    GENTYPE_MODIFY_S_C_S(Number),
    GENTYPE_MODIFY_C_V_S(u8, *const FFIString),
    GENTYPE_MODIFY_V_V_S(*const FFIString, *const FFIString),
    GENTYPE_MODIFY_S_V_S(*const FFIString),
    GENTYPE_MODIFY_C_S_S(u8),
    GENTYPE_MODIFY_V_S_S(*const FFIString),
    GENTYPE_MODIFY_S_S_S,
}
