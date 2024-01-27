## 简介

`unsafe` 的存在主要是因为 Rust 的静态检查太强了，但是强就算了，它还很保守，这就会导致当编译器在分析代码时，一些正确代码会因为编译器无法分析出它的所有正确性，结果将这段代码拒绝，导致编译错误。

`unsafe` 存在的另一个原因就是：它必须要存在。原因是计算机底层的一些硬件就是不安全的，如果 Rust 只允许你做安全的操作，那一些任务就无法完成。

## unsafe 的安全保证

`unsafe` 并不能绕过 Rust 的借用检查，也不能关闭任何 Rust 的安全检查规则，例如当你在 `unsafe` 中使用**引用**时，该有的检查一样都不会少。

`unsafe` 能给大家提供的也仅仅是之前的 5 种能力，在使用这 5 种能力时，编译器才不会进行内存安全方面的检查，最典型的就是使用**裸指针**(引用和裸指针有很大的区别)。

`unsafe` 不安全，但是该用的时候就要用，在一些时候，它能帮助我们大幅降低代码实现的成本。

而作为使用者，你的水平决定了 `unsafe` 到底有多不安全，因此你需要在 `unsafe` 中小心谨慎地去访问内存。

正因为此，写代码时要尽量控制好 `unsafe` 的边界大小，越小的 `unsafe` 越会让我们在未来感谢自己当初的选择。

除了控制边界大小，另一个很常用的方式就是在 `unsafe` 代码块外包裹一层 `safe` 的 API，例如一个函数声明为 safe 的，然后在其内部有一块儿是 `unsafe` 代码。

## 五种能力

`unsafe` 能赋予我们 5 种能力:

