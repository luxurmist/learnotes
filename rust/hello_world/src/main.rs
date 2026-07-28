/*
 * file: main.rs
 * author: luxurmist
 * date: 2026-04-01
 */

// use 引用缩写
use std::{collections::HashMap, default, string};

use hello_world::mod_name;

// 声明函数 关键字 fn
// 在每个可执行的Rust程序中，main函数是最先运行的代码。

fn main() {
    println!("Hello, world!");

    // 声明变量 关键字 let
    // 变量命名 小写蛇形命名法 snake_case
    // 类型声明 变量名: 类型
    let zero: i32 = 0;
    // 变量默认 不可变 immutable
    // 可变修饰 关键字 mut
    let mut count: u32 = 0;
    // 变量具有 遮蔽 shadowed 特性，
    // 可以定义同名新变量，前一个被隐藏，可多次遮蔽，
    // 新变量作用域结束取消遮蔽。
    let mut count: u32 = 1;

    // 声明常量 关键字 const
    // 常量命名 大写蛇形命名法 SNAKE_CASE
    // 必须注明类型与值
    // 可以在任何作用域中声明
    // 只能被设置为常量表达式
    // 在声明它的作用域中,常量在整个声明周期都有效
    const MIN_IN_SEC: i32 = 60;

    // 静态类型语言
    // 每一个值都有一个特定的 数据类型 data type
    // 数据类型包括 标量 scalar 和 复合 complund

    // 标量 代表一个单独的值
    // 整型 i8 u8 i16 u16 i32(默认) u32 i64 u64 i128 u128 isize usize(size架构相关/指针宽度)
    let x: i32 = 12;
    // 浮点型 f32 f64(默认)
    let x: f32 = 3.0;
    // 布尔 bool 值为 true 和 false
    let x: bool = true;
    // 字符类型 char 单引号声明 ' ' 四个字节Unicode
    let x: char = 'z';
    // 空元组 单元类型 unit () 表示空值 值唯一
    let x: () = ();

    // 复合 多个值组合成一个类型
    // 元组 tuple 多个不同或相同类型的值，长度固定
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // 数组 array 每个元素类型必须相同，长度固定，在内存中连续存储
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // 控制流
    // if else if
    let n: i32 = 24;

    if n > 0 {
        // println!("正数");
    } else if n > 0 {
        // println!("负数");
    } else {
        // println!("零");
    }

    // loop
    // 无限循环
    let mut count: u32 = 0;
    loop {
        count += 1;
        if count == 4 {
            // 跳过本次循环
            continue;
        }
        if count == 10 {
            // 跳出循环
            break;
        }
    }
    // 嵌套标签
    'outer: loop {
        'inner: loop {
            //中断 outer 循环
            break 'outer;
        }
    }
    // 返回值
    let result: i32 = loop {
        break 12;
    };

    // while
    let mut count: i32 = 0;
    while count < 12 {
        count += 1;
    }

    // for in
    // 迭代器区间表示法 a..b ,包含a不包含b，步长1
    // a..=b，a和b都包含
    for n in 1..11 {
        // 1..=10 循环10次
        // println!({n});
    }
    // iter into_iter iner_mut
    let mut words = ["hello", "world"];
    for word in words.iter() {
        // 借用
    }
    for word in words.iter_mut() {
        // 可变借用
    }
    for word in words.into_iter() {
        // 移权
        // 消耗数据，循环后words不可用
    }

    // 栈和堆
    // 编译时数据类型大小固定，分配到栈
    // 编译时数据类型大小不固定，分配到堆
    // 指向可变类型的指针在栈上，类型数据在堆上

    // 作用域 {}

    // 所有权 ownership
    // Rust中的每一个值都有且只有一个所有者
    // 当所有者离开作用域，这个值将被丢弃(自动调用drop方法释放内存)
    // Rust永远不会自动创建数据的 解引用拷贝
    // 需要拷贝指针指向的数据使用 clone 方法
    // 函数传递参数、返回值都会发生所有权转移
    // 实现了 copy trait 的类型自动拷贝，不会发生移权
    // 具有 copy trait: 整型，浮点，布尔，字符，元组
    let mut words = ["hello", "world"];
    let mut words_e = words.clone();

    // 引用
    // 限制指针的读写权限
    // 同一作用域下，要么只能有一个可变引用，要么只能有多个不可变引用
    // 引用必须总是有效的
    let mut words = ["hello", "world"];
    let mut words_e = &words; //不可变引用
    let mut words_e = &mut words; //可变引用

    // 切片 slice
    // 部分引用
    let mut words = String::from("Hello,world!");
    let hello = &words[0..5];
    // [0..8] [0..=7]  前[..5] 后[4..] 空[] 全[..]

    // 结构体 struct

    // 单元结构体
    struct Unit;
    let _unit = Unit;
    // 元组结构体
    struct Pair(i32, f32);
    let pari = Pair(24, 6.7);

    struct User {
        name: String,
        age: u32,
    }

    let name = String::from("YeShang");
    let age = 24;

    let new_user = User { name, age };

    let new_user = User {
        name: String::from("YeShang"),
        age: 24,
    };

    let new_user_e = User {
        age: 32,
        // 剩余字段与new_user相同
        ..new_user
    };

    // 关联函数 与 方法
    // 所有在 impl 块中定义的函数被称为 关联函数
    // 第一个参数是 self 的称为方法
    impl User {
        // 函数
        fn new(user_name: String, user_age: u32) -> Self {
            Self {
                name: user_name,
                age: user_age,
            }
        }
        // 方法
        fn show(&self) {
            println!("{} {}", self.name, self.age);
        }
    }
    let new_user = User::new("YeShang".to_string(), 36);
    // new_user.show();

    // 枚举 enum
    // 可以是各种不同类型
    enum IpAddKind {
        V4(String),
        V6(String),
    }
    let new_ip = IpAddKind::V4(String::from("000.000.000.000"));

    // 模式匹配 match
    impl IpAddKind {
        fn show(&self) {
            match self {
                IpAddKind::V4(ip) => {
                    println!("V4:{}", ip)
                }
                IpAddKind::V6(ip) => {
                    println!("V6 {}", ip)
                }
                default => {
                    println!("?")
                }
            }
        }
    }
    // new_ip.show();

    // 标准库 枚举 Option
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    // 只匹配一个模式忽略其他模式 if let
    if let IpAddKind::V4(ip) = new_ip {
        // println!("IP V4")
    } else {
        // println!("IP V6")
    }

    // 标准库 集合 collections
    // 向量(vector) 字符串(string) 哈希映射（hash map）

    // Vector
    // 单个数据结构存放多个值，内存相连
    // 只能存储相同类型的值
    // 创建空的Vector
    let mut v: Vec<i32> = Vec::new();
    // 创建有初始值的Vector
    let mut v: Vec<i32> = vec![1, 2, 3];
    // 添加元素
    v.push(4);
    // 读取元素
    let one = &v[0];
    let one = v.get(0);
    // 遍历
    for i in &v {
        //不可变
    }
    for i in &mut v {
        //可变
    }

    // String
    // UTF-8 编码的字符集合
    // 新建空字符串
    let mut s = String::new();
    // 从字符串字面值创建
    let mut s = "hello world".to_string();
    let mut s = String::from("hello worl");
    // 更新字符串
    s.push('d');
    s.push_str("!");
    // 拼接字符串 + &
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    // // s1 移权至 s3
    // 拼接字符串 format!
    let s1 = String::from("hell");
    let s2 = String::from("o,wo");
    let s3 = String::from("rld!");
    let s = format!("{s1}-{s2}-{s3}");
    // 取slice
    let s4 = &s[0..5]; 
    // 遍历字符串
    for c in s.chars() {
        // 返回 char 类型，Unicode 标量值
    }
    for c in s.bytes() {
        // 返回原始字节
    }

    // HashMap
    // 用任何类型作为键来寻找数据
    // 一个HashMap中所有的键是同类型，值也都是相同类型
    // 新建HashMap
    let mut hm: HashMap<String, i32> = HashMap::new();
    // 插入键值对
    hm.insert(String::from("OK"), 1);
    // 键不存在才插入
    hm.entry(String::from("NO")).or_insert(2);
    // 访问值
    let key = String::from("OK");
    let value = hm.get(&key);
    // match value {
    //     Some(value) => println!("{value}"),
    //     None => println!("None")
    // }
    // 遍历
    for (key, value) in &hm {
        // println!("{key}:{value}");
    }

    // 错误处理
    // panic!

    // Result 枚举
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // unwrap()
    // expect()
    // 错误传播 ?
}

