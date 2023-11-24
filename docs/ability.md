# Ability 能力

能力(ability)类似于一个接口，但是用处比接口更多首先要说明，能力可以被一个`bond` 具有.所以说，能力是针对于bond（类型的集合）的，而不是类型本身。

```tsx
box MyArray :: Ord {
    ...
}
```

类似于这样的代码，其实是对MyArray这个单约束（只有一个MyArray类型约束的bond）进行能力的赋予。

能力可以使用关键字声明:

```tsx
ability Swimmable {
    f swim():none
}
```

想要给一个box的约束`赋予(::)`这个能力的话，我们可以使用 :: 符号。

```tsx
box Duck :: Swimmable {
    f swim(){ log("I'm swimming!") }    
}
```

能力可以做加法:

```tsx
ability Runable {
    f run():none
}
```

```tsx
ability SAndR  = Swimmable + Runable

ability CanSwim  = Swimmable //这样不合法。一个ability应该是唯一的名字
```

```tsx
box Duck :: Swimmable {
    f swim(){ log("I'm swimming!") }    
    f run(){ log("I'm runing!") }    
}
```
