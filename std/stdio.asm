; putchar : writes a character to screen at cursor position
; in r1   : char
; out void
; internal r2
putchar:
  push r2

  load r2, __stdio_cursor
  call __charout
  store __stdio_cursor, r2

  putchar_rts:
    pop r2
    rts

; puts  : writes a string and a trailing new line to screen at cursor position
; in r1 : string
; out void
; internal r2, r3, r4, r5, r6, r0
puts:
  push r2
  push r3
  push r4

  load r2, __stdio_cursor
  mov r3, r1
  xor r4, r4, r4

  puts_loop:
    loadi r1, r3

    cmp r1, r4
    jeq puts_rts

    call __charout
    
    inc r3
    jmp puts_loop

  puts_rts:
    loadn r1, #'\n'
    call __charout
    store __stdio_cursor, r2

    pop r4
    pop r3
    pop r2
    rts

; in r1 char
; in r2 cursor
; out r2 cursor changed
__charout:
  push r1 ; r1 also used for calculations
  push r3
  push r4
  push r5

  loadn r3, #40
  loadn r4, #1199
  loadn r5, #'\n'
  ; more special chars can be added here

  cmp r1, r5
  jne __charout_notnl

  mod r1, r2, r3 ; calculations done using r1
  sub r1, r3, r1
  add r2, r2, r1
  jmp __charout_rts

  __charout_notnl:
    outchar r1, r2
    inc r2
  __charout_rts:
    pop r5
    pop r4
    pop r3
    pop r1
    rts

__stdio_cursor : var #1
static __stdio_cursor, #0
