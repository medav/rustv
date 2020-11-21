CC=riscv64-unknown-elf-gcc
OBJCOPY=riscv64-unknown-elf-objcopy
APP=app

all: $(APP).img

$(APP).img: $(APP).bin
	dd if=$< of=$@ bs=1024 count=1
	dd if=/dev/zero bs=1 count=8192 >> $@

$(APP).bin: $(APP).elf
	$(OBJCOPY) -O binary $< $@

$(APP).elf: $(APP).c link.ld
	$(CC) -Tlink.ld crt0.S $< -mabi=lp64 -march=rv64i -nostdlib -static -Wl,--no-gc-sections -o $@

.PHONY: clean
clean:
	rm -f *.bin *.img *.elf
