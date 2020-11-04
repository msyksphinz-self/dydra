extern crate dydra;

#[test]fn rv64ui_p_simple  () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-simple".to_string()), 1); }

#[test]fn rv64ui_p_add     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-add".to_string()), 1); }
#[test]fn rv64ui_p_addi    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-addi".to_string()), 1); }
#[test]fn rv64ui_p_addiw   () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-addiw".to_string()), 1); }
#[test]fn rv64ui_p_addw    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-addw".to_string()), 1); }
#[test]fn rv64ui_p_sub     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sub".to_string()), 1); }
#[test]fn rv64ui_p_subw    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-subw".to_string()), 1); }

#[test]fn rv64ui_p_and     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-and".to_string()), 1); }
#[test]fn rv64ui_p_andi    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-andi".to_string()), 1); }
#[test]fn rv64ui_p_or      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-or".to_string()), 1); }
#[test]fn rv64ui_p_ori     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-ori".to_string()), 1); }
#[test]fn rv64ui_p_xor     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-xor".to_string()), 1); }
#[test]fn rv64ui_p_xori    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-xori".to_string()), 1); }

#[test]fn rv64ui_p_auipc   () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-auipc".to_string()), 1); }
#[test]fn rv64ui_p_lui     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-lui".to_string()), 1); }

#[test]fn rv64ui_p_beq     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-beq".to_string()), 1); }
#[test]fn rv64ui_p_bge     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-bge".to_string()), 1); }
#[test]fn rv64ui_p_bgeu    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-bgeu".to_string()), 1); }
#[test]fn rv64ui_p_blt     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-blt".to_string()), 1); }
#[test]fn rv64ui_p_bltu    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-bltu".to_string()), 1); }
#[test]fn rv64ui_p_bne     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-bne".to_string()), 1); }

#[test]fn rv64ui_p_fence_i () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-fence_i".to_string()), 1); }

#[test]fn rv64ui_p_jal     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-jal".to_string()), 1); }
#[test]fn rv64ui_p_jalr    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-jalr".to_string()), 1); }

#[test]fn rv64ui_p_lb      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-lb".to_string()), 1); }
#[test]fn rv64ui_p_lbu     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-lbu".to_string()), 1); }
#[test]fn rv64ui_p_ld      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-ld".to_string()), 1); }
#[test]fn rv64ui_p_lh      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-lh".to_string()), 1); }
#[test]fn rv64ui_p_lhu     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-lhu".to_string()), 1); }
#[test]fn rv64ui_p_lw      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-lw".to_string()), 1); }
#[test]fn rv64ui_p_lwu     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-lwu".to_string()), 1); }

#[test]fn rv64ui_p_sb      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sb".to_string()), 1); }
#[test]fn rv64ui_p_sd      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sd".to_string()), 1); }
#[test]fn rv64ui_p_sh      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sh".to_string()), 1); }
#[test]fn rv64ui_p_sw      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sw".to_string()), 1); }

#[test]fn rv64ui_p_slt     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-slt".to_string()), 1); }
#[test]fn rv64ui_p_slti    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-slti".to_string()), 1); }
#[test]fn rv64ui_p_sltiu   () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sltiu".to_string()), 1); }
#[test]fn rv64ui_p_sltu    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sltu".to_string()), 1); }

#[test]fn rv64ui_p_sll     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sll".to_string()), 1); }
#[test]fn rv64ui_p_slli    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-slli".to_string()), 1); }
#[test]fn rv64ui_p_slliw   () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-slliw".to_string()), 1); }
#[test]fn rv64ui_p_sllw    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sllw".to_string()), 1); }

#[test]fn rv64ui_p_sra     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sra".to_string()), 1); }
#[test]fn rv64ui_p_srai    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-srai".to_string()), 1); }
#[test]fn rv64ui_p_sraiw   () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sraiw".to_string()), 1); }
#[test]fn rv64ui_p_sraw    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sraw".to_string()), 1); }

#[test]fn rv64ui_p_srl     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-srl".to_string()), 1); }
#[test]fn rv64ui_p_srli    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-srli".to_string()), 1); }
#[test]fn rv64ui_p_srliw   () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-srliw".to_string()), 1); }
#[test]fn rv64ui_p_srlw    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-srlw".to_string()), 1); }

#[test]fn rv64ud_p_fadd      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-p-fadd".to_string()), 1); }
#[test]fn rv64ud_p_fclass    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-p-fclass".to_string()), 1); }
#[test]fn rv64ud_p_fcmp      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-p-fcmp".to_string()), 1); }
#[test]fn rv64ud_p_fcvt      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-p-fcvt".to_string()), 1); }
#[test]fn rv64ud_p_fcvt_w    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-p-fcvt_w".to_string()), 1); }
#[test]fn rv64ud_p_fdiv      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-p-fdiv".to_string()), 1); }
#[test]fn rv64ud_p_fmadd     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-p-fmadd".to_string()), 1); }
#[test]fn rv64ud_p_fmin      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-p-fmin".to_string()), 1); }
#[test]fn rv64ud_p_ldst      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-p-ldst".to_string()), 1); }
#[test]fn rv64ud_p_move      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-p-move".to_string()), 1); }
#[test]fn rv64ud_p_recoding  () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-p-recoding".to_string()), 1); }
#[test]fn rv64ud_p_structural() { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-p-structural".to_string()), 1); }

