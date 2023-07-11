global long_mode_start
extern kmain     

section .text
bits 64
long_mode_start:
    ; load 0 into all data segment registers
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; call the rust main
    call kmain  

    ; * Unreachable
    ; print `OKAY` to screen
    mov rax, 0x07590741074b074f
    mov qword [0xb8000], rax


    ; shut down at this point, should not be reachable though
    hlt