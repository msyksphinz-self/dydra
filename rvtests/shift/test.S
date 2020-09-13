_start:
    addi    x1, x0, 11
    lui     x10, 0xdeadb
    addi    x10, x10, 0x3ef

    srl     x15, x10, x1
    sll     x16, x10, x1
    sra     x17, x10, x1

    srli    x18, x10, 11
    slli    x19, x10, 11
    srai    x20, x10, 11

    slli    x21, x10, 32
    srai    x22, x21, 32
    srli    x23, x21, 32

    ret
