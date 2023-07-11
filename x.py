#!/usr/bin/env python

import argparse, os, sys, subprocess
from glob import glob

OS_NAME="re1v1"
ARCH = "amd64"
NASM_DIR = "src/arch"
BUILD_DIR = "target/build"
RUST_LIB = f"target/x86_64-unknown-none/release/lib{OS_NAME}.a"
OUT_BIN = f"{OS_NAME}.bin"

COMMAND = None

def run_cmd(cmd: str, quiet=False):
    if not quiet:
        print("[cmd] %s" % cmd)
    subprocess.run(cmd, shell=True)

def compile():
    nasm_src = glob(f"./{NASM_DIR}/{ARCH}/*.nasm")

    # print(nasm_src)
    objs: list[str] = []

    for f in nasm_src:
        n = f.replace(f"./{NASM_DIR}/{ARCH}", f"./{BUILD_DIR}").replace(".nasm", ".o")
        objs.append(n)
        run_cmd(f"nasm -felf64 {f} -o {n}")

    run_cmd("cargo build --release --quiet")

    objs = " ".join(objs)
    run_cmd(f"ld -melf_x86_64 -n --gc-sections -T {NASM_DIR}/{ARCH}/linker.ld -o {BUILD_DIR}/{OUT_BIN} {objs} {RUST_LIB}")

def make_iso():
    run_cmd(f"mkdir -p {BUILD_DIR}/isofiles/boot/grub")
    run_cmd(f"cp {BUILD_DIR}/{OUT_BIN} {BUILD_DIR}/isofiles/boot/kernel.bin")
    run_cmd(f"cp {NASM_DIR}/{ARCH}/grub.cfg {BUILD_DIR}/isofiles/boot/grub")
    run_cmd(f"grub-mkrescue -o {BUILD_DIR}/{OS_NAME}.iso {BUILD_DIR}/isofiles 2> /dev/null")
    run_cmd(f"rm -r {BUILD_DIR}/isofiles")


def clean():
    run_cmd(f"cargo clean")


def prep():
    run_cmd(f"mkdir -p {BUILD_DIR}")

def main():

    if COMMAND == "build":
        prep()
        compile()
    elif COMMAND == "build-iso":
        prep()
        compile()
        make_iso()
    elif COMMAND == "run":
        run_cmd(f"qemu-system-x86_64 -cdrom {BUILD_DIR}/{OS_NAME}.iso")
    elif COMMAND == "clean":
        clean()


    return


if __name__ == "__main__":

    parser = argparse.ArgumentParser(
                    prog="x.py",
                    description="Compiling script for zexit os",
                    epilog="Made by MCorange")

    parser.add_argument("command", choices=["build", "build-iso", "clean", "run"]) 
    parser.add_argument("--arch", choices=["x86-64", "x64"])
    args = parser.parse_args()
    if args.arch in ["x86_64", "x64"]:
        ARCH = "x86_64"

    if args.command in ["build", "build-iso", "clean", "run"]:
        COMMAND = args.command

    main()