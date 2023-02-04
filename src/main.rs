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
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     length: u32
// }
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.length
//     }
//     fn can_hold(&self, other_rect: Rectangle) -> bool {
//         self.width > other_rect.width && self.length > other_rect.length
//     }
//     // 这里定义一个关联函数
//     fn square(size: u32) -> Rectangle {
//         Rectangle { width: size, length: size }
//     }
// }
// fn main() {
//     let origin = Rectangle {
//         width: 100,
//         length: 100
//     };
//     let origin_area = origin.area();
//     println!("area: {}", origin_area);
//     let r1 = Rectangle {
//         width: 10,
//         length: 10
//     };
//     let r2 = Rectangle {
//         width: 200,
//         length: 200
//     };
//     println!("origin 能不能容纳 r1：{}", origin.can_hold(r1));
//     println!("origin 能不能容纳 r2：{}", origin.can_hold(r2));
//     // 通过struct 的 关联函数创建了一个正方形
//     let s = Rectangle::square(300);
//     println!("square: {:#?}", s);
// }

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

// 联想一下switch 里面的default 如果所有的模式都不匹配的话那怎么办呢？
// fn main(){
//     let a = 0u8;
//     match a {
//         1 => println!("1"),
//         3 => println!("2"),
//         5 => println!("5"),
//         _ => ()   // _下划线通配符 指的是 js里面的switch 的 default  所有模式都不匹配就走 _ 下划线
//     }
// }

// if let
// fn main() {
//     let a = 1;
//     // 想判断a的情况去做某种事情
//     // 用match去写太罗嗦了
//     // match a {
//     //     1 => println!("1"),
//     //     _ => ()
//     // }
//     // 用if let去写反而简洁一点
//     if let 1 = a {
//         println!("1");
//     }
// }

// matches! 宏  两个参数 返回bool  布尔值 true 和 false  判断两个参数是否匹配
// #[derive(Debug)]
// enum MyEnum {
//     Foo,
//     Bar
// }
// fn main() {
//     let mut v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
//     // let target_vec = v.iter().filter(|x| x == MyEnum::Foo);
//     let target_vec = v.iter().filter(|x| matches!(x, MyEnum::Foo));
//     // 这里的 filter 有点像 js 的 es6 里面的 filter((item, index) => { return item === MyEnum['Foo'] })
//     println!("{:?}", target_vec);
// }

// 匹配守卫   match guard  说白了就是模式匹配后面额外的if 判断  同一个模式下的不同条件
// fn main() {
//     let num = Some(4);
//     match num {
//         Some(x) if x < 5 => println!("x 小于 5"),   // 输出 x 小于 5
//         Some(x) => println!("{}", x),
//         None => ()
//     }
// }

// 匹配守卫和 | 或运算符的优先级    | 或优先级 > 匹配守卫优先级
// fn main() {
//     const X: i32 = 4;
//     const Y: bool = true;
//     match X {
//         // 这里的意思是：X 要在 4,5,6这个范围内，并且 Y 要为 true
//         4 | 5 | 6 if Y => println!("匹配成功"),
//         _ => println!("匹配失败")
//     }
// }

// 匹配中的 @ 绑定 作用就是能够限制某一个字段的匹配范围
// fn main() {
//     enum Message {
//         Hello {
//             id: i32
//         }
//     }
//     let msg = Message::Hello { id: 5 };
//     match msg {
//         // id被限制在 4 ~ 6之间并且它还绑定了一个变量给内部的作用域使用：id_variable
//         Message::Hello { id: id_variable @ 4..=6 } => println!("匹配到的id是{}", id_variable),
//         // id被限制在了 7~10 之间
//         Message::Hello { id: 7..=10 } => println!("可能会匹配到 7到10范围的id"),
//         // id 并没有限制任何范围，什么值都能匹配到
//         Message::Hello { id: _ } => println!("怎么都可以匹配到id")
//     }
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

// 数组的解构赋值
// fn main() {
//     let arr = &[111, 323];
//     if let [x, ..] = arr {
//         assert_eq!(x, &111);
//     }
//     if let &[.., y] = arr {
//         assert_eq!(y, 323);
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

