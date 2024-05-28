#define SCREEN_COLS 40
#define SCREEN_ROWS 30
#define SCREEN_LEN 1200

#define putc(ch, pos) outchar ch, pos

extern int __cursor;

extern char getc();
extern void scans(char *str, int count);
extern void puts(char *str, int pos);
extern void printc(char ch);
extern void prints(char *str);
extern void sfill(char ch);
