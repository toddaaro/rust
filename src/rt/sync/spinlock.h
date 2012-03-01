#ifndef SPINLOCK_H
#define SPINLOCK_H

#include "lock_and_signal.h"

#if _POSIX_SPIN_LOCKS > 0

class spinlock {
    pthread_spinlock_t _lock;

 public:
    spinlock() {
	int res = pthread_spin_init(&_lock, PTHREAD_PROCESS_PRIVATE);
	// FIXME: Error handling
	assert(!res && "Failed to init spinlock");
    }

    ~spinlock() {
	pthread_spin_destroy(&_lock);
    }

    void lock() {
	int res = pthread_spin_lock(&_lock);
	assert(!res && "Locking a held spinlock?");
    }

    void unlock() {
	int res = pthread_spin_unlock(&_lock);
	assert(!res && "Unlocking an unheld spinlock?");
    }
};

#else

class spinlock {
    lock_and_signal _lock;

 public:
    spinlock() { }
    ~spinlock() { }
    void lock() { _lock.lock() }
    void unlock() { _lock.unlock() }
};

#endif /* _POSIX_SPIN_LOCKS > 0 */

#endif /* SPINLOCK_H */