#[test]fn rv64uf_p_fadd      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-p-fadd".to_string()), 1); }
#[test]fn rv64uf_p_fclass    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-p-fclass".to_string()), 1); }
#[test]fn rv64uf_p_fcmp      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-p-fcmp".to_string()), 1); }
#[test]fn rv64uf_p_fcvt      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-p-fcvt".to_string()), 1); }
#[test]fn rv64uf_p_fcvt_w    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-p-fcvt_w".to_string()), 1); }
#[test]fn rv64uf_p_fdiv      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-p-fdiv".to_string()), 1); }
#[test]fn rv64uf_p_fmadd     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-p-fmadd".to_string()), 1); }
#[test]fn rv64uf_p_fmin      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-p-fmin".to_string()), 1); }
#[test]fn rv64uf_p_ldst      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-p-ldst".to_string()), 1); }
#[test]fn rv64uf_p_move      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-p-move".to_string()), 1); }
#[test]fn rv64uf_p_recoding  () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-p-recoding".to_string()), 1); }
#[test]fn rv64uf_p_structural() { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-p-structural".to_string()), 1); }

#[test]fn rv64ui_v_simple  () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-simple".to_string()), 1); }

#[test]fn rv64ui_v_add     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-add".to_string()), 1); }
#[test]fn rv64ui_v_addi    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-addi".to_string()), 1); }
#[test]fn rv64ui_v_addiw   () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-addiw".to_string()), 1); }
#[test]fn rv64ui_v_addw    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-addw".to_string()), 1); }
#[test]fn rv64ui_v_sub     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-sub".to_string()), 1); }
#[test]fn rv64ui_v_subw    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-subw".to_string()), 1); }

#[test]fn rv64ui_v_and     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-and".to_string()), 1); }
#[test]fn rv64ui_v_andi    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-andi".to_string()), 1); }
#[test]fn rv64ui_v_or      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-or".to_string()), 1); }
#[test]fn rv64ui_v_ori     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-ori".to_string()), 1); }
#[test]fn rv64ui_v_xor     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-xor".to_string()), 1); }
#[test]fn rv64ui_v_xori    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-xori".to_string()), 1); }

#[test]fn rv64ui_v_auipc   () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-auipc".to_string()), 1); }
#[test]fn rv64ui_v_lui     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-lui".to_string()), 1); }

#[test]fn rv64ui_v_beq     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-beq".to_string()), 1); }
#[test]fn rv64ui_v_bge     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-bge".to_string()), 1); }
#[test]fn rv64ui_v_bgeu    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-bgeu".to_string()), 1); }
#[test]fn rv64ui_v_blt     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-blt".to_string()), 1); }
#[test]fn rv64ui_v_bltu    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-bltu".to_string()), 1); }
#[test]fn rv64ui_v_bne     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-bne".to_string()), 1); }
#[test]fn rv64ui_v_fence_i () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-fence_i".to_string()), 1); }
#[test]fn rv64ui_v_jal     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-jal".to_string()), 1); }
#[test]fn rv64ui_v_jalr    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-jalr".to_string()), 1); }

#[test]fn rv64ui_v_lb      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-lb".to_string()), 1); }
#[test]fn rv64ui_v_lbu     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-lbu".to_string()), 1); }
#[test]fn rv64ui_v_ld      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-ld".to_string()), 1); }
#[test]fn rv64ui_v_lh      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-lh".to_string()), 1); }
#[test]fn rv64ui_v_lhu     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-lhu".to_string()), 1); }
#[test]fn rv64ui_v_lw      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-lw".to_string()), 1); }
#[test]fn rv64ui_v_lwu     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-lwu".to_string()), 1); }

#[test]fn rv64ui_v_sb      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-sb".to_string()), 1); }
#[test]fn rv64ui_v_sd      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-sd".to_string()), 1); }
#[test]fn rv64ui_v_sh      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-sh".to_string()), 1); }
#[test]fn rv64ui_v_sw      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-sw".to_string()), 1); }

#[test]fn rv64ui_v_slt     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-slt".to_string()), 1); }
#[test]fn rv64ui_v_slti    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-slti".to_string()), 1); }
#[test]fn rv64ui_v_sltiu   () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-sltiu".to_string()), 1); }
#[test]fn rv64ui_v_sltu    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-sltu".to_string()), 1); }

