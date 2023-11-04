addi x15, x0, 3
addi x15, x15, 5
addi x15, x15, 4
loop: addi x14, x14, 1
bne x15, x14, loop
addi x15, x15, 1
add x14, x15, x14
sw x14, 0(x0)

