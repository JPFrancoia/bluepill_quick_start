target remote :3333
set print asm-demangle on
set print pretty on
monitor tpiu config internal itm.fifo uart off 8000000
monitor itm port 0 on
load
break DefaultHandler
break UserHardFault
break main
continue
