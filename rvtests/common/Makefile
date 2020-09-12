.PHONY: all

UTIL_SRC =
UTIL_SRC += crt.S
UTIL_SRC += syscalls.c

RISCV_GCC_OPTS ?= -DPREALLOCATE=1 -mcmodel=medlow -static -std=gnu99 -O2 -ffast-math -fno-common -fno-builtin-printf
RISCV_LINK_OPTS ?= -static -nostdlib -nostartfiles -lm -lgcc -Ttest.ld

RISCV_GCC_QEMU_OPTS ?= -DPREALLOCATE=1 -mcmodel=medlow -static -std=gnu99 -O2 -ffast-math -fno-common -fno-builtin-printf
RISCV_LINK_QEMU_OPTS ?= -static -nostdlib -nostartfiles -lm -lgcc -Ttest_qemu.ld

all: test.S
	riscv64-unknown-elf-gcc -march=rv64g -O3 -o test.riscv test.S $(RISCV_GCC_OPTS) $(RISCV_LINK_OPTS)
	riscv64-unknown-elf-objdump -D -r test.riscv > test.riscv.dmp

clean:
	$(RM) *.riscv *.dmp *.o


# TARGET  = simple_add
# TARGET  = simple_lui
# TARGET += simple_ret
# TARGET += simple_start
# TARGET += simple_start2
# TARGET += func_call
# TARGET += asm
# TARGET += branch_test
# TARGET += load_test
# TARGET += load_x86
# TARGET += load_test_qemu
# TARGET += long_insts
# TARGET += csr_test
#
# define MAKEALL
# $(1).x86 : $(1).c
# 	gcc -o $(1).x86.o -c $(1).c
# 	objdump -d -r $(1).x86.o > $(1).x86.dmp
# $(1).riscv: $(1).c
# 	riscv64-unknown-elf-gcc -march=rv64g -O3 -o $(1).riscv.o -c $(1).c
# 	riscv64-unknown-elf-objdump -d -r $(1).riscv.o > $(1).riscv.dmp
#
# $(1).asm.x86 : $(1).S
# 	gcc -O3 -o $(1).x86.o -c $(1).S
# 	objdump -d -r $(1).x86.o > $(1).x86.dmp
# $(1).asm.riscv: $(1).S
# 	riscv64-unknown-elf-gcc -march=rv64g -O3 -o $(1).riscv $(1).S $(RISCV_GCC_OPTS) $(RISCV_LINK_OPTS)
# 	riscv64-unknown-elf-objdump -D -r $(1).riscv > $(1).riscv.dmp
#
# $(1).qemu.riscv: $(1).S
# 	riscv64-unknown-elf-gcc -march=rv64g -O3 -o $(1).riscv $(1).S $(RISCV_GCC_QEMU_OPTS) $(RISCV_LINK_QEMU_OPTS)
# 	riscv64-unknown-elf-objdump -D -r $(1).riscv > $(1).riscv.dmp
#
# $(eval $(call WHOLE_RULES,$(1)))
# endef
#
# $(foreach VAR,$(TARGET),$(eval $(call MAKEALL,$(VAR))))
