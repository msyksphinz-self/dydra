QEMU_AARCH64_X86 = /home/msyksphinz/work/riscv/qemu/build-5.1.0/aarch64-linux-user/qemu-aarch64
QEMU_RISCV_X86   = /home/msyksphinz/work/riscv/qemu/build-5.1.0/aarch64-linux-user/qemu-aarch64
QEMU_X86_X86     = /home/msyksphinz/work/riscv/qemu/build-5.1.0/x86_64-linux-user/qemu-x86_64

TARGET_BINARY = target/aarch64-unknown-linux-gnu/debug/dydra

all: build_aarch64
	$(MAKE) run

build_aarch64:
	cargo build --target=aarch64-unknown-linux-gnu

run:
	QEMU_LD_PREFIX=/usr/aarch64-linux-gnu $(QEMU_AARCH64_X86) -d in_asm $(TARGET_BINARY) /home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa/rv64ui-p-slt

RISCV_TESTS_BASE = /home/msyksphinz/riscv64/riscv64-unknown-elf/share/riscv-tests/isa

qemu_step_debug:
	$(QEMU_X86_X86) -d in_asm ./target/debug/dydra --step --debug --dump-gpr --elf-file $(RISCV_TESTS_BASE)/$(ELF_FILE) > $(ELF_FILE).qemu.riscv.log 2>&1

qemu_nostep_debug:
	$(QEMU_X86_X86) -d in_asm ./target/debug/dydra --mmu --dump-gpr --elf-file $(RISCV_TESTS_BASE)/$(ELF_FILE) > $(ELF_FILE).qemu.riscv.log 2>&1
