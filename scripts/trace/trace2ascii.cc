/*
 * Nibble - Concurrent Log-Structured Memory for Many-Core Key-Value Stores
 *
 * (c) 2017 Hewlett Packard Enterprise Development LP.
 *
 * This program is free software: you can redistribute it and/or modify it under the terms of the
 * GNU Lesser General Public License as published by the Free Software Foundation, either version 3
 * of the License, or (at your option) any later version. This program is distributed in the hope that
 * it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License along with this program.
 * If not, see <http://www.gnu.org/licenses/>. As an exception, the copyright holders of this Library
 * grant you permission to (i) compile an Application with the Library, and (ii) distribute the Application
 * containing code generated by the Library and added to the Application during this compilation process
 * under terms of your choice, provided you also meet the terms and conditions of the Application license.
 */


#include <vector>

#include <stdio.h>
#include <string.h>
#include <inttypes.h>
#include <unistd.h>
#include <sys/mman.h>
#include <stdlib.h>
#include <sys/stat.h>
#include <fcntl.h>

#include "tracefmt.h"

void pexit(const char *msg) {
    perror(msg);
    exit(EXIT_FAILURE);
}

int main(int narg, char *args[]) {
    if (narg != 2) {
        fprintf(stderr, "Usage: ./trace2ascii filename\n");
        exit(EXIT_FAILURE);
    }

    // open the input file
    int fd = open(args[1], O_RDONLY);
    if (fd < 0)
        pexit("open");
    struct stat s;
    if (fstat(fd, &s))
        pexit("stat");
    size_t len = (size_t)s.st_size;
    void *source = mmap(NULL, len, PROT_READ, MAP_PRIVATE, fd, 0);
    if (source == MAP_FAILED)
        pexit("mmap");
    if (madvise(source, len, MADV_SEQUENTIAL | MADV_WILLNEED))
        pexit("madvise");

    size_t n = 0ul, max = 0ul;
    char *pos = (char*)source;
    while ((uintptr_t)pos < ((uintptr_t)source + len)) {
        entry *e = (entry*)pos;
        //if (e->key == 0) n++; // count zero keys
        printf("%lu %u %u\n", e->key, e->op, e->size);
        pos += sizeof(*e);
    }
    //printf("n %lu\n", n);

    munmap(source, len);
    close(fd);

    return 0;
}
