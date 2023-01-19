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
// fn main() {
//    let x = 5;
//    // 填写空白处
//    let p = &x;

//    println!("x 的内存地址是 {:p}", p); // output: 0x2e86b9f7dc
// }

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

// 字符串
// 切片  左闭右开区间  左边界采用，右边界不采用
// fn main() {
//     let s = String::from("hello world");
//     let len = s.len();
//     println!("s的长度是{}", len);
//     let full = &s[..len];
//     println!("截取完整字符串 {}", full);
//     println!("s 占用{}字节", std::mem::size_of_val(&s));
//     // let slice = &s[0..2];  // he
//     let slice = &s[..2]; // he
//     println!("s 切片: {}", slice);
// }

// 数组切片
// fn main() {
//     let a = [1, 2, 3, 4, 5, 6];
//     let slice = &a[1..3];
//     println!("数组切片后：{:?}", slice);
// }

// String 和 str 类型的转换
//  &str => String
// fn main(){
//     let s = String::from("hello");
//     let s1 = "hello".to_string();
// }

// String => &str
// fn main() {
//     // 取引用就可以了
//     let s = String::from("hello");
//     say_hello(&s);
//     say_hello(&s[..]);
//     say_hello(s.as_str());
// }
// fn say_hello(s: &str) {
//     println!("{}", s);
// }

// fn main() {
//     let hello = String::from("中国人");  // 常见的汉字，一个汉字占三个字节，你想截取到“中”，只能按照0 ~ 3切片
//     let slice = &hello[0..3];
//     println!("{}", slice);
// }

// 字符串操作
// push
// fn main(){
//     let mut s = String::from("hello, ");
//     s.push('r');
//     println!("追加，push() => {}", s);
//     s.push_str("ust");
//     println!("追加rust, push_str => {}", s);
// }

// insert
// fn main(){
//     let mut s = String::from("hello rust");
//     s.insert(5, ',');
//     println!("插入字符，insert() -> {}", s);
//     s.insert_str(6, "i like");
//     println!("插入字符 i like insert_str() -> {}", s);
// }

// replace
// fn main() {
//     let s = String::from("I like rust. Learning rust is my favorite!");
//     let new_string_replace = s.replace("rust", "RUST");  // 返回一个新字符串
//     println!("替换之后, {}", new_string_replace);
//     let new_string_replacen = s.replacen("rust", "RUST", 1);  // 返回一个新字符串
//     println!("替换一次之后, {}", new_string_replacen);
//     let mut s1 = String::from("I like rust. Learning rust is my favorite!");
//     s1.replace_range(7..8, "R");  // 这个方法直接操作原字符串，所以需要加上mut让原字符串可变
//     println!("范围替换之后, {}", s1);
// }

// delete
// pop
// fn main() {
//     let mut s = String::from("hello 中文");
//     let p1 = s.pop();
//     let p2 = s.pop();
//     println!("截取的第一个, {:?}", p1);
//     println!("截取的第二个, {:?}", p2);
//     println!("结果是，{}", s);
// }

// remove
// fn main(){
//     let mut string_remove = String::from("测试remove方法");
//     println!("string_remove 占 {} 个字节", std::mem::size_of_val(string_remove.as_str()));
//     string_remove.remove(0);  // 删除第一个汉字
//     // string_remove.remove(1);  // 报错，因为是汉字，并没有删除完整个汉字的字节
//     string_remove.remove(3);
//     println!("删除之后的string_remove {}", string_remove);
// }

// truncate 直接操作原字符串，删除从指定位置开始到结尾的字符串
// fn main(){
//     let mut string_remove = String::from("测试remove方法");
//     string_remove.truncate(3);
//     println!("使用 truncate() 删除之后的字符串 -> {}", string_remove);
// }

// clear 删除字符串
// fn main(){
//     let mut string_remove = String::from("测试remove方法");
//     string_remove.clear();
//     println!("已经删除字符串，{}", string_remove);
// }

