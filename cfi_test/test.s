.text
.globl foo
.type foo, @function
foo:
    .cfi_startproc
    addi.d $sp, $sp, -16
    .cfi_def_cfa_offset 16
    st.d $ra, $sp, 8
    .cfi_offset 1, 8 // only .cfi_offset 1, 8 will pass the llvm-mc compilation
    .cfi_endproc
    ret