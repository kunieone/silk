# Silk语法

## 变量 Variable

No keywords are required

可变变量 类似 python 的声明方法。不需要使用关键字

```js
a = 1
b = 3
c = "4"
d = 1 + c.to_i() // 5
my_arr = [1, 2, 3, 5, 8]
```

Except constant value

常量 需要加上 const

```js
const PI:flt = 3.14 //flt 是浮点数类型
const E:flt  = 1+1+1/2!+1/3!+1/4!+1/5!+1/6!+1/7!+1/8!
```

## Clause 语句

### Condition Clause If Clause 条件语句  语句

```js
// if
i = 0
if 1+1==2 { i = 1 } else {i = 3}
if 1+1==3 { i = 1 } elif 1-3==-2 {i = 3} else { i=0}
// 可以写成
i = 1+1==2 ? 1 or 3
```

### When 语句 When Clause

```js
//switch
j='x'
when j {
  'x' {
  log("j is x !")
  }
  'y' {
  log("j is y !")
  }
  else {}
}
```

### Loop Clause 循环语句

```js
animal_list = [ "tiger","duck","cat","dog","lion","flamingo" ]
// normal loop
for i=0; i<animal_list.len(); i++{
  log(e)
}
for i,e in animal_list {
  log(i)
}
//你可以使用 foreach 方法 来遍历
animal_list.for_each (i,e) {
  log(e)
}
// for_each是数组的一个方法，你也可以写成标准的形式：
animal_list.for_each((i,e){
  log(e)
})
```

loop vs while loop循环对比while循环

```js
i = 1
while 10==i++ {
  log(i)
  if i==5 {break}
}

loop {
  log(i)
  if i==10 {break}
  i++
}
// That's right, the loop is actually an automatic dead loop. So we will check the compiler for break in the loop, otherwise it will be a warning.
```

You can get the return value of while.

```js
f fib(n int) { -> n<2 ? n or fib(n-1)+fib(n-2) }
fib = (n int) { -> n<2 ? n or fib(n-1)+fib(n-2) } // is ok!
fib = (n int) -> n<2 ? n or fib(n-1)+fib(n-2)   // does also work!

i = 1
collect = while i == 3 { -> fib(i++) }
// [1,3,5,8]
// Please note that the return_value_arr here will be changed if it was not previously empty!
```

## 类型 Type

Let's take a look at what types are available in the Silk language.

我们来看一下 Silk 语言中都有哪些类型。

### 1. Basic Type 普通类型

```js
a: int = 3
b: flt = 2.0
c: str = "string"
d: chr = 'A' //UTF-8的字符
e: bol = false
```

int ,flt ,str ,chr, bol 是 Basic Type，他们的特点就是运算效率高，并是值传递的。

### 2. 盒子类型：Box Type

属于 box 是 Silk 中的一大特色的类型。box 本身是可以继承的，同时也支持组合。

比如我们需要使用的 HashMap，Set，BTreeSet，Array 这样的数据结构，就是通过 Box 实现的。下面是自定义的 box：你可以把他理解成 Java 或者其他面向对象语言中的类.

```js
box Bird {
  name = "Polly"
  animal_type = "parrot"
  eat = -> "Polly is eating."
  cry = -> "Polly is making a sound."
  fly = -> "Polly is flying."
}
bird1 = Bird()
log(is_of(bird1)) // Bird
bird.fly()
bird.cry()
```

#### 1. 数组就是一个 box 类型（实现 box 类型的单约束）

```js
box Arr {
  f new(...params){
  /*...*/
  }
}
list_1 = [1,2,3] //中括号包括住一个序列.
list_2 = list_1
list_2[1] = 5
log(list_1) // [1,2,5]   //你可以把数组看做一个引用类型。
log(is_of(list_1)) // Array
list_3 = Array(1,2,3)   //这是通过Box构造的数组。这两者没有本质的区别
```

```js
//你也可以使用立即执行函数，直接构建一个鸟的 "实例" 但是这样构建出来的box 无法定义出类型。只能知道这个box的来源是Box
bird2:Box = box {
  name = "Polly"
    animal_type = "parrot"
  eat = -> "Polly is eating."
    fly = -> "Polly is flying."
    cry = -> "Polly is making a sound."
}
// 同样你也可以使用Box这个构造函数，来构建一个Bird
//也可以通过面向对象的方法来构建box类型。
//第一个参数是一个Table。这个类似的写法，有点像JavaScript的对象字面量。
//但是不允许在内部写函数。
bird:Box = Box(
  *{name:"Polly",type:"parrot"},
  ["eat",->"Polly is flying."],
  ["cry",-> "Polly is making a sound."]
)
// 我们来看一下这个Box的定义.
bond MethodsArr = [str,F]  // F是一个函数约束。约束这个类型是一个不限制入参和返回值的函数。
box Box {
  f new(attr_table:Table,..methods:MethodsArr){
  ...
  }
}
//但是一般不用这个方法，使用语法糖即可。

```

