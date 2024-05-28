; getch : waits for a key press
; in void
; out r7 : key pressed
; internal r1, r2, r3
getch:
  push r1
  push r2
  push r3

  loadn r1, #255
  load r2, __ncurses_delay
  xor r3, r3, r3

  getch_loop:
    inchar r7

    cmp r7, r1
    jne getch_rts

    cmp r2, r3
    jne getch_loop
    xor r7, r7, r7
  
  getch_rts:
    pop r3
    pop r2
    pop r1
    rts

; erase : sets all screen positions to 0
; in void
; out void
; internal r1, r2
erase:
  push r1
  push r2

  xor r1, r1, r1
  loadn r2, #1200

  erase_loop:
    dec r2
    outchar r1, r2
    jnz erase_loop

  erase_rts:
    pop r2
    pop r1
    rts

; move  : moves cursor to the specified position
; in r1 : x coordinate
; in r2 : y coordinate
; out void
; internal r3
move:
  push r3

  loadn r3, #40

  mul r3, r2, r3
  add r3, r3, r1
  store __ncurses_cursor, r3

  move_rts:
    pop r3
    rts 

__ncurses_delay : var #1
static __ncurses_delay, #1

__ncurses_cursor : var #1
static __ncurses_cursor, #0
