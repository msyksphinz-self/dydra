_start:
    addi    x1,  zero, 1
    addi    x2,  x1,  2
    addi    x3,  x2,  3
    addi    x4,  x3,  4
    addi    x5,  x4,  5
    addi    x6,  x5,  6
    addi    x7,  x6,  7
    addi    x8,  x7,  8
    addi    x9,  x8,  9
    addi    x10, x9,  10
    addi    x11, x10, 11
    addi    x12, x11, 12
    addi    x13, x12, 13
    addi    x14, x13, 14
    addi    x15, x14, 15
    addi    x16, x15, 16
    addi    x17, x16, 17
    addi    x18, x17, 18
    addi    x19, x18, 19
    addi    x20, x19, 20
    add     x21, x20, x19
    add     x22, x21, x20
    add     x23, x22, x21
    add     x24, x23, x22
    add     x25, x24, x23
    add     x26, x25, x24
    add     x27, x26, x25
    add     x28, x27, x26
    add     x29, x28, x27
    add     x30, x29, x28
    add     x31, x30, x29

    sub     x31, x30, x29

    ret
