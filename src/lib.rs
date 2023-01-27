mod break_up_mod;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("加餐");
        }
    }
}
// use crate::front_of_house::hosting; 相当于js的import xxx from xxx
// 前面加了pub之后呢，相当于js的export xxx from xxx  外部能访问lib.rs的hosting
pub use crate::front_of_house::hosting;

// 这样写太罗嗦了
// use std::io;
// use std::io::Write;
// 这样写省略了多余的 use
use std::io::{self, Write};
// 通配符 *  把rand所有的方法都引入进来
use rand::*;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist()
}

// trait 特征
// 特征好像其他语言的抽象类 可以提供具体实现也可以不提供具体实现
// 加上 pub 使私有变成公有，外部使用use 关键字能够联想到lib.rs被公开的特征或者方法函数
pub trait Summary {
    fn summarize(&self) -> String;
    // 默认实现  也可以自己实现
    fn summarize_default(&self) -> String {
        format!("Summary 特征的默认实现, {}", self.summarize())
    }
}
pub struct NewArticle {
    // 然后这些字段也需要添加 pub 不然外部会报错：该字段是私有字段
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
// 为 NewArticle 实现特征
impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}
// 为 Tweet 实现特征
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// 特征约束： 可以使用特征作为函数的参数类型
// 只有实现了 Summary 这个特征的参数才能传进来  String 或者 &str 都不行，NewArticle 或者 Tweet 就可以
// pub fn notify(item: &impl Summary) {
//     println!("breaking news! {}", item.summarize())
// }
// 上面的代码也可以写成这样，更加简洁  T 类型需要一个实现了 Summary 特征的类型
// pub fn notify<T: Summary>(item: &T) {
//   println!("breaking news! {}", item.summarize())
// }
// pub fn notify<T: Summary>(item1: &T, item2: &T) {
//   println!("breaking news! {}", item1.summarize())
// }

// 多重约束
// 我们想让入参 即实现了 Summary 特征也实现了 Display 特征  怎么约束呢？
use std::fmt::Debug;
use std::fmt::Display;
// 只有实现了 Summary 和 Display 特征的item才能作为入参传进去，不然报错
// pub fn notify(item: &(impl Summary + Display)) {
//     println!("breaking news! {}", item.summarize())
// }
// 也可以这样写
pub fn notify<T: Summary + Display>(item: &T) {
    println!("breaking news! {}", item.summarize())
}

// where 语法可以让我们 一些复杂的特征约束变得可读
pub fn some_function<T, U>(t: T, u: U)
where
    T: Summary + Display,
    U: Clone + Debug,
{
}

// 想让指定的实现了某个特征的类型拥有一些方法怎么办呢?
struct Pair<T> {
    x: T,
    y: T,
}
// 这里的Self指的是 Pair<T>
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
// 实现一个方法，只有实现了 Display 和 PartialOrd 特征的 T 类型才能使用这个方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("最大的数是: x {}", self.x)
        } else {
            println!("最大的数是: y {}", self.y)
        }
    }
}
