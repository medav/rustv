CC=riscv64-unknown-elf-gcc
OBJCOPY=riscv64-unknown-elf-objcopy
APP=app

all: $(APP).img disasm.txt

$(APP).img: $(APP).bin
	dd if=$< of=$@ bs=1024
	dd if=/dev/zero bs=1 count=65536 >> $@

$(APP).bin: $(APP).elf
	$(OBJCOPY) -O binary $< $@

$(APP).elf: $(APP).c link.ld
	$(CC) -Tlink.ld crt0.S $< -mabi=lp64d -march=rv64g -static -Wl,--no-relax -o $@

disasm.txt: $(APP).elf
	riscv64-unknown-elf-objdump -d $< > $@


.PHONY: clean
clean:
	rm -f *.bin *.img *.elf
