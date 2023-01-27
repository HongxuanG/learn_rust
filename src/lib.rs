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
pub fn notify(item: &impl Summary) {
    println!("breaking news! {}", item.summarize())
}
