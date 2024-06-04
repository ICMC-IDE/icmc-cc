#include "std/io.h"

//char *musica[30];

void main () {
  char *musica[30] = {
    "AAAA", "BBBB", "CCCC", "DDDD", "EEEE", "FFFF", "GGGG", "HHHH", "IIII", "JJJJ",
    "KKKK", "LLLL", "MMMM", "NNNN", "OOOO", "PPPP", "QQQQ", "RRRR", "SSSS", "TTTT",
    "UUUU", "VVVV", "WWWW", "XXXX", "YYYY", "ZZZZ", "AAAA", "BBBB", "CCCC", "DDDD"
  };

  int p = 10;
  int c = 0;
  while (1==1) {
    if (getc() == 'a') {
      int i = c;
      do {
        puts(musica[i], p);
        p += 40;
        i++;
        if (i == 30) {
          i = 0;
        }
      } while (i != c);
      c++;
      if (c == 30) {
        c = 0;
      }
      p = 10;
    }
  }
}
