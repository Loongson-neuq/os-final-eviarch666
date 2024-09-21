# 20204 NEUQ 龙芯班期末作业 - OS & Compiler 方向

**每一道题对应仓库中的一个文件夹，你需要在对应文件夹完成题目**

## 1. [主观题] 实现一个linux应用程序，显示当前目录下所有 entries

仅可用C或Rust等不带有运行时的语言

```Rust
// Your code here
```

## 2. [主观题] 在Linux环境下编写至少4个产生不同类型异常的函数

注：是指操作系统级别的异常，不是语言运行时的异常

并简要解释异常原因及操作系统的处理结果，你可以使用任何语言

```
// Your code here
```

## 3. [主观题][自动测试] Hack 线程栈

实现 `hack_func` 函数，修改线程栈的内存，在不提供`flag`具体地址时将其值修改为`target_value`.提供了`flag`的初始值便于搜索。

你只能修改`hack_func`的代码，main 函数仅用于展示原理。这意味着你无法假定甚至`flag`的相对地址。

```C
int main()
{
    long flag = rand();
    long target_value = rand();
    
    hack_func(flag, target_value);
    
    return !(flag == target);
}

void hack_func(long flag, long target_value)
{
  // Implement your code
}
```

## 3.5 简述 Vector<T> 的原理？

Vector<T> 是一种动态数组，其大小可以根据需要增加或缩小。你的回答需要至少深入到内存分配。

并阐释以下两行代码的区别：
```C++
vector<int> vec1;
vector<int>* vec2 = new vector<int>();
```

## 4. [主观题] 实现简易的内存分配器
编写一个简易的内存分配器，模拟 malloc 和 free 的行为。你可以用一个数组作为内存池，管理内存块的分配和释放。

要求：

实现 `my_malloc` 和 `my_free` 函数，管理固定大小的内存池。
不需要处理复杂的内存碎片问题，初步实现分配和释放即可。
提示：可以使用简单的链表或数组来管理已分配和未分配的内存块。

## 5. 将 C 语言转化为汇编函数

给定函数：
```C
unsigned int max_element(unsigned int* vec, long count)
{
  unsigned int t = 0;

  for (int i = 0; i < count; i++)
  {
    register unsigned int veci = vec[i];
    if (veci > t)
      t = veci;
  }
  
  return t;
}
```

只需实现功能上一致即可。你可以使用任意架构的任意语法的汇编，但是请注明你的选择

## 6. [主观题] 使用汇编语言输出“Hello, World!”
编写一个汇编程序，通过系统调用输出“Hello, World!”字符串。

**要求：**
- 不能使用任何高级语言库函数，如 C 标准库的 `printf`。
- 直接使用 Linux 的 `write` 系统调用。

可以参考 x86 或 x86_64 汇编。

参考信息：
| SYSCALL NAME | SYSCALL_ID | ARG0 (ebx) | ARG1 (ecx) | ARG2 (edx) | ARG3 (esi) | ARG4 (edi) | ARG5 (ebp) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| write | 0x04 | unsigned int fd | const char *buf | size_t count | - | - | - |
| nanosleep | 0xA2 | struct __kernel_timespec *rqtp | struct __kernel_timespec *rmtp | - | - | - | - |

## 6.5 [主观题][自动测试] 手动通过nanosleep系统调用睡眠

你不能使用任何标准库。但是你可以选择C内联汇编或者链接汇编函数

注：本题使用x86汇编

确保你的函数符合以下定义，测试代码会根据以下签名的调用约定来处理
```C
void MySleep(double seconds);
```

nanosleep 系统调用的原型参考上一题的参考信息

nanosleep 附加信息：
```C
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

## 7. [主观题] 实现一个简易的单线程异步任务调度器
编写一个程序，使用 setjmp 和 longjmp 实现简单的协程切换。每个协程应该能够交替执行，直到完成它们的任务。

要求：

使用 setjmp 保存当前执行环境，使用 longjmp 恢复另一个协程的执行环境。
每个协程可以执行一些简单的循环或递归任务。
提示：这可以帮助学生理解基本的任务切换，虽然与操作系统中的多任务调度不同，但有助于理解上下文切换的概念。

## 8. [附加题][异步编程] 将以下异步函数转化成状态机闭包类

定义以下类表示含返回值的异步任务：
```
class Task<T>
{
  bool completed;
  T? return_value;
}
```

```
async Task<int> Foo(Process p)
{
  var started = await p.StartAsync();
  
  if (started)
  {
    await p.WaitForExit();
    
    return p.ExitCode;
  }
  
  throw new Exception("Process not started!");
}
```

提示：思考 coroutine 的原理

本题没有注明语言，你可以使用任何语言，包括伪代码

## 9. [并发编程][主观题] 使用低级模型实现简单的读写锁

要求学生用尽可能底层的模型（如 `volatile` 变量、原子变量等）实现一个简单的读写锁。该锁需要允许多个读线程并发访问，但当有写线程进入时，阻止所有读写操作。

**要求：**
- 仅使用低级的同步原语，如 `volatile` 变量、原子操作（如 `__sync` 或 `atomic` 函数）。
- 简单实现即可，不要求处理读者饥饿或写者优先等高级特性。
  
**功能需求：**
- `lock_read`: 获取读锁，允许多个线程同时获取读锁，但写操作存在时应阻塞读操作。
- `lock_write`: 获取写锁，需确保写锁独占资源。
- `unlock_read`: 释放读锁。
- `unlock_write`: 释放写锁。

你不需要考虑饥饿或优先级问题，只需实现基本的读写锁功能即可。
