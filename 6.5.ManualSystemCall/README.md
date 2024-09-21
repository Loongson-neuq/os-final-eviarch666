## 6.5 [主观题][自动测试] 手动通过nanosleep系统调用睡眠

你不能使用任何标准库。但是你可以选择C内联汇编或者链接汇编函数

注：本题使用x86汇编

确保你的函数符合以下定义，测试代码会根据以下签名的调用约定来处理
```C
void MySleep(double seconds);
```

相关信息：
x86_64 的 nanosleep 系统调用 id: `35`
x86_64 系统调用的调用约定使用 `rdi` 传递第一个参数，`rsi` 传递第二个参数。

nanosleep 系统调用定义：
```C
int nanosleep(const struct timespec *duration, struct timespec *_Nullable rem);

/*
 DESCRIPTION         top
       nanosleep() suspends the execution of the calling thread until
       either at least the time specified in *duration has elapsed, or
       the delivery of a signal that triggers the invocation of a
       handler in the calling thread or that terminates the process.

       If the call is interrupted by a signal handler, nanosleep()
       returns -1, sets errno to EINTR, and writes the remaining time
       into the structure pointed to by rem unless rem is NULL.  The
       value of *rem can then be used to call nanosleep() again and
       complete the specified pause (but see NOTES).

       The timespec(3) structure is used to specify intervals of time
       with nanosecond precision.

       The value of the nanoseconds field must be in the range [0,
       999999999].

       Compared to sleep(3) and usleep(3), nanosleep() has the following
       advantages: it provides a higher resolution for specifying the
       sleep interval; POSIX.1 explicitly specifies that it does not
       interact with signals; and it makes the task of resuming a sleep
       that has been interrupted by a signal handler easier.
RETURN VALUE         top
       On successfully sleeping for the requested duration, nanosleep()
       returns 0.  If the call is interrupted by a signal handler or
       encounters an error, then it returns -1, with errno set to
       indicate the error.
*/
```

`timespec` 定义：
```rust
#[repr(C)]
struct timespec {
    tv_sec: u64,  /* seconds */
    tv_nsec: u64, /* nanoseconds */
}
```

注：nanoseconds = seconds × 1 , 000 , 000 , 000
