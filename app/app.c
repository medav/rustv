#include <stdio.h>
#include <stdlib.h>
typedef unsigned char uint8_t;

#define ARRSIZE 1000

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
    uint8_t * arr = malloc(ARRSIZE * sizeof(uint8_t));

    for (i = 0; i < ARRSIZE; i++) {
        arr[i] = i * 3;
        printf("%d ", arr[i]);
    }
    printf("\n");

    for (i = 0; i < ARRSIZE; i++) {
        for (j = i - 1; j >= 0 && arr[j] > arr[j + 1]; j--) {
            swap(arr, j);
        }
    }


    for (i = 0; i < ARRSIZE; i++) {
        printf("%d ", arr[i]);
    }
    printf("\n");

    return 0;
}
