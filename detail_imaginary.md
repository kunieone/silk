# Silk

# Get Started

> silk 是一个面向函数式+组合式+继承的多功能语言。具有现代工程编程语言的特性，并且语法丝滑丰富，有助于我们高效率地编写工程式的高标准可维护的项目。Silk语言的入门将会有一定的难度，因为它的语法配备了许多的关键字。在语法功能上，Silk语言借鉴了当代各种的语言。
> 

> Silk is a versatile language oriented towards function + composition + inheritance. Features of a modern engineering programming language and a silky syntax that helps us write engineering-style projects of high standards and maintainable efficiently. Introduction to the Silk language can be challenging because its syntax is equipped with many keywords. It is fair to say that in terms of grammatical functions, the Silk language borrows from various contemporary languages.
> 

# #1 变量 Variable

可变变量 类似python的声明方法。不需要使用关键字

Volatile variables are similar to Python's declarative methods. No keywords are required

```jsx
a = 1
b = 3
c = '4'
d = 1 + c.to_i() // 5  
my_arr = [1,2,3,5,8]
```

常量 需要加上 const

constant value:

```jsx
const PI:flt = 3.14 //flt 是浮点数类型
const E:flt  = 1+1+1/2!+1/3!+1/4!+1/5!+1/6!+1/7!+1/8!
```

# #2 语句 Clause

### 条件语句 If语句 Condition Clause

```jsx
// if
i = 0
if 1+1==2 { i = 1 } els {i = 3}
if 1+1==3 { i = 1 } elf 1-3==-2 {i = 3} els { i=0}
// 可以写成
i = 1+1==2 ? 1 or 3
```

### 分支语句 When语句 When Clause

```jsx
//switch
j='x'
when j {
	'x'{
			log("j is x !")
	}
	'y'{
			log("j is y !")
	}
//处理其他情况，一般我们建议这个匹配是需要写的。因为这更加规范。
}
```

### 多彩的循环语句 For语句，Loop语句，While语句

```jsx
animal_list = [ "tiger","duck","cat","dog","lion","flamingo" ]
// normal for
for i=0;i<animal_list.len();i++{
	log(e)
}
// for of
for e of animal_list{ //traverse element in the literator
	log(e)
}
//for in
for i in animal_list{ //traverse i in the literator
	log(i)
}
for each
for i,e each animal_list{
	log(i)
}
//你可以使用foreach *方法* 来遍历
animal_list.for_each (i,e){
	log(e)
}
// for_each是数组的一个方法，你也可以写成标准的形式：
animal_list.for_each((i,e){
	log(e)
})
// 这个为什么能实现？因为for_each有「无括号传参」的ability
// Why is this possible? Because for_each has the ability "ParamWithoutParen"

ability Arr :: ParamWithoutParen {
	~~~~self.for_each
~~~~}
```

While loop vs. Loop loop

```jsx
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

```jsx
f fib(n int) {	-> n<2 ? n or fib(n-1)+fib(n-2) }
i = 1
collect = while 10==i { -> fib(i++) }
// Please note that the return_value_arr here will be changed if it was not previously empty!
```

# #3 类型 Type

我们来看一下Silk语言中都有哪些类型。

## 1. 普通类型 Basic Type

```jsx
a:int = 3
b:flt =  2.0
c:str = "string"
d:chr = 'A' //UTF-8的字符
e:bol = false
```

int ,flt ,str ,chr 这四个类型是BasicType，他们的特点就是运算效率高，并是值传递的。

## 2. 盒子类型：Box Type

属于box是Silk中的一大特色的类型。box本身是可以继承的，同时也支持组合。

比如我们需要使用的HashMap，Set，BTreeSet，Array 这样的数据结构，就是通过Box实现的。下面是自定义的box：你可以把他理解成Java或者其他面向对象语言中的类.

```jsx
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

### 数组就是一个box类型（实现box类型的单约束）。