## 4. 表类型 Table

```js
dict:Table =  *{
  a : 1,
  b : "2",
  c : "never gonna give you up",
  d :  *{
    song : "xue hua piao piao"
  }
}

//表内可以有各种属性，但是不能有「函数」
log(dict.a)
log(dict.d.song)
log(is_of(dict)) // Table 创建的table类型属于Table的Box
dict.delete("a") // true 删除了a=1 这个变量属性
json.to_str(dict) // "{\"a\":1,\"b\":\"2\",\"c\":\"never gonna give you up\",\"d\":{\"song\":\"xue hua piao piao\"}}"
```

## 4. 集合类型 Set

```js
countries:Set = Set("China", "Japan", "India", "USA", "Russia")
// 可以使用一些方法
contries.add(”America”)
contries.delete(”America”)
contries.alias(”China”,”PRC”)
contries.have(”Russia”)
```

### 5. 函数类型 ↓

## 函数 Function

函数是 Silk 中值得注意，也是语言中极其重要的一部分

在 silk 文件中，我们可以定义一个 main 函数:

```js
f main(args str[]):int{
  log("hello! {}".(args))
}
```

```js
f add(a int, b int):int{ -> a + b }  //函数括号内部的是参数名以及各个参数对应的“约束”，约束是类型的集合。这个约束将会在之后讲到。
```

Silk 函数的特点：

1. 参数括号里的类型约束不用加冒号。
2. 使用箭头(->)代替 return 关键字。
3. 使用冒号返回值类型约束
4. 可以使用赋值符号赋值一个函数
5. 不可以返回多个值，但是可以返回一个元组。
6. 函数可以作为参数，也可以被返回。函数是一等公民。和任何变量处于同一个位置

```js
//可以使用定义变量的方法定义函数 括号后面的是对函数返回值的类型约束
main = (args str[]):int{
}
函数的类型可以显式定义。
bond FuncThatReturnInt = () -> int //bond 关键字用来定义一个或多个类型的类型约束
// 对函数的类型声明中，括号里如果什么也没有，代表这个函数不接受参数。使用箭头表示返回的类型约束。
rand:FuncThatReturnInt = -> math.rand(3..5)
```

### 高阶函数

```js
f plus_255_decorator(fn ->int): ->int {
  -> fn()+ 255
}
```

这个函数传入了一个规定返回值为 int 的函数作为参数，并且返回了一个函数，这个函数会调用 fn，并把结果加上 255

### 一等公民的函数

```js
func_1 = { -> add(2,3) }
func_1() // 5
```

这个 func 的变量，被声明成了一个返回 add(2,3)调用结果的函数。

### 代码块=立即函数

```js
a = {
  ->1
}
log(a,is_of(a)) // 1 , int
//第一行a的声明实际上等价于:
a = (){
  -> 1
}() //声明了一个匿名函数，并且立即调用。
b = (){ -> 2 }  //这样是一个函数
d = ()->4      //如果直接返回值，可以去掉花括号。
c = -> 3        //这是简便写法。
log(is_of(b),is_of(c)) // F F    // F是一个box类型。
log(b(),c()) // 2  3
```

### 函数即闭包

```js
f example(): ->int|str {
  x = 2
  capture_x: ->int|str = -> x % 2 ? x.to_s() or x
  -> capture_x
}

f main(args str[]){
  a = example()
  log(a())  // "2"
}
```

闭包的定义与 JavaScript 的语法极其类似。我们可以通过闭包获得上下文的环境。我们也可以返回一个函数，这个函数捕获了当前作用域的变量。

```js
f counter():->int { 

}
```

## 运算 Operation

字符串拼接： `+`

```js
strrrr = "Silk" + " is" + " a" + "experimental" + " language"
```

支持自增`++`与自减`--`算符。 Supports increment and decrement operators.

```js
a = 1
a += 1
a++
b = a--//3
log(a) // 2
```

## 类型约束 Type-Bond

Silk 语言是属于**基于类型的语言**，但是我们不限制死类型，我们选择了一种可编程化的基于`类型约束`的方法，用于对类型进行诠释。

### [Silk Type Bond Details](typebond.md)

## ABILITY 能力

### [Ability Details](ability.md)
