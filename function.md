# Function

在Silk语言中，Function是一种常量Box类型 1. 常量，代表Function声明后不可以修改 2. Box类型，代表可以被赋予一些能力。比如 Overwrite.

```tsx
f shell_sort(arr int[]):int[] {
    gap = arr.len() / 2
    while gap > 0 {
        for i = gap ; i < arr.len() ; i++ {
            temp = arr[i]
            j = i
            while j >= gap and arr[j - gap] > temp {
                arr[j] = arr[j - gap]
                j -= gap
            }
            arr[j] = temp
        }
        gap /= 2
    }
    -> arr
}
// shell_sort = (){}   不添加Ovewrite的能力，将不会被修改，会报错
shell_sort ::Overwrite 
shell_sort = (){}  //现在变成了一个完全没有效果的函数。
shell_sort()
```
