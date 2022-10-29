; using the indexing function to print each char of a string
; pretty pointless considering you can just use the print operation to print the whole string

sym hello_world "hello world\n"
sym i 0

jmp start

label print
    print idx stacktop getsym i

    push len stacktop
    push getsym i
    inc
    sym i stacktop

    jlt print

    sym i 0
    ret

label start
    push getsym hello_world
    jmp print