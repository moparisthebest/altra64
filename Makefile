#
# Copyright (c) 2017 The Altra64 project contributors
# See LICENSE file in the project root for full license information.
#

ROOTDIR = $(N64_INST)
GCCN64PREFIX = $(ROOTDIR)/bin/mips64-elf-
CHKSUM64PATH = $(ROOTDIR)/bin/chksum64
MKDFSPATH = $(ROOTDIR)/bin/mkdfs
N64TOOL = $(ROOTDIR)/bin/n64tool

HEADERNAME = header.ed64
HEADERTITLE = "EverDrive OS"

SRCDIR = ./src
INCDIR = ./inc
RESDIR = ./res
BINDIR = ./target
OBJDIR = $(BINDIR)/obj
TOOLSDIR = ./tools

RUST_DIR = ./rust
RUST_TARGET_DIR = target/mips-nintendo64-none/release
RUST_FULL_TARGET_DIR = $(RUST_DIR)/$(RUST_TARGET_DIR)
RUST_DEPS := $(wildcard $(RUST_DIR)/src/*) $(wildcard $(RUST_DIR)/Cargo.*)
RUST_BIN_DEPS := $(RUST_DEPS) $(RUST_DIR)/mips-nintendo64-none.json
RUST_H_DEPS   := $(RUST_DEPS) $(RUST_DIR)/cbindgen.toml

LINK_FLAGS = -O1 -L$(ROOTDIR)/lib -L$(ROOTDIR)/mips64-elf/lib -ldragon -lmad -lyaml -lc -lm -ldragonsys -lnosys -L$(RUST_FULL_TARGET_DIR) -laltra64 $(LIBS) -Tn64ld.x
PROG_NAME = $(BINDIR)/OS64P
CFLAGS = -std=gnu99 -march=vr4300 -mtune=vr4300 -O1 -I$(INCDIR) -I$(ROOTDIR)/include -I$(ROOTDIR)/mips64-elf/include -I$(RUST_FULL_TARGET_DIR) -lpthread -lrt -D_REENTRANT -DUSE_TRUETYPE $(SET_DEBUG)
ASFLAGS = -mtune=vr4300 -march=vr4300
CC = $(GCCN64PREFIX)gcc
AS = $(GCCN64PREFIX)as
LD = $(GCCN64PREFIX)ld
OBJCOPY = $(GCCN64PREFIX)objcopy
  
SOURCES := $(wildcard $(SRCDIR)/*.c)
OBJECTS = $(SOURCES:$(SRCDIR)/%.c=$(OBJDIR)/%.o)

$(PROG_NAME).v64: $(PROG_NAME).elf $(PROG_NAME).dfs
	$(OBJCOPY) $(PROG_NAME).elf $(PROG_NAME).bin -O binary
	rm -f $(PROG_NAME).v64
	$(N64TOOL) -l 4M -t $(HEADERTITLE) -h $(RESDIR)/$(HEADERNAME) -o $(PROG_NAME).v64 $(PROG_NAME).bin -s 1M $(PROG_NAME).dfs
	$(CHKSUM64PATH) $(PROG_NAME).v64

$(RUST_FULL_TARGET_DIR)/libaltra64.a: $(RUST_BIN_DEPS)
	cd $(RUST_DIR) && cargo build --release --verbose --target mips-nintendo64-none.json -Z build-std=core && touch $(RUST_TARGET_DIR)/libaltra64.a

$(RUST_FULL_TARGET_DIR)/altra64.h: $(RUST_H_DEPS)
	cd $(RUST_DIR) && cbindgen -o $(RUST_TARGET_DIR)/altra64.h && touch $(RUST_TARGET_DIR)/altra64.h

$(PROG_NAME).elf : $(OBJECTS) $(RUST_FULL_TARGET_DIR)/libaltra64.a
	@mkdir -p $(BINDIR)
	$(LD) -o $(PROG_NAME).elf $(OBJECTS) $(LINK_FLAGS)

$(OBJECTS): $(OBJDIR)/%.o : $(SRCDIR)/%.c $(RUST_FULL_TARGET_DIR)/altra64.h
	@mkdir -p $(OBJDIR)
	$(CC) $(CFLAGS) -c $< -o $@

copy: $(PROG_NAME).v64
	sh $(TOOLSDIR)/upload.sh

$(PROG_NAME).dfs:
	@mkdir -p $(BINDIR)
	$(MKDFSPATH) $(PROG_NAME).dfs $(RESDIR)/filesystem/

all: $(PROG_NAME).v64

debug: $(PROG_NAME).v64

debug: SET_DEBUG=-DDEBUG

clean:
	rm -rf ./target ./rust/target
