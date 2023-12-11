箱子类型是 silk 语言中为了封装内容而设计的。

## 什么是封装？

## 写法

```
<> Circle {
    _ x:int
    _ y:int

    f to_s() -> `#{x}, #{y}`

    f draw(){
        std.print(at: to_s())
    }

}
```




