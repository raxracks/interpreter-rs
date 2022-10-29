sym stack null

push 1
push "hello"
push 3

jmp print_stack

dumps stack
clears

push "hi"
jmp print_stack

res stack
jmp print_stack

exit 0

label print_stack
    print getstack
    print "\n"
    ret