// 泛型
// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }
// fn main() {
//     let number_list = vec![32,43,54,65,87,91,22];
//     let largest1 = largest(&number_list);
//     println!("最大的数字是{}", largest1);
// }

// fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
//     a + b
// }
// fn main() {
//     let a = 33;
//     let b = 22;
//     let result = add(a,b);
//     println!("相加的结果是{}", result);
// }

// 结构体也可以用泛型  例如：
// struct Point<T> {
//     x: T,
//     y: T
// }
// // 甚至可以结构体内部使用不同类型:
// struct Point1<T, U> {
//     x: T,
//     y: U
// }
// fn main() {
//     let point = Point {
//         x: 11,
//         y: 22
//     };
//     let point1 = Point1 {
//         x: 11,
//         y: 22.2
//     };
// }

// 方法中使用泛型
// struct Point<T> {
//     x: T,
//     y: T
// }
// // 注意这里 "Point<T>" 已经不再表示泛型，而是代表结构体 Point<T>  不代表 Point 类型
// // 写法上你不能写为  impl<T> Point {}
// // impl后面跟<T> 才是表示泛型
// // 语法 impl<泛型> 结构体 {}   这么写你应该懂了吧
// impl<T> Point<T> {
//     // 定义一个get方法
//     fn x(&self) -> &T {
//         &self.x
//     }
// }
// fn main() {
//     let point = Point {
//         x: 22,
//         y: 33
//     };
//     println!("获取point的x的值: {}", point.x());
// }

// 泛型方法
// #[derive(Debug)]
// struct Point<T, U> {
//     x: T,
//     y: U,
// }
// impl<T, U> Point<T, U> {
//     // self 没加 &  代表不是引用 它可以move到mixup方法里面去了 以后再次调用实例的话将会报错
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }
// fn main() {
//     let point = Point { x: 23, y: 32.5 };
//     let mixed = point.mixup(Point { x: 22, y: 44.3 });
//     println!("混合后{:?}", mixed)
// }

// 可以使用具体的泛型类型实现方法
// struct Point<T> {
//     x: T,
//     y: T
// }
// impl Point<u32> {
//     fn distance_from_origin(&self) -> u32 {
//         (self.x.pow(2) + self.y.pow(2))
//     }
// }
// fn main() {
//     let point: Point<u32> = Point {
//         x: 22,
//         y: 33
//     };
//     let result = point.distance_from_origin();
//     let point1: Point<i32> = Point {
//         x: 33,
//         y: 33
//     };
//     // 报错：因为point1 的类型不是u32 而是i32 不能调用distance_from_origin方法
//     // let result2 = point1.distance_from_origin();
// }

// 枚举中的泛型 prelude里面
// enum Option<T> {
//     Some(T),
//     None
// }
// 还有 Result
// enum Result<T, E> {
//     Ok(T),
//     Err(E)
// }

// 值的泛型 const  例如可以对数组的长度做泛型
// fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
//     println!("arr {:?}", arr);
// }

// fn main() {
//     let arr = [1,2,3];
//     display_array(arr);
//     let arr = [1,2];
//     display_array(arr);
// }

// Vec 动态数组
// fn main() {
//     // 创建一个vec  相当于我们js的 let arr = new Array()
//     let mut v = Vec::new();
//     // 改变vec数组 同样需要添加 mut
//     v.push(1);
//     // 也可以使用vec!宏来创建动态数组并且可以同时初始化
//     let a = vec![1, 2, 3, 4];
//     // 读取 数组中的元素有两种方法：索引和get()   不过get返回的是Option<T> 需要使用match解构
//     let mut three = &a[2];
//     three = &4;
//     println!("three: {} vec: {:?}", three, a);

//     let mut fours: &i32;

//     let four = a.get(3).expect("无法读取");
//     fours = four;

//     // match four {
//     //     Some(x) => {
//     //         fours = x;
//     //         println!("four: {}", x)
//     //     },
//     //     None => println!("None")
//     // }
//     println!("fours: {}", fours);
// }

