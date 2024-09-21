## 3.5 简述 Vector<T> 的原理？

Vector<T> 是一种动态数组，其大小可以根据需要增加或缩小。你的回答需要至少深入到内存分配。

并阐释以下两行代码的区别：
```C++
vector<int> vec1;
vector<int>* vec2 = new vector<int>();
```

你也可以仅使用 malloc/free 手动实现一个 VectorInt。但是仍然需要解释上面两行代码的区别。
