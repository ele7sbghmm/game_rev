#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// struct Dir {
//     char * path;
//     int start;
//     int size;
// };
// struct Dir parse_dir(FILE *f) {
//     char path_len[4];
//     char path[256];
//     int start, stop;
//     memset(path, 0, sizeof path);

//     fgets(path_len, sizeof path_len, f);
//     fgets(path, path_len, f);
//     fgets(path_len, sizeof path_len, f);
//     fgets(path_len, sizeof path_len, f);
// };

void print_bytes(char *bytes) {
    int len = sizeof bytes;
    printf("%i\n", len);

    for (int i = 0; i < len; i++) {
        printf("%u %x %c\n", i, bytes[i], bytes[i]);
    }
}
void tag(FILE *f, char tag[], int len) {
    fgets(tag, len, f);
};
int tag_int(FILE *f) {
    char bytes[8];
    int i;

    memset(bytes, 1, sizeof bytes);
    tag(f, bytes, 5);
    print_bytes(bytes);

    char array[4] = {'1', '2', '3', '4'};

    sscanf(array, "%d", &i);

    printf("%d", i);

    return i;
};

int main() {
    char *path = "../_files/types.hex";
    FILE *f;

    f = fopen(path, "rb");
    if(f == NULL) {
        printf("f == NULL\n\n");
        exit(-1); // return EXIT_FAILURE;
    }

    int i = tag_int(f);

    // printf("%04x", i);
    // printf("\n");

    fclose(f);
    return 0;
}
