# crust - tiny rust kernel for RPi model B

CC = arm-none-eabi-gcc
AR = arm-none-eabi-ar
LD = arm-none-eabi-ld
OBJCOPY = arm-none-eabi-objcopy

CFLAGS = -O2 -mfpu=vfp -mfloat-abi=hard -march=armv6zk -mtune=arm1176jzf-s \
				 -nostartfiles -nodefaultlibs -nostdlib \
				 -Wall -Werror -Wno-unused -Wno-strict-aliasing \
				 -ffunction-sections -fdata-sections \
				 -fno-unwind-tables -fno-exceptions -fno-threadsafe-statics \
				 -fno-rtti -fno-use-cxa-atexit

arch ?= arm
target ?= rpi

kernel := build/$(arch)/kernel.elf
img := build/$(arch)/kernel.img

rust_kernel := target/$(target)/debug/libcrust.a
linker_layout := src/arch/$(arch)/layout.ld

assembly_source_files := $(wildcard src/arch/$(arch)/*.S)
assembly_object_files := $(patsubst src/arch/$(arch)/%.S, \
	build/$(arch)/obj/%.o, $(assembly_source_files))

.PHONY: all clean run img cargo

all: $(kernel)

clean:
	@cargo clean
	@rm -rf build

img: $(img)

$(img): $(kernel)
	@mkdir -p $(shell dirname $@)
	@$(OBJCOPY) $(kernel) -O binary $(img) 2> /dev/null

$(kernel): cargo $(assembly_object_files) $(linker_layout)
	@mkdir -p $(shell dirname $@)
	@$(CC) -Wl,-n,--gc-sections,-T,$(linker_layout) $(CFLAGS) -o $(kernel) $(assembly_object_files) $(rust_kernel) -lnosys -lgcc

# recompile rust every time, cargo does a better job caching builds
cargo:
	@cargo rustc --target $(target) -- -Z no-landing-pads

build/$(arch)/obj/%.o: src/arch/$(arch)/%.S
	@mkdir -p $(shell dirname $@)
	@$(CC) $(CFLAGS) -c -o $@ $<

