extern crate uint_execute;

// #[test]
// fn simple_start2() {
//     assert_eq!(
//         uint_execute::run(
//             "rvtests/simple_start2/test.riscv".to_string(),
//             &[
//                 0x0000000000000000,
//                 0x0000000000000001,
//                 0x0000000000000003,
//                 0x0000000000000006,
//                 0x000000000000000a,
//                 0x000000000000000f,
//                 0x0000000000000015,
//                 0x000000000000001c,
//                 0x0000000000000024,
//                 0x000000000000002d,
//                 0x0000000000000037,
//                 0x0000000000000042,
//                 0x000000000000004e,
//                 0x000000000000005b,
//                 0x0000000000000069,
//                 0x0000000000000078,
//                 0x0000000000000088,
//                 0x0000000000000099,
//                 0x00000000000000ab,
//                 0x00000000000000be,
//                 0x00000000000000d2,
//                 0x0000000000000190,
//                 0x0000000000000262,
//                 0x00000000000003f2,
//                 0x0000000000000654,
//                 0x0000000000000a46,
//                 0x000000000000109a,
//                 0x0000000000001ae0,
//                 0x0000000000002b7a,
//                 0x000000000000465a,
//                 0x00000000000071d4,
//                 0x0000000000002b7a
//             ]
//         ),
//         0
//     );
// }
//
// #[test]
// fn simple_start() {
//     assert_eq!(
//         uint_execute::run(
//             "rvtests/simple_start/test.riscv".to_string(),
//             &[
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000
//             ]
//         ),
//         0
//     );
// }
//
// #[test]
// fn simple_load() {
//     assert_eq!(
//         uint_execute::run(
//             "rvtests/load_test/test.riscv".to_string(),
//             &[
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000084,
//                 0x01234567deadbeef,
//                 0x0000000001234567,
//                 0x0000000000000123,
//                 0x0000000000000001,
//                 0x0000000000000000,
//                 0xffffffffdeadbeef,
//                 0xffffffffffffbeef,
//                 0xffffffffffffffef,
//                 0x0000000000000000,
//                 0x00000000deadbeef,
//                 0x000000000000beef,
//                 0x00000000000000ef,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x01234567deadbeef,
//                 0x0123456701234567,
//                 0x0123000001230000,
//                 0x0100010001000100,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//             ]
//         ),
//         0
//     );
// }
//
// #[test]
// fn branch_test() {
//     assert_eq!(
//         uint_execute::run(
//             "rvtests/branch_test/test.riscv".to_string(),
//             &[
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000001,
//                 0x0000000000000001,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000
//             ]
//         ),
//         0
//     );
// }
//
// #[test]
// fn csr_test() {
//     assert_eq!(
//         uint_execute::run(
//             "rvtests/csr_test/test.riscv".to_string(),
//             &[
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000076543210,
//                 0x0000000000000000,
//                 0x0000000012345678,
//                 0xffffffffbbfdf7ff,
//                 0xffffffff89a9c5ef,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000
//             ]
//         ),
//         0
//     );
// }
//
// #[test]
// fn long_insts() {
//     assert_eq!(
//         uint_execute::run(
//             "rvtests/long_insts/test.riscv".to_string(),
//             &[
//                 0x0000000000000000,
//                 0x0000000000000001,
//                 0x0000000000000003,
//                 0x0000000000000006,
//                 0x000000000000000a,
//                 0x000000000000000f,
//                 0x0000000000000015,
//                 0x000000000000001c,
//                 0x0000000000000024,
//                 0x000000000000002d,
//                 0x0000000000000037,
//                 0x0000000000000042,
//                 0x000000000000004e,
//                 0x000000000000005b,
//                 0x0000000000000069,
//                 0x0000000000000078,
//                 0x0000000000000088,
//                 0x0000000000000099,
//                 0x00000000000000ab,
//                 0x00000000000000be,
//                 0x00000000000000d2,
//                 0x0000000000000014,
//                 0xffffffffffffff42,
//                 0xffffffffffffff2e,
//                 0xffffffffffffffec,
//                 0x00000000000000be,
//                 0x00000000000000d2,
//                 0x0000000000000014,
//                 0xffffffffffffff42,
//                 0xffffffffffffff2e,
//                 0xffffffffffffffec,
//                 0x00000000000000be
//             ]
//         ),
//         0
//     );
// }
//
// #[test]
// fn simple_add() {
//     assert_eq!(
//         uint_execute::run(
//             "rvtests/simple_add/test.riscv".to_string(),
//             &[
//                 0x0000000000000000,
//                 0x0000000012345678,
//                 0x0000000001234567,
//                 0x0000000013579bdf,
//                 0x0000000011111111,
//                 0x0000000000204460,
//                 0x000000001337577f,
//                 0x000000001317131f,
//                 0x0000000000000438,
//                 0x00000000123456fc,
//                 0x00000000123452c4,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000
//             ]
//         ),
//         0
//     );
// }
//
// #[test]
// fn simple_lui() {
//     assert_eq!(
//         uint_execute::run(
//             "rvtests/simple_lui/test.riscv".to_string(),
//             &[
//                 0x0000000000000000,
//                 0x0000000012345000,
//                 0xffffffff87654000,
//                 0xffffffff99999000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000
//             ]
//         ),
//         0
//     );
// }
//
// #[test]
// fn shift() {
//     assert_eq!(
//         uint_execute::run(
//             "rvtests/shift/test.riscv".to_string(),
//             &[
//                 0x0000000000000000,
//                 0x000000000000000b,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0xffffffffdeadb3ef,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x001ffffffffbd5b6,
//                 0xfffffef56d9f7800,
//                 0xfffffffffffbd5b6,
//                 0x001ffffffffbd5b6,
//                 0xfffffef56d9f7800,
//                 0xfffffffffffbd5b6,
//                 0xdeadb3ef00000000,
//                 0xffffffffdeadb3ef,
//                 0x00000000deadb3ef,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//                 0x0000000000000000,
//             ]
//         ),
//         0
//     );
// }

