; in r1
sleep:
  push r2

  sleep_loop:
    nop
    loadn r2, #198
    sleep_loop_delay:
      dec r2
      jnz sleep_loop_delay
    dec r1
    jnz sleep_loop

  sleep_rts:
    pop r2
    rts
