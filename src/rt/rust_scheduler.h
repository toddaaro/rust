#ifndef RUST_SCHEDULER_H
#define RUST_SCHEDULER_H

#include "rust_internal.h"

class rust_scheduler : public kernel_owned<rust_scheduler> {
    // FIXME: Make these private
public:
    rust_kernel *kernel;
    rust_srv *srv;
    rust_env *env;
private:
    // Protects the random number context and the live_thread count
    lock_and_signal lock;
    randctx rctx;
    // When this hits 0 we will unregister from the kernel
    uintptr_t live_threads;
    
    array_list<rust_task_thread *> threads;

    const size_t num_threads;

    // FIXME: This is unused
    int rval;

    rust_sched_id id;

    void create_task_threads();
    void destroy_task_threads();

    rust_task_thread *create_task_thread(int id);
    void destroy_task_thread(rust_task_thread *thread);

public:
    rust_scheduler(rust_kernel *kernel, rust_srv *srv, size_t num_threads);
    ~rust_scheduler();

    void set_id(rust_sched_id id) { this->id = id; };
    void start_task_threads();
    void kill_all_tasks();
    rust_task_id create_task(rust_task *spawner,
			     const char *name,
			     size_t init_stack_sz);
    rust_task_id create_task(rust_task *spawner, const char *name);
    void exit();
    size_t number_of_threads();
    void release_thread();
};

#endif /* RUST_SCHEDULER_H */