// 连接  使用 + 或者 += 相当于使用了add()  add的第二个参数是引用类型  &str 所以 + 右边也需要使用 &str
// fn main(){
//     let string_append = String::from("hello ");
//     let string_rust = String::from("rust");
//     let result = string_append + &string_rust;
//     let mut result = result + "!";
//     result += "???";
//     println!("结果为，{}", result);
// }

// fn main(){
//     let s1 = String::from("hello");
//     let s2 = String::from("rust");
//     let s3 = String::from("!!!");
//     let s4 = String::from("go");
//     // 之所以能这么写：s1 + "-" + &s2  返回String类型  之后的String类型再跟 + "-" + &s3，一直循环下去
//     let s = s1 + "-" + &s2 + "-" + &s3 + "-" + &s4;
//     println!("拼接结果为：{}", s);
// }

// format! 拼接字符串，这种方法让我想起了javascripxt es6的字符串拼接  `${hello} world`
// fn main(){
//     let s1 = "hello";
//     let s2 = String::from("rust");
//     let s = format!("{}, {}!", s1, s2);
//     println!("使用format拼接出来的结果是：{}", s);
// }

// 操作UTF-8字符串
// fn main(){
//     for c in "中国人".chars() {
//         println!("{}", c);  // 输出每个字符
//     }
//     for b in "中国人".bytes(){
//         println!("{}", b);  // 输出底层的字节数组
//     }
//     // 借助使用utf8_slice库
//     let sliced = utf8_slice::slice("中国人", 1, 2);
//     println!("截取到的字符 {}",sliced);  // 国
// }

// 元组  相当于ts中的元组，就是一个复杂组合类型
// fn main() {
//     // 这就是简单的元组，ts用[]中括号，rust用小括号
//     let tuple = (600, 4.1, 1);
//     // 解构赋值  ts也有解构赋值
//     let (x, y, z) = tuple;
//     println!("{:?}", tuple);
//     println!("tuple第一个元素 {}", tuple.0); // ts使用arr[0]来访问第一个元素，而rust使用tuple.0来访问第一个元素
//     println!("{}, {}, {}", x, y, z);
// }

// 元组与函数
// fn main(){
//     let s = String::from("hello");
//     let (s1, len) = calculate_length(s);
//     println!("{}, length -> {}", s1, len);
// }
// fn calculate_length(s: String) -> (String, usize){
//     let len = s.len();
//     return (s, len);
// }

// 结构体struct
// #[derive(Debug)]
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
// fn create_user(username: String, email: String) -> User {
//     User {
//         username,
//         email,
//         active: true,
//         sign_in_count: 1,
//     }
// }
// fn main() {
//     let user = create_user(String::from("xuan"), String::from("17324565534@qq.com"));
//     let user1 = User {
//         email: String::from("12345678910@qq.com"),
//         ..user
//     };
//     // 报错，因为user的所有权被转移给了user1
//     // println!("user，{:?}", user);
//     println!("user1，{:#?}", user1);
// }

// 元组结构体
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);
// fn main(){
//     let black = Color(0,0,0);
//     let origin = Point(0,0,0);
// }

// 单元结构体
// struct AlwaysEqual;
// fn main(){
//     let subject = AlwaysEqual;

// }
// impl SomeTrait for AlwaysEqual {

// }

// 结构体的方法  struct 的方法
#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }
    fn can_hold(&self, other_rect: Rectangle) -> bool {
        self.width > other_rect.width && self.length > other_rect.length
    }
    // 这里定义一个关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, length: size }
    }
}
fn main() {
    let origin = Rectangle {
        width: 100,
        length: 100
    };
    let origin_area = origin.area();
    println!("area: {}", origin_area);
    let r1 = Rectangle {
        width: 10,
        length: 10
    };
    let r2 = Rectangle {
        width: 200,
        length: 200
    };
    println!("origin 能不能容纳 r1：{}", origin.can_hold(r1));
    println!("origin 能不能容纳 r2：{}", origin.can_hold(r2));
    // 通过struct 的 关联函数创建了一个正方形
    let s = Rectangle::square(300);
    println!("square: {:#?}", s);
}



