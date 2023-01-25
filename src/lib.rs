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
