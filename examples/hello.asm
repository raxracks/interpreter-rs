; direct
print "hello world\n"

; symbol
sym hello_world "hello world\n"
print getsym hello_world

; stack
push "hello world\n"
print stacktop