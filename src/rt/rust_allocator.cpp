#include <stdlib.h>
#include "vg/valgrind.h"
#include "rust_allocator.h"

extern "C" void *je_malloc(size_t size);
extern "C" void je_free(void *ptr);
extern "C" void *je_realloc(void *ptr, size_t size);

class valgrind_allocator : public rust_allocator {
public:
    virtual void *malloc(size_t size) {
        return ::malloc(size);
    }
    virtual void free(void *ptr) {
        ::free(ptr);
    }
    virtual void *realloc(void *ptr, size_t size) {
        return ::realloc(ptr, size);
    }
};

class jemalloc_allocator : public rust_allocator {
public:
    virtual void *malloc(size_t size) {
        return je_malloc(size);
    }
    virtual void free(void *ptr) {
        je_free(ptr);
    }
    virtual void *realloc(void *ptr, size_t size) {
        return je_realloc(ptr, size);
    }
};

rust_allocator *
create_rust_allocator() {
    if (!RUNNING_ON_VALGRIND) {
        return new jemalloc_allocator();
    } else {
        return new valgrind_allocator();
    }
}

void
delete_rust_allocator(rust_allocator *allocator) {
    delete allocator;
}
