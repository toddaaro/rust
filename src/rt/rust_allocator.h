#ifndef RUST_ALLOCATOR_H
#define RUST_ALLOCATOR_H

class rust_allocator {
 public:
    virtual ~rust_allocator() { }
    virtual void *malloc(size_t size) = 0;
    virtual void free(void *ptr) = 0;
    virtual void *realloc(void *ptr, size_t size) = 0;
};

rust_allocator *
create_rust_allocator();

void
delete_rust_allocator(rust_allocator *allocator);

#endif /* RUST_ALLOCATOR_H */
