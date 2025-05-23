// used to link with generated LLVM code
// clang -S -emit-llvm lib.c -o lib.ll

extern int printf(const char *fmt, ...);
extern int write(int fd, const void *buf, unsigned long count);
extern void exit(int status);

void __print_int__(int n) { printf("%d\n", n); }
void __print_bool__(int b) { printf(b ? "true\n" : "false\n"); }
void __print_unit__(void) { printf("unit\n"); }

int __pow__(int a, int b) {
    int r = 1;
    while (b-- > 0) r *= a;
    return r;
}

int __div__(int a, int b) {
    if (b == 0) {
        write(2, "division by zero\n", 17); // write to stderr
        exit(1);
    }
    return a / b;
}
