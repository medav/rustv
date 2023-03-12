#include <stdio.h>
#include <stdlib.h>
typedef unsigned char uint8_t;

#define ARRSIZE 10

uint8_t arr[] = {3, 6, 2, 8, 4, 1, 2, 5, 9, 0, 7};

#define ENABLE_DEBUG \
    asm volatile ("addi x0, x0, 1")

#define DISABLE_DEBUG \
    asm volatile ("addi x0, x0, 2")


void swap(uint8_t arr[ARRSIZE], int j) {
    uint8_t temp = arr[j];
    arr[j] = arr[j + 1];
    arr[j + 1] = temp;
}

int main() {
    int i, j;

    printf("test: %d\n", 1);

    // ENABLE_DEBUG;
    // printf("test: %s\n", 123);
    // printf(str);
    // DISABLE_DEBUG;
    // ENABLE_DEBUG;
    for (i = 0; i < ARRSIZE; i++) {
        for (j = i - 1; j >= 0 && arr[j] > arr[j + 1]; j--) {
            swap(arr, j);
        }
    }

    return 0;
}