// fn main() {
//     // 这段代码会报错：原因是 first引用的是旧数组，而v.push() 之后，旧数组需要拓展出6，就会重新开辟一段空间把旧数组拷贝过来形成一个新数组。
//     // 结果最后使用first之后，first还是指向的旧数组，一个不存在的引用，就会报错。
//     // 解决方法就是：把& 去掉 let first = &v[0] 换成 let first = v[0]
//     let mut v = vec![1, 2, 3, 4, 5];
//     let first = &v[0];
//     // TODO：可以把这段放开看看报错效果
//     // v.push(6);
//     println!("first: {}", first)
// }

// Vector可不可以存储不同类型的元素呢？答案是不行的。只能存储相同类型的元素
// 但是你可以借助 Vector + Enumerable 枚举进行存储 这样就相当于存储了相同类型的枚举的动态数组了
// #[derive(Debug)]
// enum IpAddr {
//     V4(String),
//     V6(String)
// }
// fn main() {
//     // 这样写就相当于vec 存储了两种类型的元素  但是这两种类型的元素又是同一种枚举
//     let v = vec![
//         IpAddr::V4("V4".to_string()),
//         IpAddr::V6("V6".to_string())
//     ];
//     for i in v {
//         println!("ip: {:?}", i);
//     }
// }

// 注意： world_hello 是我们项目里面 Cargo.toml 的name的属性值
// use world_hello::NewArticle;
// use world_hello::Summary;

// fn main() {
//     let new_article = NewArticle {
//         headline: String::from("页头"),
//         location: String::from("位置"),
//         author: String::from("作者"),
//         content: String::from("上下文")
//     };
//     println!("new_article {}", new_article.summarize());
//     println!("new_article {}", new_article.summarize_default())

// }

// use world_hello::notify;
// use world_hello::NewArticle;

// // 特征约束
// fn main() {
//     let new_article = NewArticle {
//         headline: String::from("页头"),
//         location: String::from("位置"),
//         author: String::from("作者"),
//         content: String::from("上下文")
//     };
//     notify(&new_article);
//     let str = "String";
//     // 会报错：因为str没有实现 Summary 特征
//     // notify(str);
// }

// use std::ops::Add;

// // 为自定义类型实现 加 操作
// #[derive(Debug)]
// struct Point<T: Add<T, Output = T>> {
//     x: T,
//     y: T
// }
// impl<T: Add<T, Output = T>> Add for Point<T> {
//     type Output = Point<T>;
//     fn add(self, p: Self) -> Self {
//         Point { x: self.x + p.x, y: self.y + p.y }
//     }
// }
// fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
//     a + b
// }
// fn main() {
//     let p1 = Point {
//         x: 1,
//         y: 2
//     };
//     let p2 = Point {
//         x: 3,
//         y: 4
//     };
//     let p3 = Point {
//         x: 1,
//         y: 2
//     };
//     let p4 = Point {
//         x: 3,
//         y: 4
//     };
//     println!("p1 + p2 = {:?}", add(p1, p2));
//     println!("p3 + p4 = {:?}", p3 + p4);
// }

// use std::fmt::{Display, self};

// #[derive(Debug)]
// enum FileState {
//     Open,
//     Close
// }
// impl Display for FileState {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Self::Close => write!(f, "Close"),
//             Self::Open => write!(f, "Open")
//         }
//     }
// }
// #[derive(Debug)]
// struct File {
//     name: String,
//     data: Vec<u8>,
//     state: FileState
// }
// impl Display for File {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "<{}, {}>", self.name, self.state)
//     }
// }
// impl File {
//     fn new(name: &str) -> File {
//         File {
//             name: String::from(name),
//             data: Vec::new(),
//             state: FileState::Close
//         }
//     }
// }
// fn main() {
//     let file = File::new("file.txt");
//     // 这里没有报错 因为 File 实现 Debug 特征
//     println!("打印文件 {:?}", file);
//     // 这里报错说：没有 File 实现Display特征
//     println!("打印文件 {}", file)
// }

