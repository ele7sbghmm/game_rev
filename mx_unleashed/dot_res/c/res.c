#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// struct Dir {
//     char * path;
//     int start;
//     int size;
// };
// struct Dir parse_dir(FILE *fd) {
//     int path_len = tag_int(fd);
//     char path[256];
//     int start, stop;
//     memset(path, 0, sizeof path);

//     fgets(path_len, sizeof path_len, f);
//     fgets(path, path_len, f);
//     fgets(path_len, sizeof path_len, f);
//     fgets(path_len, sizeof path_len, f);
// };

void print_bytes(char *bytes) {
    for (int i = 0; i < sizeof bytes; i++) {
        printf("%u %x %c\n", i, bytes[i], bytes[i]);
    }
};
void tag(FILE *f, char tag[], int len) {
    fgets(tag, len, f);
};
int tag_int(FILE *fd) {
    int i;
    char bytes[8];
    memset(bytes, 0, sizeof bytes);

    tag(fd, bytes, 4);
    print_bytes(bytes);

    return atoi(bytes);
};

int main() {
    char *path = "types.hex";
    FILE *f;

    f = fopen(path, "rb");
    if(f == NULL) {
        printf("f == NULL\n");
        exit(-1); // return EXIT_FAILURE;
    }

    int i = tag_int(f);
    printf("i > %i", i);

    fclose(f);
    return 0;
};