// dbg!宏  能够让我们看到输出的东西就像js的console.log 能显示行数
// #[derive(Debug)]
// struct Rectangle{
//     width: u32,
//     height: u32
// }
// fn main(){
//     let scale = 2;
//     let rect1 = Rectangle{
//         width: dbg!(30 * scale),
//         height: 50
//     };
//     dbg!(&rect1);
// }

// 枚举
// #[derive(Debug)]
// enum PokerSuit {
//     Hearts,
//     Clubs,
//     Diamonds,
//     Spades
// }
// fn main(){
//     let heart = PokerSuit::Hearts;
//     let diamond = PokerSuit::Diamonds;
//     println!("打印Heart，{:?}, {:?}", heart, diamond);
// }

// 有时候枚举也需要指定值，只不过这个值可以变，但是类型不能变
// enum PokerCard {
//     Hearts(u8),
//     Clubs(u8),
//     Diamonds(char),
//     Spades(char),
// }
// fn main() {
//     let c1 = PokerCard::Hearts(5);
//     let c2 = PokerCard::Diamonds('A');
// }

// 枚举甚至能包含字符串 结构体 数值甚至是另一个枚举
// enum Message {
//     Quit,
//     Move {
//         x: i32,
//         y: i32
//     },
//     Write(String),
//     ChangeColor(i32, i32, i32)
// }
// fn main(){
//     let m1 = Message::Quit;
//     let m2 = Message::Move { x: 1, y: 2 };
//     let m3 = Message::ChangeColor(255, 255, 0);
// }

// js有null 那rust呢？  rust有some和none
// fn main(){
//     let some_number = Some(5);
//     let some_string = Some("a String");
//     let none: Option<i32> = None;
// }

// js中当一个变量可能为空时我们需要处理为空的情况
/*
    a ? a : b
    a ?? b
    if (a) {
        return a
    }else{
        return b
    }
*/
// 而在rust中只要类型时Option<T>就一定要处理可能为空的情况，不是Option<T>类型你可以放心的认为它是安全的
// 我们可以使用模式匹配 match guess.cmp(&secret_number) {} 相当于我们js的switch
// match的作用就是 类型是Some<T> 执行这个  类型为None  执行那个
// fn plus_one(x: Option<i32>) -> Option<i32>{
//     // plus_one 通过 match guess.cmp(&secret_number) {} 来处理不同 Option 的情况。
//     // 第一种写法
//     // match guess.cmp(&secret_number) {} x {
//     //     None => {
//     //         return None
//     //     },
//     //     Some(i) => {
//     //         return Some(i + 1)
//     //     }
//     // }
//     // 第两种写法
//     // match guess.cmp(&secret_number) {} x {
//     //     None => None,
//     //     Some(i) => Some(i + 1)
//     // }
//     return match x {
//         None => None,
//         Some(i) => Some(i + 1)
//     }
// }
// fn main(){
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);
//     println!("six and none, {:?}, {:?}", six, none);
// }

// 数组
// 数组有两种 长度固定的array 就像 ts的tuple
// 另一种是长度能伸缩的Vector 就像 ts的array
// 都是高级类型  考虑所有权问题
// fn main() {
//     // array  [类型; 数组长度]
//     let a = [1, 2, 3, 4, 5];
//     // 还能用简洁的语法初始化数组
//     let b = [3; 4];    // 等同于[3,3,3,3]

//     // 访问数组也和js的一样
//     let first = a[0];
//     let second = b[1];
// }

// use std::io;

// fn main() {
//     let a = [1, 2, 3, 4, 5, 6];
//     println!("请输入数组索引值");
//     let mut index = String::new();
//     // 读取用户输入的索引值
//     io::stdin().read_line(&mut index).expect("读行失败");
//     let index: usize = index.trim().parse().expect("输入的索引值不是数字");
//     let element = a[index];
//     println!("获取到的索引值和元素是, {}, {}", index, element)
// }

// fn main(){
//     // 报错
//     // let array = [String::from("rust is future");2];
//     // let array = [3;5]; 是通过不断地copy出来的，前面也提到基本类型可以copy，复杂类型不行
//     // 所以[String::from("rust is future"); 2] 会报错
//     // 正确做法：使用 core::array::from_fn
//     let array: [String; 8] = core::array::from_fn(|i| String::from("rust is future"));
//     println!("{:?}", array);
// }

