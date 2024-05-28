; puts  : print a string to screen
; in r1 : string
puts:
  push r0
  push r2
  push r3
  push r4
  push r5

  xor r2, r2, r2
  loadn r3, #'\n'
  loadn r4, #40
  load r5, __stdio_cursor

  puts_loop:
    loadi r6, r1

    cmp r6, r2
    jeq puts_rts

    cmp r6, r3
    jne puts_notnl
    mod r0, r5, r4
    sub r0, r4, r0
    add r5, r5, r0
    jmp puts_nl

    puts_notnl:
      outchar r6, r5
      inc r5
    puts_nl:
      inc r1
      jmp puts_loop

  puts_rts:
    store __stdio_cursor, r5

    pop r5
    pop r4
    pop r3
    pop r2
    pop r0
    rts

__stdio_cursor : var #1
static __stdio_cursor, #0
