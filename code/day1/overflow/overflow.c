#include <stdint.h>
#include <stdio.h>

int main() {
    int8_t n = 120;
    for (int i=0; i<10; i++) {
        n += 1;
        printf("%d\n", n);
    }
    return 0;
}