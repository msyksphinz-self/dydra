    lui     x10, 0x12345
    addi    x10, x10, 0x678
    csrrw   x11, mtvec, x10

    lui     x10, 0xabcde
    addi    x10, x10, 0x3ef
    csrrs   x12, mtvec, x10

    lui     x10, 0x76543
    addi    x10, x10, 0x210
    csrrc   x13, mtvec, x10

    csrrw   x14, mtvec, x0

    ret

# _start:
#     csrr    a0,mhartid
# 1:
#     bnez    a0,1b
#     auipc   t0,0x0
#     addi    t0,t0,16
#     csrw    mtvec,t0
#     csrwi   satp,0
#     auipc   t0,0x0
#     addi    t0,t0,28
#     csrw    mtvec,t0
#     li      t0,-1
#     csrw    pmpaddr0,t0
#     li      t0,31
#     csrw    pmpcfg0,t0
#     auipc   t0,0x0
#     addi    t0,t0,24
#     csrw    mtvec,t0
#     csrwi   medeleg,0
#     csrwi   mideleg,0
#     csrwi   mie,0
#     li      gp,0
#     auipc   t0,0x0
#     addi    t0,t0,-152
#     csrw    mtvec,t0
#     li      a0,1
#     slli    a0,a0,0x1f
#     bgez    a0,exit
#     fence
#     li      gp,1
#     ecall
#
#     ret
#
# exit:
