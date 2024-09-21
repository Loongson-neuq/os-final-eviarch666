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