// 特征对象
// 特征对象解决了什么问题？函数返回特征类型的时候 我们只能返回一种类型，而特征对象让我们可以返回多种类型
// 也许你会想到利用之前说的枚举类型，但这样太麻烦了你需要先定义好枚举具体有什么
// 像动态添加到arr里面有需要手动添加进枚举里面
// enum FileState {
//     Close,
//     Open
// }
// let arr: Vec<FileState> = vec![
//     FileState::Close,
//     FileState::Open
// ];

// &dyn xxx    和   Box<dyn xxx>  是特征对象，可以解决这个这个问题

// trait Draw {
//     fn draw(&self) -> String;
// }
// struct Button {
//     width: u32,
//     height: u32,
//     label: String
// }
// impl Draw for Button {
//     fn draw(&self) -> String {
//         format!("Button")
//     }
// }
// struct SelectBox {
//     width: u32,
//     height: u32,
//     options: Vec<String>
// }
// impl Draw for SelectBox {
//     fn draw(&self) -> String {
//         format!("SelectBox")
//     }
// }
// struct Screen {
//     components: Vec<Box<dyn Draw>>
// }
// impl Screen {
//     fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }
// fn main() {
//     let screen = Screen {
//         components: vec![
//             Box::new(Button {
//                 width: 100,
//                 height: 100,
//                 label: String::from("sss")
//             }),
//             Box::new(SelectBox {
//                 width: 200,
//                 height: 200,
//                 options: vec![
//                     String::from("Yes"),
//                     String::from("Maybe"),
//                     String::from("No")
//                 ]
//             })
//         ]
//     };
//     screen.run()
// }

// HashMap
// 创建 HashMap   key-value   就好像js的对象 一样  每个键对应一个值
// HashMap的key 只能使用实现了 Eq 和 Hash 特征的类型：bool    int   uint   String   &str  此外 f32 和 f64 由于浮点数精度的问题无法相等比较 所以没有实现Hash特征
// use std::collections::HashMap;

// fn main() {
//     // 创建HashMap用 HashMap::new()
//     let mut hash = HashMap::new();
//     hash.insert("红宝石", 1);
//     hash.insert("蓝宝石", 2);
//     hash.insert("假宝石", 3);

//     // 如何将Vec 转成 HashMap
//     let arr = vec![
//         ("中国队".to_string(), 100),
//         ("美国队".to_string(), 20),
//         ("日本队".to_string(), 40)
//     ];
//     let map: HashMap<_, _> = arr.into_iter().collect();
//     println!("{:?}", map);

//     // 查询
//     let mut scores = HashMap::new();
//     scores.insert(String::from("Blue"), 1);
//     scores.insert(String::from("Yellow"), 2);

//     let target_team = String::from("Blue");

//     let team_name = scores.get(&target_team);
//     println!("team name: {}", team_name.expect("team_name不存在"));

//     // 循环
//     for (key, value) in &scores {
//         println!("key: {}, value: {}", key, value);
//     }
//     println!("scores: {:?}", scores);

//     // 更新
//     let mut scores1 = HashMap::new();
//     scores1.insert("blue", 1);
//     scores1.insert("yellow", 2);
//     // 如果key存在，更新上去，然后返回旧值
//     let old = scores1.insert("blue", 4);
//     assert_eq!(old, Some(1));
//     let new = scores1.get("blue");
//     assert_eq!(new, Some(&4));
//     let a = scores1["blue"];
//     println!("字面量获取map的值 {}", a);
//     // or_insert 入参代表默认值，如果entry查找不到key 则插入默认值
//     let v = scores1.entry("green").or_insert(5);
//     assert_eq!(*v, 5);
//     // 查找到 green 存在并且值是 5
//     let v = scores1.entry("green").or_insert(50);
//     // 结果还是5
//     assert_eq!(*v, 5);

//     // 例子：查询词语出现的次数
//     let text = "hello world wonderful world";
//     let mut map = HashMap::new();
//     for word in text.split_whitespace() {
//         // 返回的是可变引用，所以下面可以直接修改 count 的次数
//         // let count = map.entry(word).or_insert(0);
//         // 还能使用函数返回新的值  or_insert_with 入参是一个函数  函数的返回值就是插入的默认值
//         let count = map.entry(word).or_insert_with(random_stat_buff);
//         *count += 1
//     }
//     println!("词语的次数: {:?}", map);
// }
// fn random_stat_buff() -> u8 {
//     1
// }

