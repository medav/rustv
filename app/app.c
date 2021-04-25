typedef unsigned char uint8_t;

#define ARRSIZE 10

uint8_t arr[] = {3, 6, 2, 8, 4, 1, 2, 5, 9, 0, 7};


void swap(uint8_t arr[ARRSIZE], int j) {
    uint8_t temp = arr[j];
    arr[j] = arr[j + 1];
    arr[j + 1] = temp;
}

int main() {
    int i, j;

    for (i = 0; i < ARRSIZE; i++) {
        for (j = i - 1; j >= 0 && arr[j] > arr[j + 1]; j--) {
            swap(arr, j);
        }
    }

    return 0;
}
