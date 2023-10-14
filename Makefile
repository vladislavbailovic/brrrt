ASMFILES := $(wildcard asm/*.asm)
OUTFILES := $(patsubst %.asm, %.out, $(ASMFILES))
BINFILES := $(patsubst %.asm, %.bin, $(ASMFILES))

asm:  $(BINFILES)
	@echo "Binaries built"

$(BINFILES): $(OUTFILES)
	riscv64-unknown-linux-gnu-objcopy -O binary $(patsubst %.bin, %.out, $@) $@ --only-section .text

$(OUTFILES): $(ASMFILES)
	riscv64-unknown-linux-gnu-as -o $@ $(patsubst %.out,%.asm,$@)
