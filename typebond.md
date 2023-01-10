# Silk Type Bond

Silk将变量和类型的集合挂钩，而不是某个具体的类型。`i:int = 1` 其实这是声明了i属于一个int的类型约束。

约束的格式：

```tsx
bond Num = int | flt
```

Mynum是一个类型约束 代表了 约束某个变量属于int 或 flt。这个变量可以进行int 和 flt 之间的操作。

```tsx
num_a : Num = 1
num_a + 3.0 = 4
```

对一个变量无约束的声明，编译器将会自动赋予约束.

```tsx
variable_no_bond = "hello-world"
// 根据字符串字面量属于str的约束，推导变量约束为一个str.
```

你可以创建一个函数约束

```tsx
bond FuncReturnInt = ->int
```

下面是其他的约束写法：

undefined: 自动推测
none: 无
(str,int)->File / (F,Uint)->int 匿名名参数
(file_name str,mode int)->File / (f F,time Uint)->int 具名参数

| Function Bond         | param             | return    |
| --------------------- | ----------------- | --------- |
| F                     | undefined         | undefined |
| ()->int / (none)->int | none              | int       |
| ->int                 | undefined         | int       |
| (\_ , \_)->int        | two               | int       |
| _->int                | a: no_restrict    | int       |
| (a,b)->int            | a,b : no_restrict | int       |
| (a,b,..)->int         | more than a,b     | int       |
| -> _                  | undefined         | undefined |


1. `F` 仅约束是一个函数 与 ->_ 一样。但是F更加优雅
2. `->` int 仅约束返回值为int
3. `-> ->int` 约束返回一个仅约束返回值为int的函数
4. `(a int,b int)-> none`  具名约束，约束参数为两个int，并且名字需要是a,b. 无返回值。
5. `(_ , f F)-> int` 不约束第一个参数，第二个参数具名，第二个为一个函数。 返回一个int
6. `(port int , options Table)->Server` 约束第一个参数叫做port,为整型，之后的参数为一个选项Table.返回一个Server Box类型。
7. (...arr int[]) -> int[] 规定传入一个展开了的数组序列，元素为整型，返回一个数组
8. (iter ::CanIterator,param int )->int[] 约束第一个“具有” CanIterator的能力
9. (num_a int,num b,..) -> _ 不限制返回值，只限制函数前两个参数，暗示后面可以有大于0个参数。
10. (...T ::Ord[] , mode int) ->T[] 第一个参数是一个展开的数组，这个数组元素要实现Ord,mode为int，返回一个元素为T的数组（你可以把它理解成泛型了）
11. (T ::,mode int)->T[] 第一个参数为没有限制具有任何能力的类型参数，这是一个普遍意义上的泛型了。他返回一个T参数.
12. (T ::OutPut+Ord+Async)-> (T[])->int 传入一个同时具有OutPut和Ord和Async的泛型，返回一个：「 传入一个T的数组，返回int 」的函数
13. (..,xxx type) Error 错误写法 只能省略后面的参数。

我们来看这样一个delay函数的API:

```jsx
time.delay(->{ log("Eval!")},2*100) 
```

```jsx
f delay(func F,delay_time Uint):none 
```

 当使用f关键字来声明函数，其中的返回值约束直接写在()后面。

 另一种写法:

 ```jsx
 bond DelayApi = (func F,delay_time Unit)none 
//注意当使用“在变量上的约束的函数约束时，使用->加上约束。而不是:约束”
 delay:DelayApi

 ```

 ```jsx
f generate_delay(): DelayApi {.....}
//代表着返回一个函数，为什么是函数？因为DelayApi就是一个函数约束，所以只能是函数
f order_2_generate_delay(): ->DelayApi {.....}
//这是什么意思？我相信你应该已经知道了，这代表着返回一个函数，这个函数规定返回一个函数（DelayApi的约束的函数）。
 ```
    