#[test]fn rv64ui_v_sll     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-sll".to_string()), 1); }
#[test]fn rv64ui_v_slli    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-slli".to_string()), 1); }
#[test]fn rv64ui_v_slliw   () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-slliw".to_string()), 1); }
#[test]fn rv64ui_v_sllw    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-sllw".to_string()), 1); }

#[test]fn rv64ui_v_sra     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-sra".to_string()), 1); }
#[test]fn rv64ui_v_srai    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-srai".to_string()), 1); }
#[test]fn rv64ui_v_sraiw   () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-sraiw".to_string()), 1); }
#[test]fn rv64ui_v_sraw    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-sraw".to_string()), 1); }

#[test]fn rv64ui_v_srl     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-srl".to_string()), 1); }
#[test]fn rv64ui_v_srli    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-srli".to_string()), 1); }
#[test]fn rv64ui_v_srliw   () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-srliw".to_string()), 1); }
#[test]fn rv64ui_v_srlw    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-srlw".to_string()), 1); }

#[test]fn rv64ud_v_fadd      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-v-fadd".to_string()), 1); }
#[test]fn rv64ud_v_fclass    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-v-fclass".to_string()), 1); }
#[test]fn rv64ud_v_fcmp      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-v-fcmp".to_string()), 1); }
#[test]fn rv64ud_v_fcvt      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-v-fcvt".to_string()), 1); }
#[test]fn rv64ud_v_fcvt_w    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-v-fcvt_w".to_string()), 1); }
#[test]fn rv64ud_v_fdiv      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-v-fdiv".to_string()), 1); }
#[test]fn rv64ud_v_fmadd     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-v-fmadd".to_string()), 1); }
#[test]fn rv64ud_v_fmin      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-v-fmin".to_string()), 1); }
#[test]fn rv64ud_v_ldst      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-v-ldst".to_string()), 1); }
#[test]fn rv64ud_v_move      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-v-move".to_string()), 1); }
#[test]fn rv64ud_v_recoding  () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-v-recoding".to_string()), 1); }
#[test]fn rv64ud_v_structural() { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-v-structural".to_string()), 1); }
#[test]fn rv64uf_v_fadd      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-v-fadd".to_string()), 1); }
#[test]fn rv64uf_v_fclass    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-v-fclass".to_string()), 1); }
#[test]fn rv64uf_v_fcmp      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-v-fcmp".to_string()), 1); }
#[test]fn rv64uf_v_fcvt      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-v-fcvt".to_string()), 1); }
#[test]fn rv64uf_v_fcvt_w    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-v-fcvt_w".to_string()), 1); }
#[test]fn rv64uf_v_fdiv      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-v-fdiv".to_string()), 1); }
#[test]fn rv64uf_v_fmadd     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-v-fmadd".to_string()), 1); }
#[test]fn rv64uf_v_fmin      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-v-fmin".to_string()), 1); }
#[test]fn rv64uf_v_ldst      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-v-ldst".to_string()), 1); }
#[test]fn rv64uf_v_move      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-v-move".to_string()), 1); }
#[test]fn rv64uf_v_recoding  () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-v-recoding".to_string()), 1); }
#[test]fn rv64uf_v_structural() { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-v-structural".to_string()), 1); }