```jsx
box Arr {
	new (...params){
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

- 将面向对象和组合式编程统一

```jsx
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
	{name:"Polly",type:"parrot"},
	["eat",->"Polly is flying."],
	["cry",-> "Polly is making a sound."]
)
// 我们来看一下这个Box的定义.
type MethodsArr = : [str,F]  // F是一个函数约束。约束这个类型是一个不限制入参和返回值的函数。
box Box {
	f	new(attr_table:Table,..methods:MethodsArr){ 
	... 
	}  
}
//但是一般不用这个方法，使用语法糖即可。

```

## 4. 表类型 Table

```jsx
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

```jsx
countries:Set = Set("China", "Japan", "India", "USA", "Russia")
// 可以使用一些方法
contries.add(”America”)
contries.delete(”America”)
contries.alias(”China”,”PRC”)
contries.have(”Russia”)
```

### 5. 函数类型 ↓

# 函数 Function

函数是Silk中值得注意，也是语言中极其重要的一部分

在silk文件中，我们可以定义一个main函数:

```jsx
f main(args str[]):int{
	log("hello! {}".(args))
}
```

```jsx
f add(a int, b int):int{ -> a + b }  //函数括号内部的是参数名以及各个参数对应的“约束”，约束是类型的集合。这个约束将会在之后讲到。
```

Silk函数的特点：

1. 参数括号里的类型约束不用加冒号。
2. 使用箭头(->)代替return关键字。
3. 使用冒号返回值类型约束
4. 可以使用赋值符号赋值一个函数
5. 不可以返回多个值，但是可以返回一个元组。
6. 函数可以作为参数，也可以被返回。函数是一等公民。和任何变量处于同一个位置

```jsx
//可以使用定义变量的方法定义函数 括号后面的是对函数返回值的类型约束
main = (args str[]):int{ 
}
函数的类型可以显式定义。
bond FuncThatReturnInt = () -> int //bond 关键字用来定义一个或多个类型的类型约束
// 对函数的类型声明中，括号里如果什么也没有，代表这个函数不接受参数。使用箭头表示返回的类型约束。
rand:FuncThatReturnInt = -> math.rand(3..5)
```

### 高阶函数：

```jsx
f plus_255_decorator(fn ->int): ->int {
	-> fn()+ 255
}
```

这个函数传入了一个规定返回值为int的函数作为参数，并且返回了一个函数，这个函数会调用fn，并把结果加上255

其中冒号后面的 ->int 代表了一个函数类型约束。 这个函数的约束是必须返回一个整型。

```jsx
F // params:undefined ,return : undefined
()->int // params: none , return: int
->int // params: undefined , return: int
(a int)->int //params: a: int  , return :int
(a)->int //params: a: no_restrict  , return :int
(a,b)->int param: a,b : no_restrict , return: int 
(a,b,..)->int param: more than a,b , return : int
->_ //params: undefined, return 
```

### 一等公民的函数

```jsx

func = { -> add(2,3) }
func() // 5
```

这个func的变量，被声明成了一个返回add(2,3)调用结果的函数。

### 代码块=立即函数

```jsx
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

```jsx
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

闭包的定义与JavaScript的语法极其类似。我们可以通过闭包获得上下文的环境。我们也可以返回一个函数，这个函数捕获了当前作用域的变量。

```jsx
f counter():->int{

}
```

# 运算 Operation

字符串拼接： `+`

```jsx
strrrr = "Silk" + " is" + " a" + "experimental" + " language"
```

### 支持自增与自减算符。 Supports increment and decrement operators.

```jsx
a = 1
a += 1
a++ 
b = a-- //3
log(a) // 2
```

# 作用域 Scope / Code Block

你可以随意创建作用域 You can create scopes at will.

我们规定: 一个代码块会形成一个作用域。某个作用域的变量从外向里是可见的，从里向外是不可见的。这一点和很多其他的语言是一样的。

