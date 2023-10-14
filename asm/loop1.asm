addi x1, x0, 3
addi x1, x1, 5
addi x1, x1, 4
loop: addi x2, x2, 1
bne x1, x2, loop
addi x1, x1, 1
add x2, x1, x2
sw x2, 0(x16)

