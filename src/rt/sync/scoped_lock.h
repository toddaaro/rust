#ifndef SCOPED_LOCK_H
#define SCOPED_LOCK_H

#include "lock_and_signal.h"
#include "spinlock.h"

class scoped_lock {
  lock_and_signal &lock;

public:
  scoped_lock(lock_and_signal &lock)
    : lock(lock) {
    lock.lock();
  }

  ~scoped_lock() {
    lock.unlock();
  }
};

class scoped_spinlock {
  spinlock &lock;

public:
  scoped_spinlock(spinlock &lock)
    : lock(lock) {
    lock.lock();
  }

  ~scoped_spinlock() {
    lock.unlock();
  }
};

#endif /* SCOPED_LOCK_H */
