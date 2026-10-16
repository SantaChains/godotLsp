# Mutex

**继承**：[RefCounted](class_refcounted.md) → [Object](class_object.md) → **Mutex**

> A synchronization mutex (mutual exclusion). This is used to synchronize multiple Threads, and is equivalent to a binary Semaphore. It guarantees that only one thread can access a critical section at a time.

<details><summary>官方英文描述（中文精译进行中）</summary>

A synchronization mutex (mutual exclusion). This is used to synchronize multiple Threads, and is equivalent to a binary Semaphore. It guarantees that only one thread can access a critical section at a time.

This is a reentrant mutex, meaning that it can be locked multiple times by one thread, provided it also unlocks it as many times.

Warning: To ensure proper cleanup without crashes or deadlocks, the following conditions must be met:

- When a Mutex's reference count reaches zero and it is therefore destroyed, no threads (including the one on which the destruction will happen) must have it locked.
- When a Thread's reference count reaches zero and it is therefore destroyed, it must not have any mutex locked.

</details>

## 方法

### lock {#lock}

```gdscript
void lock()
```

Locks this Mutex, blocks until it is unlocked by the current owner.

Note: This function returns without blocking if the thread already has ownership of the mutex.

### try_lock {#try_lock}

```gdscript
bool try_lock()
```

Tries locking this Mutex, but does not block. Returns true on success, false otherwise.

Note: This function returns true if the thread already has ownership of the mutex.

### unlock {#unlock}

```gdscript
void unlock()
```

Unlocks this Mutex, leaving it to other threads.

Note: If a thread called lock() or try_lock() multiple times while already having ownership of the mutex, it must also call unlock() the same number of times in order to unlock it correctly.

Warning: Calling unlock() more times than lock() on a given thread, thus ending up trying to unlock a non-locked mutex, is wrong and may causes crashes or deadlocks.
