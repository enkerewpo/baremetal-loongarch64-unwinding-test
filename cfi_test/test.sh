llvm-mc -triple=loongarch64 -filetype=obj test.s -o test.o && \
llvm-dwarfdump --eh-frame test.o