// 类型转换用 as
// 类型转换  只能范围小的转大的  因为会产生意想不到的结果  例如：
// fn error_transition() {
//     let error300 = 300_i32 as i8;
//     // i8 类型最大值就是127  300 > 127   属于大范围转小范围  数据可能会出错
//     // 超出范围的值就会从0开始继续计算
//     println!("转换后的300真的是300吗？：{}", error300); // 44
// }
// fn main() {
//     error_transition();
//     let i8_max = i8::MAX;
//     let i8_min = i8::MIN;
//     println!("i8 最大的值是： {}", i8_max); // 127
//     println!("i8 最小的值是： {}", i8_min); // -128
//     let u8_min = u8::MIN;
//     println!("u8 最小的值是： {}", u8_min); // 0

//     // 常用的转换
//     let a = 3.1 as i8; // 3
//     let b = 100_i8 as i32; // 100
//     let c = 'a' as u8; // 字符转成整数那就是  a 对应 97   asci 码

//     // 打印看看
//     println!("a: {}, b: {}, c: {}", a, b, c);

//     // 内存地址转换为指针
//     let mut values: [i32; 2] = [1, 2];
//     // Returns an unsafe mutable pointer  返回一个不安全的可变指针
//     let p1 = values.as_mut_ptr();
//     println!("p1: {:?}", p1);   // 0x6619affae0
//     let first_address = p1 as usize;
//     println!("first_address: {}", first_address);   // 438517627616
//     let second_address = first_address + 4;   // 4 == std::mem::size_of::<i32>()   i32 占用4字节
//     println!("second_address: {}", second_address);   // 438517627620
//     let p2 = second_address as *mut i32;
//     println!("p2: {:?}", p2);   // 0x6619affae4
//     unsafe {
//         *p2 += 1;
//     }
//     assert_eq!(values[1], 3);
// }

// rust 高级部分
// 生命周期  'a 就是生命周期标注的写法
// fn main() {
//     let string1 = String::from("asdfasdf");
//     let string2 = "asdfa";

//     let result = longest(&string1.as_str(), string2);
//     println!("result: {}", result)
// }
// 1. 每个引用都获得独自的生命周期
// 2. 有且只有一个入参只有一个生命周期，返回值将会是这个生命周期
// 3. 若存在多个输入生命周期，且其中一个是 &self 或 &mut self，则 &self 的生命周期被赋给所有的输出生命周期
// fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
//     if str1.len() > str2.len() {
//         str1
//     } else {
//         str2
//     }
// }

// fn error_longest<'a>(str1: &'a str, str2: &str) -> &'a str {
//     str1
// }

// 认识方法的生命周期
// struct ImportantExcerpt<'a> {
//     part: &'a str
// }
// impl<'a> ImportantExcerpt<'a> {
//     fn level(&self) -> i32 {
//         3
//     }
// }

// // 'a: 'b 指的是 a生命周期要和b生命周期活得一样久
// impl<'a: 'b, 'b> ImportantExcerpt<'a> {
//     fn announce_and_return_part(&'a self, announcement: &'static str) -> &'b str {
//         self.part
//     }
// }
// fn main(){}


// 泛型和特征约束一起使用
// use std::fmt::Display;
// fn longest_with_an_announcement<'a, T>(a: &'a str, b: &'a str, ann: T) -> &'a str where T: Display {
//     if a.len() > b.len() {
//         a
//     }else{
//         b
//     }
// }
// fn main() {}


// &'static 和 T: 'static

// use std::fmt::Display;

// fn main() {
//     let r1;
//     let r2;
//     {
//         static STATIC_EXAMPLE: i32 = 14;
//         r1 = &STATIC_EXAMPLE;
//         let x = "&'static str";
//         r2 = x;
//     }
//     println!("'static i32: {}", r1);
//     println!("'static str: {}", r2);

