TARGET += branch_test
TARGET += csr_test
TARGET += load_test
TARGET += long_insts
TARGET += simple_add
TARGET += simple_lui
TARGET += simple_start
TARGET += simple_start2

TARGET_DIR = $(addsuffix _dir, $(TARGET))

.PHONY: all $(TARGET) $(TARGET_DIR)

all: | $(TARGET_DIR)

$(TARGET_DIR):
	$(MAKE) -C $(subst _dir,,$@)

clean:
	$(RM) */*.riscv */*.dmp */*.o