// 数组切片
// fn main() {
//     let a = [1, 2, 3, 4, 5];
//     let slice = &a[1..3];
//     assert_eq!(slice, &[2,3])
// }

// 数组的实际应用
// fn main() {
//     let one = [1, 2, 3];
//     let two: [u8; 3] = [1, 2, 3];
//     let blank1 = [0; 3];
//     let blank2: [u8; 3] = [0; 3];
//     // 二维数组
//     let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];
//     for a in arrays {
//         print!("{:?}:", a);
//         for n in a {
//             print!("\t{} + 10 = {}", n, n + 10);
//         }
//         let mut sum = 0;
//         for i in 0..a.len() {
//             sum += a[i];
//         }
//         println!("\t({:?}) = {}", a, sum);
//     }
// }

// use std::{io, cmp::Ordering};

// use rand::Rng;

// // 猜数游戏
// fn main(){
//     println!("请输入一个数字与神秘数字比较，判断win或者fail");
//     let secret_number = rand::thread_rng().gen_range(0, 100);
//     println!("这个神秘数字, {}", secret_number);
//     let mut guess = String::new();
//     io::stdin().read_line(&mut guess).expect("无法读行");
//     println!("你猜测的数字为：{}", &guess);
//     let guess: u32 = guess.trim().parse().expect("转换为数字失败");
    
//     match guess.cmp(&secret_number) {
//         Ordering::Less => println!("you loss"),
//         Ordering::Greater => println!("you win"),
//         Ordering::Equal => println!("equal")
//     } 

// }



// for 循环
// fn main() {
//     let a = [4,3,2,1];
//     // 获取元素的索引  enumerate 返回 （索引，元素值）
//     for (i, v) in a.iter().enumerate() {
//         println!("当前元素索引值 {}，值 {}", i, v);
//     }
//     // 只是想循环10次又不想创建一个长度是10的数组怎么办呢？
//     // 在 Rust 中 _ 的含义是忽略该值或者类型的意思
//     // 0..10 是闭开区间   0..=10是闭区间   包含 0 和 10
//     for _ in 0..=10 {
//         println!("该信息将会打印10次");
//     }
// }

// while循环
// fn main(){
//     let mut a = 0;
//     while a <= 5 {
//         println!("while循环 {}", a);
//         a += 1;
//     }
//     println!("while循环结束");
// }

// loop循环
// fn main() {
//     let mut n: u128 = 0;
//     // 死循环  用break打断
//     loop {
//         // if n > 5 {
//         //     break;
//         // }
//         println!("{}", n);
//         n += 1000000000;
//     }
//     // println!("我出来了");
// }

// loop循环的特殊
// fn main() {
//     let mut counter = 0;
//     // loop本身也是个表达式，所以可以再等号右边书写
//     let result = loop {
//         counter += 1;
//         if counter == 10 {
//             // break 和 js的break不一样，break后面还能跟一个表达式 代表 return [表达式]的意思
//             break counter * 2;
//         }
//     };
//     println!("result is: {}", result);
// }

// use std::{io, cmp::Ordering};

// use rand::Rng;

// // 多次猜数游戏
// fn main() {
//     println!("请输入一个数字与神秘数字比较，判断win或者fail");
//     let secret_number = rand::thread_rng().gen_range(0, 100);
//     println!("这个神秘数字, {}", secret_number);
//     loop {
//         let mut guess = String::new();
//         io::stdin().read_line(&mut guess).expect("无法读行");
//         println!("你猜测的数字为：{}", &guess);
//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => {
//                 println!("无效数字，请重新猜测......");
//                 continue
//             }
//         };
        
//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("猜测的数字太小了，请重新猜测......"),
//             Ordering::Greater => println!("猜测的数字太大了，请重新猜测......"),
//             Ordering::Equal => {
//                 println!("你赢了！");
//                 break;
//             }
//         } 
//     }
// }
