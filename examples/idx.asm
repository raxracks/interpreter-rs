; using the indexing function to print each char of a string
; pretty pointless considering you can just use the print operation to print the whole string

sym hello_world "hello world\n"
sym i 0

jmp start

label print
    print idx getsym param0 getsym i

    sym i add getsym i 1

    jlt getsym i len getsym param0 print

    sym i 0
    ret

label start
    sym param0 getsym hello_world
    jmp print