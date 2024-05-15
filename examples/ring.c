char screen_buf[1200];
char in_c;
int cursor;
int buf_p;
int buf_r;
int i;
int t;

void flush() {
  t = buf_r;
  i = 0;

  while (i < 1200) {
    outchar screen_buf[buf_r], i;
    buf_r++;
    i++;

    if (buf_r >= 1200) {
      buf_r = 0;
    }
  }
}

void main()
{
  while (1==1) {
    in_c = inchar;
    if (in_c < 255) {
      outchar in_c, cursor;
      screen_buf[buf_p] = in_c;

      cursor++;
      buf_p++;

      if (buf_p >= 1200) {
        buf_p = 0;
      }
      if (buf_r >= 1200) {
        buf_r = 0;
      }

      if (buf_p == buf_r) {
        buf_r += 40;
        for (i=buf_p; i<buf_r; i++) {
          screen_buf[i] = 0;
        }
        cursor -= 40;
        flush();
      }
    }
  }
}
