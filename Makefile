TARGET = arm-unknown-linux-gnueabihf
RFLAGS = -l ./crate -O -g

all: tmp Makefile kernel.img

tmp/kernel.o: src/**.rs
	rustc --target=$(TARGET) --emit=obj src/kernel.rs --out-dir=tmp $(RFLAGS)

tmp/kernel.elf: tmp/kernel.o
	arm-none-eabi-gcc -O0 -mfpu=vfp -mfloat-abi=hard -march=armv6zk -mtune=arm1176jzf-s -nostartfiles tmp/kernel.o -o tmp/kernel.elf

kernel.img: tmp/kernel.elf
	arm-none-eabi-objcopy tmp/kernel.elf -O binary kernel.img

tmp:
	mkdir -p tmp

clean:
	rm kernel.img && rm -r tmp

