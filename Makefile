CFILES := $(wildcard data/c/*.c)
SFILES := $(patsubst %.c, %.asm, $(CFILES))
OFILES := $(patsubst %.c, %.out, $(CFILES))
# BFILES := $(patsubst %.c, %.bin, $(CFILES))

ASMFILES := $(wildcard data/asm/*.asm)
OUTFILES := $(patsubst %.asm, %.out, $(ASMFILES))
BINFILES := $(patsubst %.asm, %.bin, $(ASMFILES))

all:  $(OUTFILES) $(OFILES)
	@echo "Binaries built"

# $(BINFILES): $(OUTFILES)
# 	riscv32-unknown-linux-gnu-objcopy -O binary $(patsubst %.bin, %.out, $@) $@ --only-section .text

$(OUTFILES): $(ASMFILES)
	riscv32-unknown-linux-gnu-as -o $@ $(patsubst %.out,%.asm,$@)

$(OFILES): $(SFILES)
	riscv32-unknown-linux-gnu-as -o $@ $(patsubst %.out,%.asm,$@)

$(SFILES): $(CFILES)
	riscv32-unknown-linux-gnu-gcc -S -march=rv32i -mabi=ilp32 -o $@ $(patsubst %.asm,%.c,$@)
	# sed -E -e '/\s+\./d' -e '/^main:/d' -e 's/^\s+//' -i $@

# $(BFILES): $(OFILES)
# 	riscv32-unknown-linux-gnu-objcopy -O binary $(patsubst %.bin, %.out, $@) $@ --only-section .text

show: $(OFILES)
	riscv32-unknown-linux-gnu-objdump --disassemble -M numeric,no-aliases $^

clean:
	# -rm $(BINFILES)
	# -rm $(BFILES)
	-rm $(OUTFILES)
	-rm $(SFILES)
	-rm $(OFILES)
