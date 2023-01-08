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
fn main(){
    for c in "中国人".chars() {
        println!("{}", c);  // 输出每个字符
    }
    for b in "中国人".bytes(){  
        println!("{}", b);  // 输出底层的字节数组
    }
    // 使用utf8_slice库
    let sliced = utf8_slice::slice("中国人", 1, 2);
    println!("截取到的字符 {}",sliced);  // 国
}
