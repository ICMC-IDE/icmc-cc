; getc   : aguarda e le um caracter do teclado
; out r7 : caracter lido
getc:
  push r1

  loadn r1, #255

  getc_loop:
    inchar r7
    cmp r7, r1
    jeq getc_loop

  getc_rts:
    pop r1
    rts

; puts    : imprime uma string em uma posição da tela
; in * r1 : string
; in r2   : posição
puts:
  push r3
  push r4
  push r5
  push r6
  push r7

  loadn r5, #0
  loadn r6, #'\n'
  loadn r7, #40

  puts_loop:
    loadi r3, r1

    cmp r3, r5
    jeq puts_rts

    cmp r3, r6
    jne puts_loop_ne
    mod r4, r2, r7
    sub r4, r7, r4
    add r2, r2, r4
    dec r2

    puts_loop_ne:
    outchar r3, r2
    inc r1
    inc r2
    jmp puts_loop

  puts_rts:
    pop r7
    pop r6
    pop r5
    pop r4
    pop r3
    rts

; printc : imprime um caracter na posição do cursor
; in r1  : caracter
printc:
  push r2

  load r2, __cursor
  outchar r1, r2
  inc r2
  store __cursor, r2

  printc_rts:
    pop r2
    rts

; prints  : imprime uma string na posição do cursor
; in * r1 : string
prints:
  push r2

  load r2, __cursor
  call puts
  store __cursor, r2

  prints_rts:
    pop r2
    rts

__cursor : var #1 
static __cursor, #0
