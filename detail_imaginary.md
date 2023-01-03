# Get Started

```jsx
bond any = str | int | chr | flt | bol | Box
```

> silk 是一个面向函数式+组合式+继承的多功能语言。具有现代工程编程语言的特性，并且语法丝滑丰富，有助于我们高效率地编写工程式的高标准可维护的项目。
> 

# 变量 Variable


```jsx
a = 1
b = 3
c = '4'
d = 1 + c.to_i() // 5  

```


```jsx
const PI:flt = 3.14 //flt 是浮点数类型
const E:flt  = 1+1+1/2!+1/3!+1/4!+1/5!+1/6!+1/7!+1/8!
```

# 语句

### 条件判断

```jsx
// if
i = 0
if 1+1==2 { i = 1 } el {i = 3}
if 1+1==3 { i = 1 } elf 1-3==-2 {i = 3} el { i=0}
// 可以写成
i = 1+1==2 ? 1 or 3
```

```jsx
//switch
j='x'
when j {
	'x' {
			log("j is x !")
	}
    'y' {
		    log("j is x !")
	}
}
//
```

# 类型约束 Type Bond

*Silk语言是属于**基于类型的语言**，但是我们不限制死类型，我们选择了一种可编程化的基于**`类型约束`**的方法，用于对类型进行诠释。*

*类型和约束*

在Silk语言中我们会说某个变量是str类型，实际上是指这个变量实现了str类型的单约束。所以某个变量是什么类型和某个变量属于这个这个类型的集合的单约束是等价的。我们只是因为习惯所以会这样称呼。但是在逻辑上{type}≠type。

我们默认一个变量对应一个约束，而不是一个变量对应一个类型。（除非是单约束，我们以某个变量是某个类型来称呼）

类型是一个元素，而约束是这个类型的某个集合。当然可以是空集。比如none。

```jsx
类型： int , bol, str … //单个的类型

约束： :int,:int|bol,:str|int|bol …  //类型的组合
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

# 类型 Type

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

## 2. 非普通类型：Box Type

属于box是Silk中的一大特色的类型。box本身是可以继承的，同时也支持组合。

比如我们需要使用的HashMap，Set，BTreeSet，Array 这样的数据结构，就是通过Box实现的。下面是自定义的box：你可以把他理解成Java或者其他面向对象语言中的类。这几乎是无障碍过渡的。

```jsx
box Bird {
	name = "Polly"
	animal_type = "parrot"
	eat = -> "Polly is eating."
  fly = -> "Polly is flying."
  cry = -> "Polly is making a sound."
}
bird1 = Bird()
log(is_of(bird1)) // Bird
bird.fly()
bird.cry()
```

### 数组就是一个box类型。

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

我们来看一下这个Box的定义.
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

# 二. 运算 Operator

- 字符串拼接： `+`

```jsx
strrrr = "Silk" + " is" + " a" + "experimental" + " language"
```

### 支持自增与自减算符。

```jsx
a = 1
a += 1
a++ 
b = a-- //3
log(a) // 2
```

# 作用域 Scope

你可以随意创建作用域

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

```jsx
str_hlo = {
	-> "hello"
}
```

# 函数 Function

我们来看Silk中值得注意，也是很多对很多程序员极其重要的一部分——函数。

在silk文件中，我们可以定义一个main函数:

```jsx
f main(args str[]):int{
	log("hello! {}".(args))
}
```

### **使用箭头代替return关键字**

```jsx
f add(a int, b int):int{ -> a + b }
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
F // param:undefined ,return : undefined
()->int // param: none , return: int
->int // param: undefined , return: int
(a int)->int //param: a: int  , return :int
(a)->int //param: a: no_restrict  , return :int
(a,b)->int param: a,b : no_restrict , return: int 
(a,b,..)->int param: more than a,b , return : int
-> _ //param: undefined, return 

```

### 一等公民的函数

```jsx

func = { -> add(2,3) }
func() // 5
```

这个func的变量，被声明成了一个返回add(2,3)调用结果的函数。