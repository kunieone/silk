# Silk 语言设计

Silk 的核心理念是 更少的字符，更多的信息，更轻的维护

<!-- 请你使用下面更多的符号来支持整个语法，添加对各种高级语言特性的支持，以及更多解决现在问题的语法，和一个面向未来的编程语言范式。使用更多的符号 -->

基本类型: `str` `int` `chr` `bol` `key`

箱子类型: `Box` `Arr` `Queue` `Link`

注释:`//` `/* */`

<!-- 定义一个Box -->

```ts
// <>代表声明一个Box类。这也就像是class.里面可以传入类型参数，俗称泛型。
<> Person {
    name:str = "张三"
    _age:int = 18 //使用下划线开头的不会被暴露。
    // 装饰器,比如这个是代理装饰器。
    @proxy(name int) {
        f get(){}
        f set(){}
    }
}
```

```c
<T> Sequence {
    // 当我不去初始化赋值当时候，他们将会走一个默认值，不过这里T没有默认值的ability
    _innerList:T[]
    // 因此这样的声明不合格
}

<T ::Default> Sequence(seq:T[]) {
    _innerList:T[] = seq


    // 这里@Add是为Sequence实现 `+`操作符 的能力

    @Add(seq:T){
        seq.concat(_innerList)
    }
}

```

<!-- 装饰器: -->

@Proxy
@Watch
@Add
@Minus
@Plus
@Init
@Log

<!-- 枚举使用 枚举需要使用`|`开头，他会自动增加 box里可以定义枚举，并且可以混合定义其他方法 -->

```c
<> Color {
    | BLACK
    | BLUE
    | RED
}
```

seq = Seq()

<!-- 面是更多待使用的符号，用于实现高级功能 -->

|<>|
<>
<=>
|>
<|
->->
<\*>
=>
:=
@
!!

```ruby
<> Shape {
    @Init
    f draw()
}

<> Circle >> Shape {
    radius: Int

    @Init
    f draw() {
        std.print("Drawing a circle with radius " + radius)
    }
}

<> Square >> Shape {
    side: Int

    @Init
    f draw() {
        print("Drawing a square with side " + side)
    }
}

f drawShape(shape: Shape) {
    : shape  {
     Circle c => c.draw()
     Square s => s.draw()
    _ => print("Unsupported shape")
    }
}

let circle = Circle(5)
let square = Square(4)
drawShape(circle) // 输出："Drawing a circle with radius 5"
drawShape(square) // 输出："Drawing a square with side 4"
```