```jsx

a = 1
{
	{
		{
			b = 2
		}
	}
}
log(b)  // none
```

代码块即表示一个作用域，其中可以有返回值。

A code block represents a scope in which there can be a return value.

```jsx
str_hlo = {
	-> "hello"
}
```

我们来通过作用域，来了解与Silk语言中函数的关系

Let's look at the scope to understand the relationship with functions in the Silk language

```jsx
//这是一个作用域，你也可以看成是一个“代码块”，或者是一个立即执行函数 
// This is a scope, which you can also think of as a "block of code", or an immediate execution function
arr = {
	-> [1,2,3,"value"]
}
/* Similiar in Javascript:
arr = (()=>{
	return [1,2,3,"value"]
})()
*/
// in Silk
arr = (){
 -> [1,2,3,"value"]
}()
// 没错。Silk语言在处理作用域的时候，其实就是对函数的一次声明+立即执行。所以大括号实际上是一个语法糖。
```

# 类型约束 Type-Bond

Silk语言是属于**基于类型的语言***，*但是我们不限制死类型，我们选择了一种可编程化的基于**`类型约束`**的方法，用于对类型进行诠释。

Silk is a type-based language, but we do not restrict dead types, and we have chosen a programmable type-constrained approach for interpreting types.

- *类型和约束 difference between type and type-bond*
  
    在Silk语言中我们会说某个变量是str类型，实际上是指这个变量实现了str类型的单约束。所以某个变量是什么类型和某个变量属于这个这个类型的集合的单约束是等价的。我们只是因为习惯所以会这样称呼。但是在逻辑上{type}≠type。
    
    In Silk we say that a variable is of type str, which actually means that the variable implements a single constraint of type str. So the single constraint of what type a variable is and that a variable belongs to a set of that type is equivalent. We just call it that way because of habit. But logically {type}≠type.
    
