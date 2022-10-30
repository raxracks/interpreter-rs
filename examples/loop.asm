sym i 0
sym epoch getepoch

label main
    sym i add getsym i 1
    print getsym i
    print "\n"

    jlt getsym i 100000 main

    print "\n100,000 itertions took "
    print sub getepoch getsym epoch
    print "ms\n"

    exit 0