// 泛型 Generics
// 泛型函数
fn generice_fn<T>(x: T) -> T {
    x
}
// 泛型结构体
struct GenericeStruct<T, U, V> {
    x: T,
    y: U,
    z: V,
}
// 泛型枚举
enum GenericeEnum<T, U> {
    One(T),
    Two(U),
}
// 泛型方法
impl<T, U, V> GenericeStruct<T, U, V> {
    fn generice_method(&self) -> &T {
        &self.x
    }
}
// 限制类型的泛型方法
impl GenericeStruct<i32, i32, i32> {
    fn gm_i32_double(&self) -> i32 {
        *&self.x * 2
    }
}


// 特质 trait
// 不同类型都具有的行为
// 定义trait
trait Quantifiable{
    // 方法签名
    fn print_value(&self);
    // 默认实现
    fn print_ok() {
        println!("ok");
    }
}

trait SelfRecovery{

}

// 为类型实现trait
struct HealthPoint {
    value: u32,
}
struct MagicPoint {
    value: u32,
}

impl Quantifiable for HealthPoint {
    fn print_value(&self) {
        println!("{}", self.value);
    }
}

impl Quantifiable for MagicPoint {
    fn print_value(&self) {
        println!("{}", self.value);
    }
}

// 使用trait作为参数
fn hp_print(hp: impl Quantifiable) {
    hp.print_value();
}

// trait bound
fn mg_print<T: Quantifiable>(mp: T) {
    mp.print_value();
}

// 多个 trait bound
fn hp_print_recovery<T: Quantifiable + SelfRecovery>(hp: T) {
    hp.print_value();
}

// where 写法
fn hp_mp_print<T, U>(hp: T, mp: U)
where
    T: Quantifiable + SelfRecovery,
    U: Quantifiable + SelfRecovery,
{
    hp.print_value();
    mp.print_value();
}

// 返回值 使用 trait
fn create_hp() -> impl Quantifiable {
    HealthPoint{value: 0}
}

// 在泛型中为实现特定trait的类型实现特定方法
struct HP<T>{
    v: T,
}
impl<T: Quantifiable> HP<T> {

}

// 生命周期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
// 返回值的生命周期与较短的一致
// 实现结构体方法时，生命周期必须总是在impl关键字之后声明并在结构体名称之后被使用



// 测试 tests
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {

        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