- 解引用裸指针
- 调用一个 `unsafe` 或外部的函数
- 访问或修改一个可变的[静态变量](https://course.rs/advance/global-variable.html#%E9%9D%99%E6%80%81%E5%8F%98%E9%87%8F)
- 实现一个 `unsafe` 特征
- 访问 `union` 中的字段

### 裸指针

指针(raw pointer，又称原生指针) 在功能上跟引用类似，同时它也需要显式地注明可变性。但是又和引用有所不同，裸指针长这样: `*const T` 和 `*mut T`，它们分别代表了不可变和可变。

- 可以绕过 Rust 的借用规则，可以同时拥有一个数据的可变、不可变指针，甚至还能拥有多个可变的指针
- 并不能保证指向合法的内存
- 可以是 `null`
- 没有实现任何自动的回收 (drop)

裸指针跟 C 指针是非常像的，使用它需要以牺牲安全性为前提，但我们获得了更好的性能，也可以跟其它语言或硬件打交道。

#### 基于引用创建裸指针

```rust
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;
```

`as` 可以用于强制类型转换，我们将引用 `&num / &mut num` 强转为相应的裸指针 `*const i32 / *mut i32`。

**创建裸指针是安全的行为，而解引用裸指针才是不安全的行为** :

```rust
fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
    }
}
```

#### 基于内存地址创建裸指针

我们基于现有的引用来创建裸指针，这种行为是很安全的。但是接下来的方式就不安全了：

```rust
use std::{slice::from_raw_parts, str::from_utf8_unchecked};

// 获取字符串的内存地址和长度
fn get_memory_location() -> (usize, usize) {
  let string = "Hello World!";
  let pointer = string.as_ptr() as usize;
  let length = string.len();
  (pointer, length)
}

// 在指定的内存地址读取字符串
fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
  unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
}

fn main() {
  let (pointer, length) = get_memory_location();
  let message = get_str_at_location(pointer, length);
  println!(
    "The {} bytes at 0x{:X} stored: {}",
    length, pointer, message
  );
  // 如果大家想知道为何处理裸指针需要 `unsafe`，可以试着反注释以下代码
  // let message = get_str_at_location(1000, 10);
}
```

#### 基于智能指针创建裸指针

```rust
let a: Box<i32> = Box::new(10);
// 需要先解引用a
let b: *const i32 = &*a;
// 使用 into_raw 来创建
let c: *const i32 = Box::into_raw(a);
```

#### 总结

使用裸指针可以让我们创建两个可变指针都指向同一个数据，如果使用安全的 Rust，你是无法做到这一点的，违背了借用规则，编译器会对我们进行无情的阻止。因此裸指针可以绕过借用规则，但是由此带来的数据竞争问题，就需要大家自己来处理了，总之，需要小心！

既然这么危险，为何还要使用裸指针？除了之前提到的性能等原因，还有一个重要用途就是跟 `C` 语言的代码进行交互( FFI )，在讲解 FFI 之前，先来看看如何调用 unsafe 函数或方法。

### 调用 unsafe 函数或方法

`unsafe` 函数从外表上来看跟普通函数并无区别，唯一的区别就是它需要使用 `unsafe fn` 来进行定义。这种定义方式是为了告诉调用者：当调用此函数时，你需要注意它的相关需求，因为 Rust 无法担保调用者在使用该函数时能满足它所需的一切需求。

#### 用安全抽象包裹 unsafe 代码

一个函数包含了 `unsafe` 代码不代表我们需要将整个函数都定义为 `unsafe fn`。事实上，在标准库中有大量的安全函数，它们内部都包含了 `unsafe` 代码块

### FFI

> 建议使用第三方库来完成交互代码

`FFI`（Foreign Function Interface）可以用来与其它语言进行交互，但是并不是所有语言都这么称呼，例如 Java 称之为 `JNI（Java Native Interface）`。

`FFI` 之所以存在是由于现实中很多代码库都是由不同语言编写的，如果我们需要使用某个库，但是它是由其它语言编写的，那么往往只有两个选择：

- 对该库进行重写或者移植
- 使用 `FFI`

下面的例子演示了如何调用 C 标准库中的 `abs` 函数：

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

C 语言的代码定义在了 `extern` 代码块中，而 `extern` 必须使用 `unsafe` 才能进行进行调用，原因在于其它语言的代码并不会强制执行 Rust 的规则，因此 Rust 无法对这些代码进行检查，最终还是要靠开发者自己来保证代码的正确性和程序的安全性。

#### ABI

在 `extern "C"` 代码块中，我们列出了想要调用的外部函数的签名。其中 `"C"` 定义了外部函数所使用的**应用二进制接口**`ABI` (Application Binary Interface)：`ABI` 定义了如何在汇编层面来调用该函数。在所有 `ABI` 中，C 语言的是最常见的。

#### 在其它语言中调用 Rust 函数

我们可以使用 `extern` 来创建一个接口，其它语言可以通过该接口来调用相关的 Rust 函数。但是此处的语法与之前有所不同，之前用的是语句块，而这里是在函数定义时加上 `extern` 关键字，当然，别忘了指定相应的 `ABI`：

```rust
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
```

上面的代码可以让 `call_from_c` 函数被 `C` 语言的代码调用，当然，前提是将其编译成一个共享库，然后链接到 C 语言中。

这里还有一个比较奇怪的注解 `#[no_mangle]`，它用于告诉 Rust 编译器：不要乱改函数的名称。 `Mangling` 的定义是：当 Rust 因为编译需要去修改函数的名称，例如为了让名称包含更多的信息，这样其它的编译部分就能从该名称获取相应的信息，这种修改会导致函数名变得相当不可读。

因此，为了让 Rust 函数能顺利被其它语言调用，我们必须要禁止掉该功能。

### 访问或修改一个可变的静态变量

可见全局变量部分。

### 实现 unsafe 特征

`unsafe` 的特征确实不多见。之所以会有 `unsafe` 的特征，是因为该特征至少有一个方法包含有编译器无法验证的内容。

### 访问 union 中的字段

访问 `union` 的字段是不安全的，因为 Rust 无法保证当前存储在 `union` 实例中的数据类型。

`union` 的使用方式跟结构体确实很相似，但是前者的所有字段都共享同一个存储空间，意味着往 `union` 的某个字段写入值，会导致其它字段的值会被覆盖。

关于 `union` 的更多信息，可以在[这里查看](https://doc.rust-lang.org/reference/items/unions.html)。

