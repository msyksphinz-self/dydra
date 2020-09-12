    .section    .text

_start:
    li      x10, 1
    li      x11, 1

    beq     x10, x11, label
    li      x20, 10
    li      x21, 11
    li      x22, 12
    li      x23, 13
    li      x24, 14
label:
#    li      x12, 0x10
#label:
#    li      x13, 0x11

    ret
