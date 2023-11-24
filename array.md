# Array

Arr是Silk语言里的一个Box类型,它本身的构造函数是 f Arr(...T ::[],)

```tsx
f quick_sort(arr ::Ord[]) ::Ord[]{
    if arr.len() > 1 {
        let pivot = partition(arr)
        quick_sort(&mut arr[..pivot])
        quick_sort(&mut arr[pivot + 1..])
    }
}
f partition<T: PartialOrd>(arr: &mut [T]) -> usize {
    let pivot = arr.len() - 1
    let mut i = 0
    for j in 0..pivot {
        if arr[j] <= arr[pivot] {
            arr.swap(i, j)
            i += 1
        }
    }
    arr.swap(i, pivot)
    i
}
```
