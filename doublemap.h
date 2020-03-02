#include <stdio.h>
#include <unistd.h>
#include <sys/mman.h>
#include <string.h>
#include <stdint.h>

int ast_doublemunlock(const void *addr, size_t size);
void* ast_doublemap(size_t size);
