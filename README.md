# 细说Rust错误处理

- [细说Rust错误处理](#%e7%bb%86%e8%af%b4rust%e9%94%99%e8%af%af%e5%a4%84%e7%90%86)
  - [1. 前言](#1-%e5%89%8d%e8%a8%80)
  - [2. 背景](#2-%e8%83%8c%e6%99%af)
  - [3. unwrap的危害!](#3-unwrap%e7%9a%84%e5%8d%b1%e5%ae%b3)
  - [4. 对比语言处理错误](#4-%e5%af%b9%e6%af%94%e8%af%ad%e8%a8%80%e5%a4%84%e7%90%86%e9%94%99%e8%af%af)
    - [4.1 golang的错误处理演示](#41-golang%e7%9a%84%e9%94%99%e8%af%af%e5%a4%84%e7%90%86%e6%bc%94%e7%a4%ba)
    - [4.2 Rust 错误处理示例](#42-rust-%e9%94%99%e8%af%af%e5%a4%84%e7%90%86%e7%a4%ba%e4%be%8b)
  - [5. Rust中的错误处理](#5-rust%e4%b8%ad%e7%9a%84%e9%94%99%e8%af%af%e5%a4%84%e7%90%86)
  - [6. 自定义Error转换:From](#6-%e8%87%aa%e5%ae%9a%e4%b9%89error%e8%bd%ac%e6%8d%a2from)
  - [7. 重命名Result](#7-%e9%87%8d%e5%91%bd%e5%90%8dresult)
  - [8. Option转换](#8-option%e8%bd%ac%e6%8d%a2)
  - [9. 避免unwrap()](#9-%e9%81%bf%e5%85%8dunwrap)
  - [10. 自定义Error同级转换](#10-%e8%87%aa%e5%ae%9a%e4%b9%89error%e5%90%8c%e7%ba%a7%e8%bd%ac%e6%8d%a2)
  - [11. Error常见开源库](#11-error%e5%b8%b8%e8%a7%81%e5%bc%80%e6%ba%90%e5%ba%93)
  - [12. 参考链接](#12-%e5%8f%82%e8%80%83%e9%93%be%e6%8e%a5)
  - [13 错误处理实战](#13-%e9%94%99%e8%af%af%e5%a4%84%e7%90%86%e5%ae%9e%e6%88%98)
  - [14. 总结](#14-%e6%80%bb%e7%bb%93)

![handle-error.png](https://github.com/baoyachi/rust-error-handle/raw/master/handle_error.png)

原文地址:[https://github.com/baoyachi/rust-error-handle](https://github.com/baoyachi/rust-error-handle)


## 1. 前言
这篇文章写得比较长，全文读完大约需要15-20min，如果对`Rust`的错误处理不清楚或还有些许模糊的同学，请静下心来细细阅读。当读完该篇文章后，可以说对`Rust`的错误处理可以做到掌握自如。

笔者花费较长篇幅来描述**错误处理**的来去，详细介绍其及一步步梳理内容，望大家能耐心读完后对大家有所帮助。当然，在写这篇文章之时，也借阅了大量互联网资料，详见链接见底部**参考链接**

掌握好`Rust`的错误设计，不仅可以提升我们对错误处理的认识，对代码结构、层次都有很大的帮助。那废话不多说，那我们开启这段阅读之旅吧😄！

## 2. 背景
笔者在写这篇文章时，也翻阅一些资料关于`Rust`的错误处理资料，多数是对其一笔带过，导致之前接触过其他语言的新同学来说，上手处理`Rust`的错误会有**当头棒喝**的感觉。找些资料发现**unwrap()**也可以解决问题，然后心中暗自窃喜，程序在运行过程中，**因为忽略检查或程序逻辑判断**，导致某些情况，程序**panic**。这可能是我们最不愿看到的现象，遂又回到起点，重新去了解`Rust`的错误处理。

这篇文章，通过一步步介绍，让大家清晰知道`Rust`的错误处理的究竟。介绍在`Rust`中的错误使用及如何处理错误，以及在实际工作中关于其使用技巧。

## 3. unwrap的危害!
下面我们来看一段代码,执行一下：

```rust
fn main() {
    let path = "/tmp/dat";
    println!("{}", read_file(path));
}

fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}
```
程序执行结果：
```bash
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/libcore/result.rs:1188:5
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /Users/runner/.cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.40/src/backtrace/libunwind.rs:88
  ...
  15: rust_sugar::read_file
             at src/main.rs:7
  16: rust_sugar::main
             at src/main.rs:3
  ...
  25: rust_sugar::read_file
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

什么，因为`path`路径不对，程序竟然崩溃了，这个是我们不能接受的！

**unwrap()** 这个操作在rust代码中，应该看过很多这种代码，甚至此时我们正在使用它。它主要用于`Option`或`Result`的打开其包装的结果。常常我们在代码中，使用简单，或快速处理，使用了 **unwrap()** 的操作，但是，它是一个非常危险的信号!

可能因为**没有程序检查或校验**，潜在的bug可能就出现其中，使得我们程序往往就**panic**了。这可能使我们最不愿看到的现象。

在实际项目开发中，程序中可能充斥着大量代码，我们很难避免**unwrap()**的出现，为了解决这种问题，我们通过做**code review**,或使用脚本工具检查其降低其出现的可能性。

通常每个项目都有一些约束，或许：在大型项目开发中， 不用**unwrap()** 方法，使用其他方式处理程序，**unwrap()** 的不出现可能会使得程序的健壮性高出很多。

这里前提是团队或大型项目，如果只是写一个简单例子（demo）就不在本篇文章的讨论范畴。因为一个Demo的问题，可能只是快速**示范或演示**，不考虑程序健壮性, **unwrap()** 的操作可能会更方便代码表达。

可能有人会问，我们通常跑程序**unit test**，其中的很多**mock**数据会有 **unwrap()** 的操作，我们只是为了在单元测试中使得程序简单。这种也能不使用吗？答案：是的，完全可以不使用 **unwrap()** 也可以做到的。

## 4. 对比语言处理错误
说到**unwrap()**，我们不得不提到`rust`的错误处理，**unwrap()** 和`Rust`的错误处理是密不可分的。

### 4.1 golang的错误处理演示

如果了解`golang`的话，应该清楚下面这段代码的意思：
```go
package main

import (
    "io/ioutil"
    "log"
)

func main() {
    path := "/tmp/dat"  //文件路径
    file, err := readFile(path) 
    if err != nil {
        log.Fatal(err) //错误打印
    }
    println("%s", file) //打印文件内容
}

func readFile(path string) (string, error) {
    dat, err := ioutil.ReadFile(path)  //读取文件内容
    if err != nil {  //判断err是否为nil
        return "", err  //不为nil,返回err结果
    }
    return string(dat), nil  //err=nil,返回读取文件内容
}
```
我们执行下程序，打印如下。执行错误，当然，因为我们给的文件路径不存在，程序报错。
```bash
2020/02/24 01:24:04 open /tmp/dat: no such file or directory
```

这里，`golang`采用多返回值方式，程序报错返回错误问题，通过判断 **err!=nil** 来决定程序是否继续执行或终止该逻辑。当然，如果接触过`golang`项目时，会发现程序中大量充斥着`if err!=nil`的代码，对此网上有对`if err!=nil`进行了很多讨论，因为这个不在本篇文章的范畴中，在此不对其追溯、讨论。

### 4.2 Rust 错误处理示例
对比了`golang`代码，我们对照上面的例子，看下在`Rust`中如何编写这段程序，代码如下：
```rust
fn main() {
    let path = "/tmp/dat";  //文件路径
    match read_file(path) { //判断方法结果
        Ok(file) => { println!("{}", file) } //OK 代表读取到文件内容，正确打印文件内容
        Err(e) => { println!("{} {}", path, e) } //Err代表结果不存在，打印错误结果
    }
}

fn read_file(path: &str) -> Result<String,std::io::Error> { //Result作为结果返回值
    std::fs::read_to_string(path) //读取文件内容
}
```
当前，因为我们给的文件路径不存在，程序报错，打印内容如下：
```bash
No such file or directory (os error 2)
```

在`Rust`代表中，`Result`是一个`enum`枚举对象,部分源码如下：

```rust
pub enum Result<T, E> {
    /// Contains the success value
    Ok(#[stable(feature = "rust1", since = "1.0.0")] T),

    /// Contains the error value
    Err(#[stable(feature = "rust1", since = "1.0.0")] E),
}
```
通常我们使用`Result`的枚举对象作为程序的返回值，通过`Result`来判断其结果，我们使用`match`匹配的方式来获取`Result`的内容，判断正常（Ok）或错误(Err)。

或许，我们大致向上看去，`golang`代码和`Rust`代码没有本质区别，都是采用返回值方式，给出程序结果。下面我们就对比两种语言说说之间区别：

* `golang`采用多返回值方式，我们在拿到目标结果时（上面是指文件内容*file*），需要首先对`err`判断是否为`nil`,并且我们在`return`时，需要给**多返回值**分别赋值，调用时需要对 `if err!=nil` 做结果判断。
* `Rust`中采用`Result`的枚举对象做结果返回。枚举的好处是：多选一。因为`Result`的枚举类型为`Ok`和`Err`，使得我们每次在返回`Result`的结果时，要么是`Ok`,要么是`Err`。它不需要`return`结果同时给两个值赋值，这样的情况只会存在一种可能性: **Ok or Err** 。
* golang的函数调用需要对 `if err!=nil`做结果判断，因为这段代码 判断是**手动逻辑**，往往我们可能因为疏忽，导致这段逻辑缺失，缺少校验。当然，我们在编写代码期间可以通过某些工具 `lint` 扫描出这种潜在bug。
* `Rust`的`match`判断是自动打开，当然你也可以选择忽略其中某一个枚举值,我们不在此说明。

可能有人发现，如果我有多个函数，需要多个函数的执行结果，这样需要`match`代码多次，代码会不会是一坨一坨，显得代码很臃肿，难看。是的，这个问题提出的的确是有这种问题，不过这个在后面我们讲解的时候，会通过程序语法糖避免多次`match`多次结果的问题，不过我们在此先不叙说，后面将有介绍。


## 5. Rust中的错误处理
前面不管是`golang`还是`Rust`采用`return`返回值方式，两者都是为了解决程序中错误处理的问题。好了，前面说了这么多，我们还是回归正题：Rust中是如何对错误进行处理的？

要想细致了解`Rust`的错误处理，我们需要了解`std::error::Error`，该trait的内部方法，部分代码如下：
参考链接：[https://doc.rust-lang.org/std/error/trait.Error.html](https://doc.rust-lang.org/std/error/trait.Error.html)

```rust
pub trait Error: Debug + Display {

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    #[rustc_deprecated(since = "1.33.0", reason = "replaced by Error::source, which can support \
                                                   downcasting")]

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> { None }

    #[doc(hidden)]
    fn type_id(&self, _: private::Internal) -> TypeId where Self: 'static {
        TypeId::of::<Self>()
    }

    #[unstable(feature = "backtrace", issue = "53487")]
    fn backtrace(&self) -> Option<&Backtrace> {
        None
    }
}
```

* `description()`在文档介绍中，尽管使用它不会导致编译警告，但新代码应该实现`impl Display` ，新`impl`的可以省略，不用实现该方法, 要获取字符串形式的错误描述，请使用`to_string()`。
* `cause()`在**1.33.0**被抛弃，取而代之使用`source()`方法，新`impl`的不用实现该方法。
* `source()`此错误的低级源，如果内部有错误类型`Err`返回：`Some(e)`,如果没有返回：`None`。
  
  * 如果当前`Error`是低级别的`Error`,并没有**子Error**,需要返回`None`。介于其本身默认有返回值`None`，可以**不覆盖**该方法。
  * 如果当前`Error`包含**子Error**,需要返回**子Error**：`Some(err)`,需要**覆盖**该方法。
* `type_id()`该方法被隐藏。
* `backtrace()`返回发生此错误的堆栈追溯，因为标记`unstable`，在`Rust`的`stable`版本不被使用。
* 自定义的`Error`需要**impl std::fmt::Debug**的trait,当然我们只需要在默认对象上添加注解：`#[derive(Debug)]`即可。


总结一下，自定义一个`error`需要实现如下几步：

* 手动实现impl `std::fmt::Display`的trait,并**实现** `fmt(...)`方法。
* 手动实现impl `std::fmt::Debug`的`trait`，一般直接添加注解即可：`#[derive(Debug)]`
* 手动实现impl `std::error::Error`的`trait`,并根据自身`error`级别是否**覆盖**`std::error::Error`中的`source()`方法。

下面我们自己手动实现下`Rust`的**自定义错误:CustomError**
```rust
use std::error::Error;

///自定义类型 Error,实现std::fmt::Debug的trait
#[derive(Debug)]
struct CustomError {
    err: ChildError,
}

///实现Display的trait，并实现fmt方法
impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CustomError is here!")
    }
}

///实现Error的trait,因为有子Error:ChildError,需要覆盖source()方法,返回Some(err)
impl std::error::Error for CustomError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.err)
    }
}


///子类型 Error,实现std::fmt::Debug的trait
#[derive(Debug)]
struct ChildError;

///实现Display的trait，并实现fmt方法
impl std::fmt::Display for ChildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ChildError is here!")
    }
}

///实现Error的trait,因为没有子Error,不需要覆盖source()方法
impl std::error::Error for ChildError {}

///构建一个Result的结果，返回自定义的error:CustomError
fn get_super_error() -> Result<(), CustomError> {
    Err(CustomError { err: ChildError })
}

fn main() {
    match get_super_error() {
        Err(e) => {
            println!("Error: {}", e);
            println!("Caused by: {}", e.source().unwrap());
        }
        _ => println!("No error"),
    }
}
```
* `ChildError`为子类型`Error`,**没有覆盖**`source()`方法，空实现了`std::error::Error`
* `CustomError`有子类型`ChildError`,**覆盖**了`source()`,并返回了子类型Option值：`Some(&self.err)`

运行执行结果，显示如下：
```bash
Error: CustomError is here!
Caused by: ChildError is here!
```
至此，我们就了解了如何实现`Rust`中**自定义Error**了。


## 6. 自定义Error转换:From
上面我们说到，函数返回`Result`的结果时，需要获取函数的返回值是成功(Ok)还是失败(Err)，需要使用`match`匹配，我们看下多函数之间调用是如何解决这类问题的？假设我们有个场景：
* 读取一文件
* 将文件内容转化为`UTF8`格式
* 将转换后格式内容转为`u32`的数字。

所以我们有了下面三个函数(省略部分代码)：
```rust
...

///读取文件内容
fn read_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

/// 转换为utf8内容
fn to_utf8(v: &[u8]) -> Result<&str, std::str::Utf8Error> {
    std::str::from_utf8(v)
}

/// 转化为u32数字
fn to_u32(v: &str) -> Result<u32, std::num::ParseIntError> {
    v.parse::<u32>()
}
```

最终，我们得到`u32`的数字，对于该场景如何组织我们代码呢？

* `unwrap()`直接打开三个方法，取出值。这种方式太暴力，并且会有`bug`,造成程序`panic`,不被采纳。
* `match`匹配，如何返回OK,继续下一步，否则报错终止逻辑，那我们试试。

参考代码如下:
```rust
fn main() {
    let path = "./dat";
    match read_file(path) {
        Ok(v) => {
            match to_utf8(v.as_bytes()) {
                Ok(u) => {
                    match to_u32(u) {
                        Ok(t) => {
                            println!("num:{:?}", u);
                        }
                        Err(e) => {
                            println!("{} {}", path, e)
                        }
                    }
                }
                Err(e) => {
                    println!("{} {}", path, e)
                }
            }
        }
        Err(e) => {
            println!("{} {}", path, e)
        }
    }
}

///读取文件内容
fn read_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

/// 转换为utf8内容
fn to_utf8(v: &[u8]) -> Result<&str, std::str::Utf8Error> {
    std::str::from_utf8(v)
}

/// 转化为u32数字
fn to_u32(v: &str) -> Result<u32, std::num::ParseIntError> {
    v.parse::<u32>()
}
```

天啊，虽然是实现了上面场景的需求，但是代码犹如叠罗汉，程序结构越来越深啊，这个是我们没法接受的！`match`匹配导致程序如此**不堪一击**。那么有没有第三种方法呢？当然是有的：`From`转换。

前面我们说到如何**自定义的Error**,如何我们将上面三个`error`收纳到我们**自定义的Error**中，将它们三个`Error`变成**自定义Error**的**子Error**，这样我们对外的`Result`统一返回**自定义的Error**。这样程序应该可以改变点什么，我们来试试吧。
```rust
#[derive(Debug)]
enum CustomError {
    ParseIntError(std::num::ParseIntError),
    Utf8Error(std::str::Utf8Error),
    IoError(std::io::Error),
}
impl std::error::Error for CustomError{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            CustomError::IoError(ref e) => Some(e),
            CustomError::Utf8Error(ref e) => Some(e),
            CustomError::ParseIntError(ref e) => Some(e),
        }
    }
}

impl Display for CustomError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            CustomError::IoError(ref e) => e.fmt(f),
            CustomError::Utf8Error(ref e) => e.fmt(f),
            CustomError::ParseIntError(ref e) => e.fmt(f),
        }
    }
}

impl From<ParseIntError> for CustomError {
    fn from(s: std::num::ParseIntError) -> Self {
        CustomError::ParseIntError(s)
    }
}

impl From<IoError> for CustomError {
    fn from(s: std::io::Error) -> Self {
        CustomError::IoError(s)
    }
}

impl From<Utf8Error> for CustomError {
    fn from(s: std::str::Utf8Error) -> Self {
        CustomError::Utf8Error(s)
    }
}
```

* `CustomError`为我们实现的**自定义Error**
* `CustomError`有三个**子类型Error**
* `CustomError`分别实现了三个**子类型Error** `From`的trait,将其类型包装为**自定义Error**的子类型

好了，有了自定义的`CustomError`，那怎么使用呢? 我们看代码：

```rust
use std::io::Error as IoError;
use std::str::Utf8Error;
use std::num::ParseIntError;
use std::fmt::{Display, Formatter};


fn main() -> std::result::Result<(),CustomError>{
    let path = "./dat";
    let v = read_file(path)?;
    let x = to_utf8(v.as_bytes())?;
    let u = to_u32(x)?;
    println!("num:{:?}",u);
    Ok(())
}

///读取文件内容
fn read_file(path: &str) -> std::result::Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

/// 转换为utf8内容
fn to_utf8(v: &[u8]) -> std::result::Result<&str, std::str::Utf8Error> {
    std::str::from_utf8(v)
}

/// 转化为u32数字
fn to_u32(v: &str) -> std::result::Result<u32, std::num::ParseIntError> {
    v.parse::<u32>()
}


#[derive(Debug)]
enum CustomError {
    ParseIntError(std::num::ParseIntError),
    Utf8Error(std::str::Utf8Error),
    IoError(std::io::Error),
}
impl std::error::Error for CustomError{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            CustomError::IoError(ref e) => Some(e),
            CustomError::Utf8Error(ref e) => Some(e),
            CustomError::ParseIntError(ref e) => Some(e),
        }
    }
}

impl Display for CustomError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            CustomError::IoError(ref e) => e.fmt(f),
            CustomError::Utf8Error(ref e) => e.fmt(f),
            CustomError::ParseIntError(ref e) => e.fmt(f),
        }
    }
}

impl From<ParseIntError> for CustomError {
    fn from(s: std::num::ParseIntError) -> Self {
        CustomError::ParseIntError(s)
    }
}

impl From<IoError> for CustomError {
    fn from(s: std::io::Error) -> Self {
        CustomError::IoError(s)
    }
}

impl From<Utf8Error> for CustomError {
    fn from(s: std::str::Utf8Error) -> Self {
        CustomError::Utf8Error(s)
    }
}
```

其实我们主要关心的是这段代码：
```rust
fn main() -> Result<(),CustomError>{
    let path = "./dat";
    let v = read_file(path)?;
    let x = to_utf8(v.as_bytes())?;
    let u = to_u32(x)?;
    println!("num:{:?}",u);
    Ok(())
}
```
我们使用了`?`来替代原来的`match`匹配的方式。`?`使用问号作用在函数的结束，意思是：

* 程序接受了一个`Result<(),CustomError>`自定义的错误类型。
* 当前如果函数结果错误，程序自动抛出`Err`自身错误类型，并包含相关自己类型错误信息，因为我们做了`From`转换的操作，该函数的自身类型错误会通过实现的`From`操作自动转化为`CustomError`的自定义类型错误。
* 当前如果函数结果正确，继续之后逻辑，直到程序结束。

这样，我们通过`From`和`?`解决了之前`match`匹配代码层级深的问题，因为这种转换是**无感知**的，使得我们在处理好错误类型后，只需要关心我们的目标值即可，这样不需要显示对`Err(e)`的数据单独处理，使得我们在函数后添加`?`后，程序一切都是自动了。

还记得我们之前讨论在对比`golang`的错误处理时的:`if err!=nil`的逻辑了吗，这种因为用了`?`语法糖使得该段判断将不再存在。

另外，我们还注意到，`Result`的结果可以作用在`main`函数上，

* 是的，`Result`的结果不仅能作用在`main`函数上
* `Result`还可以作用在单元测试上，这就是我们文中刚开始提到的：因为有了`Result`的作用，使得我们在程序中几乎可以完全摒弃`unwrap()`的代码块，使得程序更轻，大大减少潜在问题，程序组织结构更加清晰。
  
下面这是作用在单元测试上的`Result`的代码：
```rust
...

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_num() -> std::result::Result<(), CustomError> {
        let path = "./dat";
        let v = read_file(path)?;
        let x = to_utf8(v.as_bytes())?;
        let u = to_u32(x)?;
        assert_eq!(u, 8);
        Ok(())
    }
}
```

## 7. 重命名Result
我们在实际项目中，会大量使用如上的`Result`结果，并且`Result`的`Err`类型是我们`自定义错误`,导致我们写程序时会显得非常**啰嗦**、**冗余**
```rust
///读取文件内容
fn read_file(path: &str) -> std::result::Result<String, CustomError> {
    let val = std::fs::read_to_string(path)?;
    Ok(val)
}

/// 转换为utf8内容
fn to_utf8(v: &[u8]) -> std::result::Result<&str, CustomError> {
    let x = std::str::from_utf8(v)?;
    Ok(x)
}

/// 转化为u32数字
fn to_u32(v: &str) -> std::result::Result<u32, CustomError> {
    let i = v.parse::<u32>()?;
    Ok(i)
}
```
我们的程序中，会大量充斥着这种**模板代码**，`Rust`本身支持对类型自定义，使得我们只需要重命名`Result`即可:
```rust
pub type IResult<I> = std::result::Result<I, CustomError>; ///自定义Result类型：IResult
```
这样，凡是使用的是自定义类型错误的`Result`都可以使用`IResult`来替换`std::result::Result`的类型，使得简化程序，隐藏`Error`类型及细节，关注目标主体，代码如下：
```rust
///读取文件内容
fn read_file(path: &str) -> IResult<String> {
    let val = std::fs::read_to_string(path)?;
    Ok(val)
}

/// 转换为utf8内容
fn to_utf8(v: &[u8]) -> IResult<&str> {
    let x = std::str::from_utf8(v)?;
    Ok(x)
}

/// 转化为u32数字
fn to_u32(v: &str) -> IResult<u32> {
    let i = v.parse::<u32>()?;
    Ok(i)
}
```
将`std::result::Result<I, CustomError>` 替换为：`IResult<I>`类型

当然，会有人提问，如果是多参数类型怎么处理呢，同样，我们只需将`OK`类型变成 **tuple** `(I,O)`类型的多参数数据即可，大概这样：
```rust
pub type IResult<I, O> = std::result::Result<(I, O), CustomError>;
```

使用也及其简单，只需要返回：**I**,**O**的具体类型,举个示例：
```rust
fn foo() -> IResult<String, u32> {
    Ok((String::from("bar"), 32))
}
```

使用重命名类型的`Result`，使得我们错误类型统一，方便处理。在实际项目中，可以大量看到这种例子的存在。

## 8. Option转换 
我们知道，在`Rust`中，需要使用到`unwrap()`的方法的对象有`Result`,`Option`对象。我们看下`Option`的大致结构：
```rust
pub enum Option<T> {
    /// No value
    #[stable(feature = "rust1", since = "1.0.0")]
    None,
    /// Some value `T`
    #[stable(feature = "rust1", since = "1.0.0")]
    Some(#[stable(feature = "rust1", since = "1.0.0")] T),
}
```
`Option`本身是一个`enum`对象，如果该函数（方法）调用结果值没有值，返回`None`,反之有值返回`Some(T)`

如果我们想获取`Some(T)`中的`T`,最直接的方式是：`unwrap()`。我们前面说过，使用`unwrap()`的方式太过于暴力，如果出错，程序直接`panic`，这是我们最不愿意看到的结果。

 Ok,那么我们试想下, 利用`Option`能使用`?`语法糖吗？如果能用`?`转换的话，是不是代码结构就更简单了呢？我们尝试下,代码如下：

```rust

#[derive(Debug)]
enum Error {
    OptionError(String),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::OptionError(ref e) => e.fmt(f),
        }
    }
}

pub type Result<I> = std::result::Result<I, Error>;


fn main() -> Result<()> {
    let bar = foo(60)?;
    assert_eq!("bar", bar);
    Ok(())
}

fn foo(index: i32) -> Option<String> {
    if index > 60 {
        return Some("bar".to_string());
    }
    None
}
```

执行结果报错：
```bash
error[E0277]: `?` couldn't convert the error to `Error`
  --> src/main.rs:22:22
   |
22 |     let bar = foo(60)?;
   |                      ^ the trait `std::convert::From<std::option::NoneError>` is not implemented for `Error`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = note: required by `std::convert::From::from`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
error: could not compile `hyper-define`.
```
提示告诉我们没有转换`std::convert::From<std::option::NoneError>`，但是`NoneError`本身是`unstable`，这样我们没法通过`From`转换为**自定义Error**。

本身，在`Rust`的设计中，关于`Option`和`Result`就是一对孪生兄弟一样的存在，`Option`的存在可以忽略异常的细节，直接关注目标主体。当然，`Option`也可以通过内置的组合器`ok_or()`方法将其变成`Result`。我们大致看下实现细节：
```rust
impl<T> Option<T> {
    pub fn ok_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Some(v) => Ok(v),
            None => Err(err),
        }
    }
}    
```
这里通过`ok_or()`方法通过接收一个**自定义Error**类型，将一个`Option`->`Result`。好的，变成`Result`的类型，我们就是我们熟悉的领域了，这样处理起来就很灵活。

关于`Option`的其他处理方式，不在此展开解决，详细的可看下面链接：

延伸链接：[https://stackoverflow.com/questions/59568278/why-does-the-operator-report-the-error-the-trait-bound-noneerror-error-is-no](https://stackoverflow.com/questions/59568278/why-does-the-operator-report-the-error-the-trait-bound-noneerror-error-is-no)

## 9. 避免unwrap()
有人肯定会有疑问，如果需要判断的逻辑，又不用`?`这种操作，怎么取出`Option`或`Result`的数据呢，当然点子总比办法多，我们来看下`Option`如何做的：
```rust
fn main() {
    if let Some(v) = opt_val(60) {
        println!("{}", v);
    }
}

fn opt_val(num: i32) -> Option<String> {
    if num >= 60 {
        return Some("foo bar".to_string());
    }
    None
}
```

是的，我们使用`if let Some(v)`的方式取出值，当前`else`的逻辑就可能需要自己处理了。当然，`Option`可以这样做，`Result`也一定可以:

```rust
fn main() {
    if let Ok(v) = read_file("./dat") {
        println!("{}", v);
    }
}

fn read_file(path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}
```
只不过，在处理`Result`的判断时，使用的是`if let Ok(v)`，这个和`Option`的`if let Some(v)`有所不同。

到这里，`unwrap()`的代码片在项目中应该可以规避了。补充下，这里强调了几次规避，就如前所言：**团队风格统一，方便管理代码，消除潜在危机**。

## 10. 自定义Error同级转换
我们在项目中，一个函数（方法）内部会有多次`Result`的结果判断：`?`,假设我们自定义的全局Error名称为：`GlobalError`。

这时候，如果全局有一个`Error`可能就会出现如下错误：

```rust
std::convert::From<error::GlobalError<A>>` is not implemented for `error::GlobalError<B>
```

意思是：我们自定义的`GlobalError`没有通过From<GlobalError<T>>转换我们自己自定义的`GlobalError`，那这样，就等于**自己转换自己**。注意：

* 第一：这是我们不期望这样做的。
* 第二：遇到这种自己转换自己的`T`类型很多，我们不可能把出现的`T`类型通通实现一遍。
这时候，我们考虑自定义另一个Error了，假设我们视为：`InnnerError`,我们全局的Error取名为：`GlobalError`，我们在遇到上面错误时，返回`Result<T,InnerError>`,这样我们遇到`Result<T,GlobalError>`时，只需要通过`From<T>`转换即可，代码示例如下：

```rust
impl From<InnerError> for GlobalError {
    fn from(s: InnerError) -> Self {
        Error::new(ErrorKind::InnerError(e))
    }
}
```

上面说的这种情况，可能会在项目中出现**多个自定义Error**,出现这种情况时，存在多个不同Error的`std::result::Result<T,Err>`的返回。这里的`Err`就可以根据我们业务现状分别反回不同类型了。最终，只要实现了`From<T>`的`trait`可转化为最终期望结果。

## 11. Error常见开源库
好了，介绍到这里，我们应该有了非常清晰的认知：关于如何处理`Rust`的错误处理问题了。但是想想上面的这些逻辑多数是模板代码，我们在实际中，大可不必这样。说到这里，开源社区也有了很多对错误处理库的支持，下面列举了一些：

* [https://github.com/rust-lang-nursery/failure](https://github.com/rust-lang-nursery/failure)
* [https://github.com/rust-lang-nursery/error-chain](https://github.com/rust-lang-nursery/error-chain)
* [https://github.com/dtolnay/anyhow](https://github.com/dtolnay/anyhow)
* [https://github.com/dtolnay/thiserror](https://github.com/dtolnay/thiserror)
* [https://github.com/tailhook/quick-error](https://github.com/tailhook/quick-error)


## 12. 参考链接
* [https://blog.burntsushi.net/rust-error-handling/](https://blog.burntsushi.net/rust-error-handling/)
* [https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/question-mark-in-main-and-tests.html](https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/question-mark-in-main-and-tests.html)
* [https://doc.rust-lang.org/rust-by-example/error/result.html](https://doc.rust-lang.org/rust-by-example/error/result.html)
* [https://doc.rust-lang.org/rust-by-example/error.html](https://doc.rust-lang.org/rust-by-example/error.html)
* [https://github.com/rust-lang/rust/issues/43301](https://github.com/rust-lang/rust/issues/43301)

## 13 错误处理实战
这个例子介绍了如何在`https://github.com/Geal/nom`中处理错误，这里就不展开介绍了，有兴趣的可自行阅读代码。

详细见链接：[https://github.com/baoyachi/rust-error-handle/blob/master/src/demo_nom_error_handle.rs](https://github.com/baoyachi/rust-error-handle/blob/master/src/demo_nom_error_handle.rs)

## 14. 总结
好了，经过上面的长篇大论，不知道大家是否明白如何自定义处理Error呢了。大家现在带着之前的已有的问题或困惑，赶紧实战下`Rust`的错误处理吧，大家有疑问或者问题都可以留言我，希望这篇文章对你有帮助。

文中代码详见:[https://github.com/baoyachi/rust-handle-error/tree/master/src](https://github.com/baoyachi/rust-handle-error/tree/master/src)

原文地址:[https://github.com/baoyachi/rust-error-handle](https://github.com/baoyachi/rust-error-handle)
