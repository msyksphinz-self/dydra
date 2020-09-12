    .section    .text

_start:
    lui     x10, %hi(data_region)
    addi    x10, x10, %lo(data_region)

    ld      x11, 0(x10)
    lw      x12, 4(x10)
    lh      x13, 6(x10)
    lb      x14, 7(x10)

    lw      x16, 0(x10)
    lh      x17, 0(x10)
    lb      x18, 0(x10)

    lwu     x20, 4(x10)
    lhu     x21, 6(x10)
    lbu     x22, 7(x10)

    lwu     x20, 0(x10)
    lhu     x21, 0(x10)
    lbu     x22, 0(x10)

    sd      zero, 16(x10)
    sd      zero, 24(x10)
    sd      zero, 32(x10)
    sd      zero, 40(x10)

    sd      x11, 16(x10)

    sw      x12, 24(x10)
    sw      x12, 28(x10)

    sh      x13, 34(x10)
    sh      x13, 38(x10)

    sb      x14, 41(x10)
    sb      x14, 43(x10)
    sb      x14, 45(x10)
    sb      x14, 47(x10)

    ld      x25, 16(x10)
    ld      x26, 24(x10)
    ld      x27, 32(x10)
    ld      x28, 40(x10)

    ret

#    .section    .data
data_region:
    .word   0xdeadbeef
    .word   0x01234567
.rept   100
    nop
.endr
