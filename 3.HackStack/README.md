## 3. [主观题][自动测试] Hack 线程栈

实现 `hack_func` 函数，修改线程栈的内存，在不提供`flag`具体地址时将`main`函数中的局部变量`flag`的值修改为`target_value`。提供了`flag`的初始值便于搜索。你可以假定搜索范围总是在`max_search_bytes`之内。

你只能修改`hack_func`的代码，main 函数仅用于展示原理。这意味着你无法假定甚至`flag`的相对地址。

样例代码：
```C
int main()
{
    long flag = rand();
    long target_value = rand();
    
    hack_func(flag, target_value);
    
    return !(flag == target);
}

void hack_func(long flag, long target_value, long max_search_bytes)
{
  // Implement your code
}
```