- 我们默认一个变量对应一个约束，而不是一个变量对应一个类型。（除非是单约束，我们以某个变量是某个类型来称呼）
  
    We default to one variable for one constraint, not one variable for one type. (Unless it's a single constraint, we call a variable a certain type)
    
- 类型是一个元素，而约束是这个类型的某个集合。当然可以是空集。比如none。
  
    A type is an element, and a constraint is a collection of that type. It can be an empty set, of course. Like none.
    

```jsx
类型： int , bol, str … //each one is a single type.

约束： :int,:int|bol,:str|int|bol …  //a combination of one type or types.
```

*类型约束语法：*

```jsx
: type_1 | type_2 | type3 ...
```

我们把包括冒号在内的后面的一个或多个类型，表示为这个变量的**类型约束**。你也可以理解成其他语言中的类型声明。

## 显式约束

```jsx
hello:str = "world" // str变量实现了{str}的单约束。实际上和属于str类型在底层上是等价的。
number:int|flt = 1.0 //number的约束是：{int,flt}
```

### 约束可以使用bond关键字来定义一个名称。

当我们要形容一个约束，我们可以使用bond给这个约束命名。标准库里的默认约束是小写。但是我们建议自定义约束使用大写的方式。

```jsx
/// num 是标准库里的一个约束
bond num = int | flt
///
```

## 推导约束 Bond Deduce

```jsx
hello = "world" //如果不显式地约束类型，编译器将会**推导**(deduce)这个变量的类型。并为它加上约束.
is_of(hello)  // str

a = 1 //自推导a的约束是： int
a = '2' // Bond_Err: Int of id(a) can't be transformed to Bond Chr. 

b:int|chr = 2
b ='4'   //pass
```

```jsx
bond changable = int | chr | flt | str
dyn_variable:changable = '1'
log(dyn_variable)  // '1'
dyn_variable = "2"  //这里编译器不会报错。这个变量将会被改变
dyn_variable = 3.0  // 同理
log(is_of(dyn_variable)) // int|chr|flt|str 这里返回了一个约束请注意，变量是对应约束的，二部输出前面又赋值了3.0,这里就是flt.

log(dyn_variable===3.0) //三个等号是比较的实际上是:变量的约束是否相等以及值是否相等
log(is_of(dyn_variable)==is_of(3.0)&& get_value(dyn_variable)get_value(3.0))
// int|chr|flt|str == flt -> false
// 3.0==3.0  -> true
// false&&true ->false

log(dyn_variable==3.0)//我们现在可能会担心这个问题，到底怎样怎样才能判断两个是否相等？！
// 这个语句会返回true。 因为 == 等号 比较的实际上是他们的隐类型和值。
```

## 约束的“能力”

我们有这样一个类型约束

```jsx
bond str_bol = bol | str
```

如果我想要实现这样的逻辑：让StrBol可以和一个字符串相等比较操作。

expect:

```jsx
foo:StrBol = true
log(foo == "true") //true
log(foo == "false") //false
foo = false
log(foo == "true") //false
log(foo == "false") //true
```

那么该怎么办呢？我们现在可以给StrBol添加能力。

```jsx
 ability StrBol :: Eq { //给str_bol 类型 ::表示实现某个能力
	f eq(other str):bol { //我们现在可以修改相等判断的逻辑
			if is_of(@.reveal())=="bol" { //这里@代指自己，很好理解
					-> self.to_s() == other 
			}el{
					-> Eq.default(self,other)
			}
	}
}
```

例子：实现转换的能力

```jsx
///现在我要定义一个中文的布尔类型:
bond ChineseBool = bol
```

我想要添加一个转换为单约束str（其实就是转换为str“类型”）的方法

```jsx
ability ChineseBool {
	f to_s():str{
		if @.reveal()==true {-> "对"}
		el {->"错"}
	}
}
```

接下来使用隐式约束转换:

```jsx
dui:ChineseBool = false
dui:str = dui
log(dui) // 返回 "对"
```

## 单约束bond

```jsx
bond CustomStr = str
```

现在我定义了一个bond，这个bond就是str。

```jsx
str_1:CustomStr = "hello"
str_2:str = "hello"
log(str_1==str_2) // true
log(str_1==str_2) // false
```

**注意**

在Silk中，一个变量只能是一种约束。而不是一个类型。我们的基本类型。实际上如果在变量声明中，将会成为一个单约束{type}。你可以理解成：类型是最基础的元素type。变量不拥有类型，变量只能是某一种类型的非空集合{type1,type2,…}。

```jsx
example: bol | string = "false"
example = "true" //请问在这里，example 属于哪个约束？
```

如果你对于其他编译型语言很熟练，哪怕是Typescript，你应该会认为，example现在属于{str}。但是实际上，exampl的约束在一开始就声明了，即 bol | string 或者是一个{bol,string}的集合。所以变量并不是在某一时刻属于约束中的某一个类型，而是变量一直拥有这个约束，它不会属于任何一个约束中的类型，它本身就是这个「约束」的类型。

```jsx
bond BolOrStr =bol | string
example: BolOrStr = "true"
log(is_of(example))  // BolOrStr is_of可以找到某个变量的构造函数，这里我们发现他的构造函数不是
log(is_of(example)==is_of("true")) //false
log(example=="true") //true 那么为什么这个比较
```

我们也可以定义成完全无约束的类型。但是这将会带来开销

```jsx
any_type:any = none //空
any_type = 1 // 数字
any_type = *{
} //  表
any_type = (){log("a")} 
```

函数中也存在约束。

```jsx

F // param:undefined ,return : undefined
()->int // param: none , return: int
->int // param: undefined , return: int
(a int)->int //param: a: int  , return :int
(a)->int //param: a: no_restrict  , return :int
(a,b)->int param: a,b : no_restrict , return: int 
(a,b,..)->int param: more than a,b , return : int
-> _ //param: undefined, return 

// 我们可以定义一个函数的约束
bond add_bond = (int,int)->int
add:add_bond =(x,y){
} 
```