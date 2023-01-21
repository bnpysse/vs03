// Gentle Intro Rust 第二部分 2023年1月21日0时29分6秒
// 结构 枚举 和匹配
fn main() {
    // Rust 与其他语言有所不同,其它语言的变量总是会 引用 {references}
    // s2 成为对引用到 s1 的字符串对象的又一个引用. 在 C++ 中, s1 是一种 值(value) ,它会复制
    // 到 s2, 但是Rust会移动该值,它没有看到 字符串{strings} 是具有可复制性的
    // String是已经分配了包含"Hello dolly"的内存,而木复制这内容,将涉及到分配更多的内存
    //        String
    //      | addr |--------------------> Call me Ishmeal.... 
    //      | size |                        |
    //      | cap  |                        |
    //                                      |
    //        &str                          |
    //      | addr |------------------------|
    //      | size |
    //
    //       f64
    //      |8 bytes|
    // 第二个值是一个字符串切片 (&str) ,它与第一个字符串指向相同的内存,再加个大小, --它仅仅是(地址)名字--
    // 第三值是一个 f64 ,只有8个字符,不涉及任何其他内存
    // 复制{copy} 值只能通过它们在内存中的表示来定义,而当 Rust 拷贝时,只是在其他地方复制这些字节.
    // 类似的,一个没有 复制{copy} 的 值{value} 也 只是移动了{move}

    // 总而言之, 非复制{non-Copy} 的分配工作,会将 值 从一个位置 移动{move} 到另一个位置,
    // =====Rust喜欢move它,move它===== 
    println!("\n=====Rust喜欢move它,move它=====");
    let s1 = "hello dolly";
    let s2 = &s1;
    println!("s1 is {}, s2 is {}", s1, s2);
      
    // =====变量的范围===== 
    println!("\n=====变量的范围=====");
    // 经验法则是更愿意保留对原始数据的 引用{&} ,以此来 借用{borrow} 它
    // 但一个引用必须 不能 长剑过 拥有人{owner} !!
    // 首先, Rust是一个 块范围{block-scoped} 语言,变量仅在其代码夫持续时间内存在
    let a = 10;
    let b = "hello";
    {
        let c = "hello".to_string();
        // a, b, c都有
    }
    // 出了 block-scope, c 沒有了,但是 a,b 还是有的
    // 循环的变量只在循环代码块中可见,创建一个使用相同名称的新变量并不是一个错误,但它可能会造成混淆
    for i in 0..a {
        let b = &b[1..];
        // 原来的 b 不再可见
    }
    // b 没有了, i 也没有了
    // 当一个变量'超出范围',那么它会被  扔掉了{dropped},任何使用的内存都会被回收,而该变量的其他
    // 资源{resource} 将返回给系统. 不用的资源在不需要时立即回收.
    // 另一个Rust的特色问题是:变量看起来可能在其范围内,但其值已经是被  {move} 了.
    // 这里有个例子,其引用到 tmp 值,而引用 也 只在其区块 {} 内存在.
    let s1 = "hello dolly".to_string();
    let mut rs1 = &s1;
    {
        let tmp = "hello world".to_string();
        rs1 = &tmp;
        println!("the ref of rs1 is :{}", rs1);
    }
    // 先借用 {borrow} 了 s1 的值,然后再 借用tmp 的值,但 tmp 在{}之外被扔掉了
    // 在区块中, rs1 指向 &tmp, 但在区块结束后, tmp 整个都被 扔掉了{dropped}, 这个时候,
    // rs1 就变成了一个指向陈旧(或者说已被扔掉的)数据的引用.

    // =====元组=====
    println!("\n=====元组=====");
    // 有时候从函数返回多个值,元组就是一个简洁的解决方案.
    let t = add_mul(2.0, 10.0);
    println!("The result is : {:?}", t);
    println!("The result is : add {} mul {}", t.0, t.1);
    let (add,mul) = t;
    println!("The result is :  add {} mul {}", add, mul);

    // 元组能包括 不同 类型,这也是它与数组的主要区别
    let tuple = ("hello", 5, 'c');
    assert_eq!(tuple.0, "hello");
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, 'c');

    // 迭代器{Iterator}, enumberate 就像同名的 Python 生成器 {generator} 一样
    for t in ["zero", "one", "two", "three", "four", "five"].iter().enumerate() {
        println!("{}--{}", t.0, t.1);
    }
    // zip 会将两个迭代器,组合成一个 包含来自两者的元组 的迭代器
    let names = ["ten", "hundred", "thousand", ];
    let numbers = [10, 100, 1000];
    for p in names.iter().zip(numbers.iter()) {
        println!("{}--{}", p.0, p.1);
    }
    
    // =====结构{Structs}=====
    println!("\n=====结构=====");
    // 元组很方便,但是要追踪每个部分的含义,用 t.1 这种写法不够直接明了
    #[derive(Debug)]
    struct Person {
        first_name: String,
        last_name: String,
    }
    let p = Person {
        first_name: "John".to_string(),
        last_name: "Smith".to_string(),
    };
    println!("The Person is : {} {}",p.first_name,p.last_name);

    // 上面的初始化有些笨拙,所以想要把构造一个 Person ,融入其自身的函数
    // 这样即是通过 Impl 块来实现,这初始函数可以做成 Person 的一个关联函数

    impl Person {
        fn new(first: &str, last: &str) -> Person {
            Person{
                first_name: first.to_string(),
                last_name: last.to_string(),
            }
        }
        // 明确使用 &self,并作为 引用 传递,可以把 &self 想像成 self: &Person 的简写
        fn full_name(&self) -> String {
            format!("{},{}", self.first_name, self.last_name)
        }
        // 关键字 Self(注意是首字母大写) ,指的是 结构类型,你可以在脑海中用 Person 替换掉 Self
        // 注意这里是表达式,是不能带 分号; 的
        fn copy(&self) -> Self {
            Self::new(&self.first_name, &self.last_name)
        }
        // 方法允许修改数据,用到 可变的自我{mutable self} 参数:
        fn set_first_name(&mut self, name: &str) {
            self.first_name = name.to_string();
        }
        // 当使用简单的 self 参数时,数据会 移动{move}
        // 注意,当 v.to_tuple() 被 调用之后 ,v 已经移动并且不再可用
        fn to_tuple(self) -> (String, String) {
            (self.first_name, self.last_name)
        }
    }
    let p = Person::new("John", "Smith");
    println!("Though new fun is :{},{}", p.first_name, p.last_name);

    // 自我引用{reference self}
    println!("Fullname is : {}", p.full_name()); 
    // 总结:
    // 没有 self 相关参数,可以将函数与结构关联, 如 new "构造函数"
    // &self 参数:可以使用结构体的值,但不能改变它们
    // &mut self 参数:可以修改这些值 
    // self 参数: 将消耗值,因为它移动了
    // 如果对 Person 执行调试打印,需要添加 #[derive(Debug)] 在 Person 前面,实现输出
    println!("{:?}", p);

    // =====生命周期{Lifetimes}=====
    println!("\n=====生命周期Lifetimes=====");
    // 通常,结构体包含值,但通常它们还需要包含 引用{&}
    #[derive(Debug)]
    struct A {
        s: &'static str
    }
    // 如果不知道一个'引用'的生命周期,是不允许你存储它的
    // 所有  引用{&} 都是从某个值那里 借用{borrowed} 的,而且所有的 值  都是有 生命周期{lifetimes}的
    // 引用 的生命周期不能长于该值的生命周期

    let a = A { s: "hello dammit"};
    println!("{:?}", a);

    // =====特点{Traits}====
    println!("\n=====特点{Traits}====");
    // Traits 的中文含义有好几种,但本质是: 定义结构的一系列行为/方法
    // Rust 不能 继承 来自其他的结构, 它们都是独特的类型, 没有 sub-typing{子类型}
    // 一个类型之间的关系又应该是怎样做呢?

}

fn add_mul(x: f64, y: f64) -> (f64, f64) {
    (x + y, x * y)
}