// #[test] fn rv64ui_p_simple() {
//     assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-simple".to_string()), 1);
// }
// #[test] fn rv64ui_p_add() {
//     assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-add".to_string()), 1);
// }
// #[test] fn rv64ui_p_addi() {
//     assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-addi".to_string()), 1);
// }
// #[test] fn rv64ui_p_addiw() {
//     assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-addiw".to_string()), 1);
// }
// #[test] fn rv64ui_p_sub() {
//     assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sub".to_string()), 1);
// }
#[test]fn rv64ui_p_simple  () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-simple".to_string()), 1); }

#[test]fn rv64ui_p_add     () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-add".to_string()), 1); }
#[test]fn rv64ui_p_addi    () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-addi".to_string()), 1); }
#[test]fn rv64ui_p_addiw   () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-addiw".to_string()), 1); }
#[test]fn rv64ui_p_addw    () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-addw".to_string()), 1); }
#[test]fn rv64ui_p_sub     () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sub".to_string()), 1); }
#[test]fn rv64ui_p_subw    () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-subw".to_string()), 1); }

#[test]fn rv64ui_p_and     () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-and".to_string()), 1); }
#[test]fn rv64ui_p_andi    () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-andi".to_string()), 1); }
#[test]fn rv64ui_p_or      () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-or".to_string()), 1); }
#[test]fn rv64ui_p_ori     () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-ori".to_string()), 1); }
#[test]fn rv64ui_p_xor     () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-xor".to_string()), 1); }
#[test]fn rv64ui_p_xori    () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-xori".to_string()), 1); }

#[test]fn rv64ui_p_auipc   () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-auipc".to_string()), 1); }
#[test]fn rv64ui_p_lui     () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-lui".to_string()), 1); }

#[test]fn rv64ui_p_beq     () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-beq".to_string()), 1); }
#[test]fn rv64ui_p_bge     () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-bge".to_string()), 1); }
#[test]fn rv64ui_p_bgeu    () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-bgeu".to_string()), 1); }
#[test]fn rv64ui_p_blt     () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-blt".to_string()), 1); }
#[test]fn rv64ui_p_bltu    () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-bltu".to_string()), 1); }
#[test]fn rv64ui_p_bne     () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-bne".to_string()), 1); }
// #[test]fn rv64ui_p_fence_i () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-fence_i".to_string()), 1); }

#[test]fn rv64ui_p_jal     () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-jal".to_string()), 1); }
// #[test]fn rv64ui_p_jalr    () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-jalr".to_string()), 1); }

#[test]fn rv64ui_p_lb      () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-lb".to_string()), 1); }
#[test]fn rv64ui_p_lbu     () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-lbu".to_string()), 1); }
#[test]fn rv64ui_p_ld      () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-ld".to_string()), 1); }
#[test]fn rv64ui_p_lh      () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-lh".to_string()), 1); }
#[test]fn rv64ui_p_lhu     () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-lhu".to_string()), 1); }
#[test]fn rv64ui_p_lw      () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-lw".to_string()), 1); }
#[test]fn rv64ui_p_lwu     () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-lwu".to_string()), 1); }

#[test]fn rv64ui_p_sb      () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sb".to_string()), 1); }
#[test]fn rv64ui_p_sd      () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sd".to_string()), 1); }
#[test]fn rv64ui_p_sh      () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sh".to_string()), 1); }
#[test]fn rv64ui_p_sw      () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sw".to_string()), 1); }

#[test]fn rv64ui_p_slt     () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-slt".to_string()), 1); }
#[test]fn rv64ui_p_slti    () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-slti".to_string()), 1); }
#[test]fn rv64ui_p_sltiu   () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sltiu".to_string()), 1); }
#[test]fn rv64ui_p_sltu    () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sltu".to_string()), 1); }

#[test]fn rv64ui_p_sll     () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sll".to_string()), 1); }
#[test]fn rv64ui_p_slli    () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-slli".to_string()), 1); }
#[test]fn rv64ui_p_slliw   () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-slliw".to_string()), 1); }
#[test]fn rv64ui_p_sllw    () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sllw".to_string()), 1); }

#[test]fn rv64ui_p_sra     () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sra".to_string()), 1); }
#[test]fn rv64ui_p_srai    () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-srai".to_string()), 1); }
#[test]fn rv64ui_p_sraiw   () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sraiw".to_string()), 1); }
#[test]fn rv64ui_p_sraw    () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-sraw".to_string()), 1); }

#[test]fn rv64ui_p_srl     () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-srl".to_string()), 1); }
#[test]fn rv64ui_p_srli    () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-srli".to_string()), 1); }
#[test]fn rv64ui_p_srliw   () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-srliw".to_string()), 1); }
#[test]fn rv64ui_p_srlw    () { assert_eq!(uint_execute::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-srlw".to_string()), 1); }
