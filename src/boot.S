.section ".text._start"
.global _start

_start:
    // read cpu id, stop slave cores
    mrs     x1, mpidr_el1
    and     x1, x1, #3
    cbz     x1, 2f
1:  wfe
    b       1b
2: 
    ldr     x1, =_start
    mov     sp, x1

    bl      init
    b       1b
