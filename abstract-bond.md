# Abstract Bond

一个变量可以属于某一种bond,比如 `bond Num = flt | int` `num_a : Num =13` 然而一种具体的约束并不能够解决所有问题，如果我要写一个函数，这个函数有一个参数变量，具体类型我不知道，但是我要规定它具有Swim的能力。因为我要在函数体里调用这个变量的.swim方法。可以这么解决:

能力约束：

松能力约束 没有约束两者相同的Box
延迟约束 类似于泛型约束

```tsx
f excute_swim(e ::Swimable): ::Swimable {
    e.swim()
    -> e
}
// 强能力约束
f excute_swim2 :T:Swimable(e T): T {
    e.swim()
    -> e
}
// 仅约定传入值和返回值是同一个约束
// 延迟约束 
f excute_swim3 :T: (e T):T {
    e.swim()
    -> e
}

excute_swim4: :T:Swimable(T)->T =(e T){
    
}
```