// #[test]fn rv64ua_v_amoadd_d  () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amoadd_d".to_string()), 1); }
// #[test]fn rv64ua_v_amoadd_w  () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amoadd_w".to_string()), 1); }
// #[test]fn rv64ua_v_amoand_d  () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amoand_d".to_string()), 1); }
// #[test]fn rv64ua_v_amoand_w  () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amoand_w".to_string()), 1); }
// #[test]fn rv64ua_v_amomax_d  () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amomax_d".to_string()), 1); }
// #[test]fn rv64ua_v_amomax_w  () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amomax_w".to_string()), 1); }
// #[test]fn rv64ua_v_amomaxu_d () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amomaxu_d".to_string()), 1); }
// #[test]fn rv64ua_v_amomaxu_w () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amomaxu_w".to_string()), 1); }
// #[test]fn rv64ua_v_amomin_d  () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amomin_d".to_string()), 1); }
// #[test]fn rv64ua_v_amomin_w  () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amomin_w".to_string()), 1); }
// #[test]fn rv64ua_v_amominu_d () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amominu_d".to_string()), 1); }
// #[test]fn rv64ua_v_amominu_w () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amominu_w".to_string()), 1); }
// #[test]fn rv64ua_v_amoor_d   () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amoor_d".to_string()), 1); }
// #[test]fn rv64ua_v_amoor_w   () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amoor_w".to_string()), 1); }
// #[test]fn rv64ua_v_amoswap_d () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amoswap_d".to_string()), 1); }
// #[test]fn rv64ua_v_amoswap_w () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amoswap_w".to_string()), 1); }
// #[test]fn rv64ua_v_amoxor_d  () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amoxor_d".to_string()), 1); }
// #[test]fn rv64ua_v_amoxor_w  () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amoxor_w".to_string()), 1); }
// #[test]fn rv64ua_v_lrsc      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-lrsc".to_string()), 1); }
// #[test]fn rv64uc_v_rvc       () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64uc-v-rvc".to_string()), 1); }
// #[test]fn rv64um_v_div       () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-v-div".to_string()), 1); }
// #[test]fn rv64um_v_divu      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-v-divu".to_string()), 1); }
// #[test]fn rv64um_v_divuw     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-v-divuw".to_string()), 1); }
// #[test]fn rv64um_v_divw      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-v-divw".to_string()), 1); }
// #[test]fn rv64um_v_mul       () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-v-mul".to_string()), 1); }
// #[test]fn rv64um_v_mulh      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-v-mulh".to_string()), 1); }
// #[test]fn rv64um_v_mulhsu    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-v-mulhsu".to_string()), 1); }
// #[test]fn rv64um_v_mulhu     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-v-mulhu".to_string()), 1); }
// #[test]fn rv64um_v_mulw      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-v-mulw".to_string()), 1); }
// #[test]fn rv64um_v_rem       () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-v-rem".to_string()), 1); }
// #[test]fn rv64um_v_remu      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-v-remu".to_string()), 1); }
// #[test]fn rv64um_v_remuw     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-v-remuw".to_string()), 1); }
// #[test]fn rv64um_v_remw      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-v-remw".to_string()), 1); }


// #[test]fn rv64ua_p_amoadd_d  () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amoadd_d".to_string()), 1); }
// #[test]fn rv64ua_p_amoadd_w  () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amoadd_w".to_string()), 1); }
// #[test]fn rv64ua_p_amoand_d  () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amoand_d".to_string()), 1); }
// #[test]fn rv64ua_p_amoand_w  () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amoand_w".to_string()), 1); }
// #[test]fn rv64ua_p_amomax_d  () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amomax_d".to_string()), 1); }
// #[test]fn rv64ua_p_amomax_w  () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amomax_w".to_string()), 1); }
// #[test]fn rv64ua_p_amomaxu_d () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amomaxu_d".to_string()), 1); }
// #[test]fn rv64ua_p_amomaxu_w () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amomaxu_w".to_string()), 1); }
// #[test]fn rv64ua_p_amomin_d  () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amomin_d".to_string()), 1); }
// #[test]fn rv64ua_p_amomin_w  () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amomin_w".to_string()), 1); }
// #[test]fn rv64ua_p_amominu_d () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amominu_d".to_string()), 1); }
// #[test]fn rv64ua_p_amominu_w () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amominu_w".to_string()), 1); }
// #[test]fn rv64ua_p_amoor_d   () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amoor_d".to_string()), 1); }
// #[test]fn rv64ua_p_amoor_w   () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amoor_w".to_string()), 1); }
// #[test]fn rv64ua_p_amoswap_d () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amoswap_d".to_string()), 1); }
// #[test]fn rv64ua_p_amoswap_w () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amoswap_w".to_string()), 1); }
// #[test]fn rv64ua_p_amoxor_d  () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amoxor_d".to_string()), 1); }
// #[test]fn rv64ua_p_amoxor_w  () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amoxor_w".to_string()), 1); }
// #[test]fn rv64ua_p_lrsc      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-lrsc".to_string()), 1); }
// #[test]fn rv64uc_p_rvc       () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64uc-p-rvc".to_string()), 1); }
// #[test]fn rv64um_p_div       () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-p-div".to_string()), 1); }
// #[test]fn rv64um_p_divu      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-p-divu".to_string()), 1); }
// #[test]fn rv64um_p_divuw     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-p-divuw".to_string()), 1); }
// #[test]fn rv64um_p_divw      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-p-divw".to_string()), 1); }
// #[test]fn rv64um_p_mul       () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-p-mul".to_string()), 1); }
// #[test]fn rv64um_p_mulh      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-p-mulh".to_string()), 1); }
// #[test]fn rv64um_p_mulhsu    () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-p-mulhsu".to_string()), 1); }
// #[test]fn rv64um_p_mulhu     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-p-mulhu".to_string()), 1); }
// #[test]fn rv64um_p_mulw      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-p-mulw".to_string()), 1); }
// #[test]fn rv64um_p_rem       () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-p-rem".to_string()), 1); }
// #[test]fn rv64um_p_remu      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-p-remu".to_string()), 1); }
// #[test]fn rv64um_p_remuw     () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-p-remuw".to_string()), 1); }
// #[test]fn rv64um_p_remw      () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-p-remw".to_string()), 1); }
