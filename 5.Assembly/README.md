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
