// fn main() {
//    let mut _a = 923423;
//    _a = 3;
//    println!("a = {}", _a);
// }
// fn main() {
//     let (a, mut b): (bool, bool) = (true, false);
//     println!("a = {:?}, b = {:?}", a, b);
//     b = true;
//     println!("b = {}", b);
//     assert_eq!(a, b, "是不是相等的");
// }

// 常量
// const MAX_POINTS: i32 = 100_000;
// // 解构式赋值
// struct Structure {
//     e: i32,
// }
// fn main() {
//     let (a, b, c, d, e);
//     (a, b) = (1, 2);
//     [c, .., d, _] = [1, 2, 3, 4, 5];
//     Structure { e, .. } = Structure { e: 5 };
//     println!("a = {}, b = {}, c = {}, d = {}, e = {}", a, b, c, d, e);
//     let x = 5;
//     let x = x + 1;
//     {
//         let x = x * 2;
//         println!("inner scope x = {}", x);
//     }
//     println!("outer scope x = {}", x);
// }

// fn main(){
//     let a: u8 = 255;
//     let b = a.wrapping_add(1);
//     println!("a = {}, b = {}", a, b);
// }

// 猜数游戏
// use std::io;
// fn main() {
//     println!("猜数");
//     println!("请输入你的猜测");
//     let mut guess = String::new();
//     io::stdin().read_line(&mut guess).expect("无法读取");
//     println!("你猜的数是 {}", guess);
// }

// fn main(){
//     let x = '中';
//     println!("字符'中'占用了{}个字节的内存大小", std::mem::size_of_val(&x));
//     let bool = false;
//     if !bool {
//         println!("boolean占用了{}个字节的内存大小", std::mem::size_of_val(&bool));
//     }
//     let a = 5;
//     println!("数字占{}字节", std::mem::size_of_val(&a));
// }

// 语句和表达式
// fn main(){
//    add_with_extra(32, 32);
// }
// fn add_with_extra(x:i32, y:i32) -> i32{
//     let x = x + 1;
//     let y = y + 5;
//     // String是复杂类型 不能自动拷贝  它存储在堆中，它的堆指针存储在栈中
//     let mut s = String::from("sadfadf");
//     s.push_str("asdfasd");
//     println!("{}", s);
//     return x + y;
// }

// 所有权
// fn main() {
//     let s = String::from("hello");
//     let s1 = s;
//     println!("{}, world", s);  // 报错，s已经无效了
// }
// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1.clone();
//     println!("{}, {}", s1, s2);   // 使用深拷贝解决问题，但是需要小心使用 clone 影响性能
// }
// fn main(){
//     let x = "helloworld";
//     let y = x;
//     println!("x = {}, y = {}", x, y);  // y str只是拷贝了引用，没涉及到所有权的东西
// }

// 那么哪些是可以copy的呢？
// fn main() {
//     // 实际上基本类型及其组合都能copy
//     // 布尔
//     let bool1 = false;
//     let bool2 = bool1;
//     println!("bool1 = {}, bool2 = {}", bool1, bool2);

//     let int1 = 23;
//     let int2 = int1;
//     println!("int1 = {}, int2 = {}", int1, int2);

//     let f1 = 23.4;
//     let f2 = f1;
//     println!("f1 = {}, f2 = {}", f1, f2);

//     let char1 = '中';
//     let char2 = char1;
//     println!("char1 = {}, char2 = {}", char1, char2);

//     // 元组也可以，但是其组合必须是也可以copy的才行，像 bool 和 String 的组合是不行的。
//     let (a,b) = (false, 23);
//     let (c,d) = (a,b);
//     println!("a = {}, b = {}, c = {}, d = {}", a, b, c, d);
// }

// 引用和解引用

// 引用的地址在哪，前面不加 * 就是 p 输出内存地址
fn main() {
   let x = 5;
   // 填写空白处
   let p = &x;

   println!("x 的内存地址是 {:p}", p); // output: 0x2e86b9f7dc
}

// fn main() {
//     let x = 5;
//     let y = &x;   // 引用   引用了x的值
//     assert_eq!(5, x);
//     assert_eq!(5, *y);  // 解引用 意思是解出引用的值出来  需要加*   不然直接写y会报错，直接写y的意思是使用了整型5和引用类型y作比较，当然会报错啦
// }

// 想改变引用的值怎么办？
// fn main() {
//     let mut s = String::from("hello");
//     change(&mut s);
//     println!("s = {}", s);
// }
// fn change(some_string: &mut String) {
//     some_string.push_str("world");
// }

// 可变引用和不可变引用不能同时存在
// fn main() {
//     let mut s = String::from("hello");
//     let r1 = &s;
//     let r2 = &s;
//     // let r3 = &mut s;   // 报错，在同一作用域下同时存在了可变引用和不可变引用
//     // println!("{}, {} and {}", r1, r2, r3);// 报错，在同一作用域下同时存在了可变引用和不可变引用
//     // 但是先输出 r1 和 r2 确实没问题
//     // 这里 给出的解释是：引用的作用域和变量的作用域是不一样的，
//     // 变量的作用域是从创建到某个花括号结束
//     // 引用的作用域则是从创建到最后一次使用它的地方结束
//     println!("{}, {}", r1, r2);
//     // 所以这里的r1和r2已经在这里结束了，可以正常使用r3了
//     let r3 = &mut s;
//     println!("{}", r3);
// }

// 【错误的】悬垂指针
// fn main() {
//     let s = dangle();
//     println!("s = {}", s);
// }
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s   // 报错，因为s到这一步已经被释放掉了，但是却返回了它的引用，此时指针所指的值已经被释放掉了，这个引用指向了一个无效的String
// }

// 解决悬垂指针
// fn main(){
//     let s = not_dangle();
//     println!("s = {}", s);
// }
// fn not_dangle() -> String{
//     let s = String::from("hello");
//     s
// }
