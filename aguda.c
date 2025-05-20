// used to link with generated LLVM code

extern int printf(const char *fmt, ...);

int __print_int__(int n) {
    printf("%d\n", n);
    return 0;
}

int __print_bool__(int b) {
    printf(b ? "true\n" : "false\n");
    return 0;
}
int __print_unit__() {
    printf("unit\n");
    return 0;
}

int __pow__(int a, int b) {
    int r = 1;
    while (b-- > 0) r *= a;
    return r;
}

// clang -S -emit-llvm aguda.c -o aguda.ll