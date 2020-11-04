extern crate dydra;

#[test]fn rv64ui_p_simple_step  () { assert_eq!(dydra::run_riscv_test("/home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-simple".to_string(), true), 1); }

