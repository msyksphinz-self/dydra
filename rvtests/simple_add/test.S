_start:
    lui     x1, 0x12345
    add     x1, x1, 0x678
    lui     x2, 0x01234
    add     x2, x2, 0x567
    add     x3, x1, x2
    sub     x4, x1, x2
    and     x5, x1, x2
    or      x6, x1, x2
    xor     x7, x1, x2
    andi    x8, x1, 0x4bc
    ori     x9, x1, 0x4bc
    xori    x10, x1, 0x4bc
    ret
