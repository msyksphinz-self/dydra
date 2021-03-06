extern crate dydra;

// #[test]fn rv64ui_p_simple_step  () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-simple".to_string(), true),  1); }
//
// #[test]fn rv64ui_p_add_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-add".to_string(), true),  1); }
// #[test]fn rv64ui_p_addi_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-addi".to_string(), true),  1); }
// #[test]fn rv64ui_p_addiw_step   () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-addiw".to_string(), true),  1); }
// #[test]fn rv64ui_p_addw_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-addw".to_string(), true),  1); }
// #[test]fn rv64ui_p_sub_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sub".to_string(), true),  1); }
// #[test]fn rv64ui_p_subw_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-subw".to_string(), true),  1); }
//
// #[test]fn rv64ui_p_and_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-and".to_string(), true),  1); }
// #[test]fn rv64ui_p_andi_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-andi".to_string(), true),  1); }
// #[test]fn rv64ui_p_or_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-or".to_string(), true),  1); }
// #[test]fn rv64ui_p_ori_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-ori".to_string(), true),  1); }
// #[test]fn rv64ui_p_xor_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-xor".to_string(), true),  1); }
// #[test]fn rv64ui_p_xori_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-xori".to_string(), true),  1); }
//
// #[test]fn rv64ui_p_auipc_step   () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-auipc".to_string(), true),  1); }
// #[test]fn rv64ui_p_lui_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-lui".to_string(), true),  1); }
//
// #[test]fn rv64ui_p_beq_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-beq".to_string(), true),  1); }
// #[test]fn rv64ui_p_bge_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-bge".to_string(), true),  1); }
// #[test]fn rv64ui_p_bgeu_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-bgeu".to_string(), true),  1); }
// #[test]fn rv64ui_p_blt_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-blt".to_string(), true),  1); }
// #[test]fn rv64ui_p_bltu_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-bltu".to_string(), true),  1); }
// #[test]fn rv64ui_p_bne_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-bne".to_string(), true),  1); }
//
// #[test]fn rv64ui_p_fence_i_step () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-fence_i".to_string(), true),  1); }
//
// #[test]fn rv64ui_p_jal_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-jal".to_string(), true),  1); }
// #[test]fn rv64ui_p_jalr_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-jalr".to_string(), true),  1); }
//
// #[test]fn rv64ui_p_lb_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-lb".to_string(), true),  1); }
// #[test]fn rv64ui_p_lbu_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-lbu".to_string(), true),  1); }
// #[test]fn rv64ui_p_ld_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-ld".to_string(), true),  1); }
// #[test]fn rv64ui_p_lh_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-lh".to_string(), true),  1); }
// #[test]fn rv64ui_p_lhu_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-lhu".to_string(), true),  1); }
// #[test]fn rv64ui_p_lw_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-lw".to_string(), true),  1); }
// #[test]fn rv64ui_p_lwu_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-lwu".to_string(), true),  1); }
//
// #[test]fn rv64ui_p_sb_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sb".to_string(), true),  1); }
// #[test]fn rv64ui_p_sd_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sd".to_string(), true),  1); }
// #[test]fn rv64ui_p_sh_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sh".to_string(), true),  1); }
// #[test]fn rv64ui_p_sw_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sw".to_string(), true),  1); }
//
// #[test]fn rv64ui_p_slt_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-slt".to_string(), true),  1); }
// #[test]fn rv64ui_p_slti_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-slti".to_string(), true),  1); }
// #[test]fn rv64ui_p_sltiu_step   () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sltiu".to_string(), true),  1); }
// #[test]fn rv64ui_p_sltu_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sltu".to_string(), true),  1); }
//
// #[test]fn rv64ui_p_sll_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sll".to_string(), true),  1); }
// #[test]fn rv64ui_p_slli_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-slli".to_string(), true),  1); }
// #[test]fn rv64ui_p_slliw_step   () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-slliw".to_string(), true),  1); }
// #[test]fn rv64ui_p_sllw_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sllw".to_string(), true),  1); }
//
// #[test]fn rv64ui_p_sra_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sra".to_string(), true),  1); }
// #[test]fn rv64ui_p_srai_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-srai".to_string(), true),  1); }
// #[test]fn rv64ui_p_sraiw_step   () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sraiw".to_string(), true),  1); }
// #[test]fn rv64ui_p_sraw_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sraw".to_string(), true),  1); }
//
// #[test]fn rv64ui_p_srl_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-srl".to_string(), true),  1); }
// #[test]fn rv64ui_p_srli_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-srli".to_string(), true),  1); }
// #[test]fn rv64ui_p_srliw_step   () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-srliw".to_string(), true),  1); }
// #[test]fn rv64ui_p_srlw_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-srlw".to_string(), true),  1); }
//
// #[test]fn rv64ud_p_fadd_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-p-fadd".to_string(), true),  1); }
// #[test]fn rv64ud_p_fclass_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-p-fclass".to_string(), true),  1); }
// #[test]fn rv64ud_p_fcmp_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-p-fcmp".to_string(), true),  1); }
// #[test]fn rv64ud_p_fcvt_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-p-fcvt".to_string(), true),  1); }
// #[test]fn rv64ud_p_fcvt_w_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-p-fcvt_w".to_string(), true),  1); }
// #[test]fn rv64ud_p_fdiv_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-p-fdiv".to_string(), true),  1); }
// #[test]fn rv64ud_p_fmadd_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-p-fmadd".to_string(), true),  1); }
// #[test]fn rv64ud_p_fmin_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-p-fmin".to_string(), true),  1); }
// // #[test]fn rv64ud_p_ldst_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-p-ldst".to_string(), true),  1); }
// // #[test]fn rv64ud_p_move_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-p-move".to_string(), true),  1); }
// // #[test]fn rv64ud_p_recoding_step  () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-p-recoding".to_string(), true),  1); }
// // #[test]fn rv64ud_p_structural()_step { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-p-structural".to_string(), true),  1); }
//
// #[test]fn rv64uf_p_fadd_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-p-fadd".to_string(), true),  1); }
// #[test]fn rv64uf_p_fclass_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-p-fclass".to_string(), true),  1); }
// #[test]fn rv64uf_p_fcmp_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-p-fcmp".to_string(), true),  1); }
// #[test]fn rv64uf_p_fcvt_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-p-fcvt".to_string(), true),  1); }
// #[test]fn rv64uf_p_fcvt_w_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-p-fcvt_w".to_string(), true),  1); }
// #[test]fn rv64uf_p_fdiv_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-p-fdiv".to_string(), true),  1); }
// #[test]fn rv64uf_p_fmadd_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-p-fmadd".to_string(), true),  1); }
// #[test]fn rv64uf_p_fmin_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-p-fmin".to_string(), true),  1); }
// // #[test]fn rv64uf_p_ldst_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-p-ldst".to_string(), true),  1); }
// // #[test]fn rv64uf_p_move_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-p-move".to_string(), true),  1); }
// // #[test]fn rv64uf_p_recoding_step  () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-p-recoding".to_string(), true),  1); }
// // #[test]fn rv64uf_p_structural()_step { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-p-structural".to_string(), true),  1); }
//
// #[test]fn rv64ui_v_simple_step  () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-simple".to_string(), true),  1); }
//
// #[test]fn rv64ui_v_add_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-add".to_string(), true),  1); }
// #[test]fn rv64ui_v_addi_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-addi".to_string(), true),  1); }
// #[test]fn rv64ui_v_addiw_step   () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-addiw".to_string(), true),  1); }
// #[test]fn rv64ui_v_addw_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-addw".to_string(), true),  1); }
// #[test]fn rv64ui_v_sub_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-sub".to_string(), true),  1); }
// #[test]fn rv64ui_v_subw_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-subw".to_string(), true),  1); }
//
// #[test]fn rv64ui_v_and_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-and".to_string(), true),  1); }
// #[test]fn rv64ui_v_andi_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-andi".to_string(), true),  1); }
// #[test]fn rv64ui_v_or_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-or".to_string(), true),  1); }
// #[test]fn rv64ui_v_ori_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-ori".to_string(), true),  1); }
// #[test]fn rv64ui_v_xor_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-xor".to_string(), true),  1); }
// #[test]fn rv64ui_v_xori_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-xori".to_string(), true),  1); }
//
// #[test]fn rv64ui_v_auipc_step   () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-auipc".to_string(), true),  1); }
// #[test]fn rv64ui_v_lui_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-lui".to_string(), true),  1); }
//
// #[test]fn rv64ui_v_beq_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-beq".to_string(), true),  1); }
// #[test]fn rv64ui_v_bge_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-bge".to_string(), true),  1); }
// #[test]fn rv64ui_v_bgeu_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-bgeu".to_string(), true),  1); }
// #[test]fn rv64ui_v_blt_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-blt".to_string(), true),  1); }
// #[test]fn rv64ui_v_bltu_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-bltu".to_string(), true),  1); }
// #[test]fn rv64ui_v_bne_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-bne".to_string(), true),  1); }
// #[test]fn rv64ui_v_fence_i_step () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-fence_i".to_string(), true),  1); }
// #[test]fn rv64ui_v_jal_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-jal".to_string(), true),  1); }
// #[test]fn rv64ui_v_jalr_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-jalr".to_string(), true),  1); }
//
// #[test]fn rv64ui_v_lb_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-lb".to_string(), true),  1); }
// #[test]fn rv64ui_v_lbu_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-lbu".to_string(), true),  1); }
// #[test]fn rv64ui_v_ld_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-ld".to_string(), true),  1); }
// #[test]fn rv64ui_v_lh_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-lh".to_string(), true),  1); }
// #[test]fn rv64ui_v_lhu_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-lhu".to_string(), true),  1); }
// #[test]fn rv64ui_v_lw_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-lw".to_string(), true),  1); }
// #[test]fn rv64ui_v_lwu_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-lwu".to_string(), true),  1); }
//
// #[test]fn rv64ui_v_sb_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-sb".to_string(), true),  1); }
// #[test]fn rv64ui_v_sd_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-sd".to_string(), true),  1); }
// #[test]fn rv64ui_v_sh_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-sh".to_string(), true),  1); }
// #[test]fn rv64ui_v_sw_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-sw".to_string(), true),  1); }
//
// #[test]fn rv64ui_v_slt_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-slt".to_string(), true),  1); }
// #[test]fn rv64ui_v_slti_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-slti".to_string(), true),  1); }
// #[test]fn rv64ui_v_sltiu_step   () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-sltiu".to_string(), true),  1); }
// #[test]fn rv64ui_v_sltu_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-sltu".to_string(), true),  1); }
//
// #[test]fn rv64ui_v_sll_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-sll".to_string(), true),  1); }
// #[test]fn rv64ui_v_slli_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-slli".to_string(), true),  1); }
// #[test]fn rv64ui_v_slliw_step   () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-slliw".to_string(), true),  1); }
// #[test]fn rv64ui_v_sllw_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-sllw".to_string(), true),  1); }
//
// #[test]fn rv64ui_v_sra_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-sra".to_string(), true),  1); }
// #[test]fn rv64ui_v_srai_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-srai".to_string(), true),  1); }
// #[test]fn rv64ui_v_sraiw_step   () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-sraiw".to_string(), true),  1); }
// #[test]fn rv64ui_v_sraw_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-sraw".to_string(), true),  1); }
//
// #[test]fn rv64ui_v_srl_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-srl".to_string(), true),  1); }
// #[test]fn rv64ui_v_srli_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-srli".to_string(), true),  1); }
// #[test]fn rv64ui_v_srliw_step   () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-srliw".to_string(), true),  1); }
// #[test]fn rv64ui_v_srlw_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-v-srlw".to_string(), true),  1); }
//
// #[test]fn rv64ud_v_fadd_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-v-fadd".to_string(), true),  1); }
// #[test]fn rv64ud_v_fclass_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-v-fclass".to_string(), true),  1); }
// #[test]fn rv64ud_v_fcmp_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-v-fcmp".to_string(), true),  1); }
// #[test]fn rv64ud_v_fcvt_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-v-fcvt".to_string(), true),  1); }
// #[test]fn rv64ud_v_fcvt_w_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-v-fcvt_w".to_string(), true),  1); }
// #[test]fn rv64ud_v_fdiv_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-v-fdiv".to_string(), true),  1); }
// #[test]fn rv64ud_v_fmadd_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-v-fmadd".to_string(), true),  1); }
// #[test]fn rv64ud_v_fmin_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-v-fmin".to_string(), true),  1); }
// // #[test]fn rv64ud_v_ldst_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-v-ldst".to_string(), true),  1); }
// // #[test]fn rv64ud_v_move_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-v-move".to_string(), true),  1); }
// // #[test]fn rv64ud_v_recoding_step  () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-v-recoding".to_string(), true),  1); }
// // #[test]fn rv64ud_v_structural()_step { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ud-v-structural".to_string(), true),  1); }
// #[test]fn rv64uf_v_fadd_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-v-fadd".to_string(), true),  1); }
// #[test]fn rv64uf_v_fclass_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-v-fclass".to_string(), true),  1); }
// #[test]fn rv64uf_v_fcmp_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-v-fcmp".to_string(), true),  1); }
// #[test]fn rv64uf_v_fcvt_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-v-fcvt".to_string(), true),  1); }
// #[test]fn rv64uf_v_fcvt_w_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-v-fcvt_w".to_string(), true),  1); }
// #[test]fn rv64uf_v_fdiv_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-v-fdiv".to_string(), true),  1); }
// #[test]fn rv64uf_v_fmadd_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-v-fmadd".to_string(), true),  1); }
// #[test]fn rv64uf_v_fmin_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-v-fmin".to_string(), true),  1); }
// // #[test]fn rv64uf_v_ldst_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-v-ldst".to_string(), true),  1); }
// // #[test]fn rv64uf_v_move_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-v-move".to_string(), true),  1); }
// // #[test]fn rv64uf_v_recoding_step  () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-v-recoding".to_string(), true),  1); }
// // #[test]fn rv64uf_v_structural()_step { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64uf-v-structural".to_string(), true),  1); }
//
//
//
// // #[test]fn rv64ua_v_amoadd_d_step  () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amoadd_d".to_string(), true),  1); }
// // #[test]fn rv64ua_v_amoadd_w_step  () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amoadd_w".to_string(), true),  1); }
// // #[test]fn rv64ua_v_amoand_d_step  () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amoand_d".to_string(), true),  1); }
// // #[test]fn rv64ua_v_amoand_w_step  () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amoand_w".to_string(), true),  1); }
// // #[test]fn rv64ua_v_amomax_d_step  () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amomax_d".to_string(), true),  1); }
// // #[test]fn rv64ua_v_amomax_w_step  () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amomax_w".to_string(), true),  1); }
// // #[test]fn rv64ua_v_amomaxu_d_step () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amomaxu_d".to_string(), true),  1); }
// // #[test]fn rv64ua_v_amomaxu_w_step () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amomaxu_w".to_string(), true),  1); }
// // #[test]fn rv64ua_v_amomin_d_step  () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amomin_d".to_string(), true),  1); }
// // #[test]fn rv64ua_v_amomin_w_step  () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amomin_w".to_string(), true),  1); }
// // #[test]fn rv64ua_v_amominu_d_step () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amominu_d".to_string(), true),  1); }
// // #[test]fn rv64ua_v_amominu_w_step () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amominu_w".to_string(), true),  1); }
// // #[test]fn rv64ua_v_amoor_d_step   () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amoor_d".to_string(), true),  1); }
// // #[test]fn rv64ua_v_amoor_w_step   () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amoor_w".to_string(), true),  1); }
// // #[test]fn rv64ua_v_amoswap_d_step () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amoswap_d".to_string(), true),  1); }
// // #[test]fn rv64ua_v_amoswap_w_step () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amoswap_w".to_string(), true),  1); }
// // #[test]fn rv64ua_v_amoxor_d_step  () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amoxor_d".to_string(), true),  1); }
// // #[test]fn rv64ua_v_amoxor_w_step  () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-amoxor_w".to_string(), true),  1); }
// // #[test]fn rv64ua_v_lrsc_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-v-lrsc".to_string(), true),  1); }
// // #[test]fn rv64uc_v_rvc_step       () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64uc-v-rvc".to_string(), true),  1); }
// #[test]fn rv64um_v_div_step       () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-v-div".to_string(), true),  1); }
// // #[test]fn rv64um_v_divu_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-v-divu".to_string(), true),  1); }
// // #[test]fn rv64um_v_divuw_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-v-divuw".to_string(), true),  1); }
// // #[test]fn rv64um_v_divw_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-v-divw".to_string(), true),  1); }
// #[test]fn rv64um_v_mul_step       () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-v-mul".to_string(), true),  1); }
// // #[test]fn rv64um_v_mulh_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-v-mulh".to_string(), true),  1); }
// // #[test]fn rv64um_v_mulhsu_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-v-mulhsu".to_string(), true),  1); }
// // #[test]fn rv64um_v_mulhu_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-v-mulhu".to_string(), true),  1); }
// // #[test]fn rv64um_v_mulw_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-v-mulw".to_string(), true),  1); }
// // #[test]fn rv64um_v_rem_step       () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-v-rem".to_string(), true),  1); }
// // #[test]fn rv64um_v_remu_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-v-remu".to_string(), true),  1); }
// // #[test]fn rv64um_v_remuw_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-v-remuw".to_string(), true),  1); }
// // #[test]fn rv64um_v_remw_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-v-remw".to_string(), true),  1); }
//
//
// // #[test]fn rv64ua_p_amoadd_d_step  () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amoadd_d".to_string(), true),  1); }
// // #[test]fn rv64ua_p_amoadd_w_step  () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amoadd_w".to_string(), true),  1); }
// // #[test]fn rv64ua_p_amoand_d_step  () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amoand_d".to_string(), true),  1); }
// // #[test]fn rv64ua_p_amoand_w_step  () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amoand_w".to_string(), true),  1); }
// // #[test]fn rv64ua_p_amomax_d_step  () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amomax_d".to_string(), true),  1); }
// // #[test]fn rv64ua_p_amomax_w_step  () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amomax_w".to_string(), true),  1); }
// // #[test]fn rv64ua_p_amomaxu_d_step () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amomaxu_d".to_string(), true),  1); }
// // #[test]fn rv64ua_p_amomaxu_w_step () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amomaxu_w".to_string(), true),  1); }
// // #[test]fn rv64ua_p_amomin_d_step  () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amomin_d".to_string(), true),  1); }
// // #[test]fn rv64ua_p_amomin_w_step  () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amomin_w".to_string(), true),  1); }
// // #[test]fn rv64ua_p_amominu_d_step () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amominu_d".to_string(), true),  1); }
// // #[test]fn rv64ua_p_amominu_w_step () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amominu_w".to_string(), true),  1); }
// // #[test]fn rv64ua_p_amoor_d_step   () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amoor_d".to_string(), true),  1); }
// // #[test]fn rv64ua_p_amoor_w_step   () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amoor_w".to_string(), true),  1); }
// // #[test]fn rv64ua_p_amoswap_d_step () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amoswap_d".to_string(), true),  1); }
// // #[test]fn rv64ua_p_amoswap_w_step () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amoswap_w".to_string(), true),  1); }
// // #[test]fn rv64ua_p_amoxor_d_step  () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amoxor_d".to_string(), true),  1); }
// // #[test]fn rv64ua_p_amoxor_w_step  () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-amoxor_w".to_string(), true),  1); }
// // #[test]fn rv64ua_p_lrsc_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64ua-p-lrsc".to_string(), true),  1); }
// // #[test]fn rv64uc_p_rvc_step       () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64uc-p-rvc".to_string(), true),  1); }
// #[test]fn rv64um_p_div_step       () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-p-div".to_string(), true),  1); }
// // #[test]fn rv64um_p_divu_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-p-divu".to_string(), true),  1); }
// // #[test]fn rv64um_p_divuw_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-p-divuw".to_string(), true),  1); }
// // #[test]fn rv64um_p_divw_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-p-divw".to_string(), true),  1); }
// #[test]fn rv64um_p_mul_step       () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-p-mul".to_string(), true),  1); }
// // #[test]fn rv64um_p_mulh_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-p-mulh".to_string(), true),  1); }
// // #[test]fn rv64um_p_mulhsu_step    () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-p-mulhsu".to_string(), true),  1); }
// // #[test]fn rv64um_p_mulhu_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-p-mulhu".to_string(), true),  1); }
// // #[test]fn rv64um_p_mulw_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-p-mulw".to_string(), true),  1); }
// // #[test]fn rv64um_p_rem_step       () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-p-rem".to_string(), true),  1); }
// // #[test]fn rv64um_p_remu_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-p-remu".to_string(), true),  1); }
// // #[test]fn rv64um_p_remuw_step     () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-p-remuw".to_string(), true),  1); }
// // #[test]fn rv64um_p_remw_step      () { assert_eq!(dydra::run_riscv_test("/riscv64-unknown-elf/share/riscv-tests/isa/rv64um-p-remw".to_string(), true),  1); }
