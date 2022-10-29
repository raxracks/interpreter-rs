sym i 0 ; define symbol "i" with value 0

label main
    print getsym i ; print value of symbol "i"
    print "\n"

    ; the limit goes before the new calculated value because of the nature of a stack (last in, first out)
    push 100000 ; limit

    push getsym i ; push value of symbol "i" onto stack
    inc ; increment value on stack
    sym i stacktop ; store value on stack into symbol "i"

    jlt main ; go back to the start if i is less than limit

    exit 0