//     let r3;
//     {
//         let s1 = "String".to_string();
//         static_bound(&s1);
//         // 如果反注释下面这一行代码，就会报错，s1是String类型，没有'static的生命周期，所以离开了这段代码块就会drop掉
//         // 到时候r3将会变成悬垂指针：r3 引用的东西消失了
//         // r3 = &s1
//         r3 = s1
//     }
//     println!("r3: {}", r3);

// }

// fn static_bound<T: Display + 'static>(t: &T) {
//     println!("{}", t)
// }


// static 到底针对谁
// 是针对引用本身 还是 针对引用所指的数据?
// 答案是: 引用所指的数据
// fn main() {
//     {
//         let static_str = "我在只读内存里面";
//         println!("{}", static_str);
//         // 由于static_str 是 static  虽然引用不能再被使用了,但是数据还在 内存里面
//     }
//     // 下面的代码会报错: static_str已经离开所在的作用域了,引用会报错
//     // println!("static_str: {}", static_str);
// }



// 闭包
// fn main() {
//     let example_closure = |x| x;
//     // 当闭包没有标注类型的时候，第一次类型推导出什么类型，该闭包就是什么类型
//     // 下面的example_closure推导出了String类型
//     let s = example_closure(String::from("hello"));
//     // 这里就不能用u32 类型
//     // let n = example_closure(5);
// }

// 这是一个缓存存储器，下面是他的简单实现
// 现在它只支持u32类型，如果想要String 类型的呢 ？
// struct Cacher<T: Fn(u32) -> u32> {
//     query: T,
//     value: Option<u32>,
// }

// impl<T: Fn(u32) -> u32> Cacher<T> {
//     fn new(query: T) -> Cacher<T> {
//         Cacher { query, value: None }
//     }
//     fn value(&mut self, arg: u32) -> u32 {
//         match self.value {
//             Some(v) => v,
//             None => {
//                 let v = (self.query)(arg);
//                 self.value = Some(v);
//                 v
//             }
//         }
//     }
// }

// 下面这样写就实现了 多态 不仅限于u32 这个类型
// struct Cacher<T, U>
//     where 
//         T: Fn(U) -> U,
//         U: Copy
// {
//     query: T,
//     value: Option<U>,
// }
// impl<T, U> Cacher<T, U>
//     where 
//         T: Fn(U) -> U,
//         U: Copy
// {
//     fn new(query: T) -> Cacher<T, U> {
//         Cacher { query, value: None }
//     }
//     fn value(&mut self, arg: U) -> U {
//         match self.value {
//             Some(v) => v,
//             None => {
//                 let v = (self.query)(arg);
//                 self.value = Some(v);
//                 v
//             }
//         }
//     }
// }
// fn main() {}

// 闭包和函数很相似，但是有一些地方还是不同的
// 比如：闭包能在内部访问外部的变量，而函数不能在函数体内部访问外部的变量

// 闭包的三种类型约束：转移所有权(FnOnce)、不可变借用(Fn)、可变借用(FnMut)
// 1. 转移所有权
// fn func_once<F>(func: F)
// // FnOnce 顾名思义：只能被调用一次的闭包，但是可以通过某些手法解除这个限制  ==> Copy 特征
//     where F: FnOnce(usize) -> bool
// {
//     println!("{}", func(3));
//     // 报错：使用了两次func
//     // println!("{}", func(4));
// }

// fn func_once1<F>(func: F)
// // 只要同时实现了Copy这个特征，就不会转移所有权
//     where F: FnOnce(usize) -> bool + Copy
// {
//     println!("{}", func(3));
//     println!("{}", func(4));
// }

// fn main() {
//     let x = vec![1,2,3];
//     func_once(|z| z == x.len());
//     func_once1(|z| z == x.len())
// }


// 2. 可变借用
// fn main() {
//     let mut str = String::new();
//     let mut update_string = |x| str.push_str(x);
//     update_string("hello");
//     println!("{}", str);
// }

// 不加mut就是另一种写法了：
fn main() {
    let mut str = String::new();
    let update_string = |x| str.push_str(x);
    exec(update_string);
    println!("{}", str)
}
fn exec<'a, F: FnMut(&'a str)>(mut func: F) {  // 在入参约束上加mut 代表可变借用
    func("hello")
}
