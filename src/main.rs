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
    // 这里起伏有点儿搞不太懂了,一开始不加是出错的,加了可以编译通过,现在又必须去掉才能编译通过
    // 到底是用,还是不用?或许是与 Nightly 的版本有关系的.2023年1月21日22时25分52秒

    // #[derive(Debug)]
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
    println!("这个地方好像有点问题,一开始没加'derive',结果出错,然后加了,通过了一次,但现在又出错,去掉后反而没事儿了\n{:?}", p);

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
    println!("\n=====特点Traits====");
    // Traits 的中文含义有好几种,但本质是: 定义结构的一系列行为/方法
    // Rust 不能 继承 来自其他的结构, 它们都是独特的类型, 没有 sub-typing{子类型}
    // 一个类型之间的关系又应该是怎样做呢?
    // 我个人理解有点 多态 的意思 2023年1月21日19时11分28秒
    // Rust 经常提到 实现{implemepting} X 的特点{trait} 
    trait Show {
        fn show(&self) -> String;
    }
    impl Show for i32 {
        fn show(&self) -> String {
            format!("four-byte signed {}", self)
        }
    }
    impl Show for f32 {
        fn show(&self) -> String {
            format!("eight-byte signed {}", self)
        }
    }
    let answer = 42;
    let maybe_pi = 3.14;
    let s1 = answer.show();
    let s2 = maybe_pi.show();
    println!("show {}", s1);
    println!("show {}", s2);

    // 一个完整的 Person-Debug实现,我个人认为就是重新定义了 Person的输出样式
    use std::fmt;
    impl fmt::Debug for Person {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // write! 是一个宏,内部的 f 是实现了 Write 的东西
            write!(f, "{}", self.full_name())
        }
    }
    // 下面是一个实现遍历浮点范围的迭代器,原来的迭代器只能实现整形数
    // 迭代器的非正式定义:它是一个带有结构体,具有一个可能会返回 Some 或 None 的 next 方法
    // f64 的迭代器 trait ,是写入 Iterator<Item=f64> ,可以理解为: 迭代器的关联类型 Item 设置为 f64
    // ... 表达式语句指的是 Iterator 所提供的方法,只需要定义 Item 和 next ,那该表达式语句就可以使用
    
    struct FRange {
        val: f64,
        end: f64,
        incr: f64,
    }
    fn range(x1: f64, x2:f64, skip:f64) -> FRange {
        FRange { val: x1, end: x2, incr: skip }
    }
    impl Iterator for FRange {
        type Item = f64;
        fn next(&mut self) -> Option<Self::Item> {
            let res = self.val;
            if res >= self.end {
                None
            } else {
                self.val += self.incr;
                Some(res)
            }
        }
    }
    for x in range(0.0, 1.0, 0.1) {
        println!("{:.1}", x);
    }
    // 使用 Vec ,然后通过 map 方法来使用之
    let v: Vec<f64> = range(0.0, 1.0, 0.1)
        .map(|x| x.sin())
        .collect();
    for i in v {
        println!("{:.11}", i);
    }

    // =====泛型函数====
    println!("\n=====泛型函数====");
    // 我们需要一个函数,来抛出实现 Debug 的任何值.
    // 可以在其中传递一个 任何 值类型的引用, T 是一个类型参数,需要在函数名称后面声明
    fn dump<T>(value: &T)
    where T: std::fmt::Debug {
        println!("value is {:?}", value);
    }
    let n = 42;
    dump(&n);
    // Rust泛型函数需要 Traits boounds 类型,我们在这里说, "T是实现了Debug的任意类型"
    // Rust的泛型函数,一开始可能看起来有点难受,但是,显式,就是明确定义,就能确切地知道可以安全地提供哪种值
    // 这些函数是 单态{monomorphic} 调用的, 与 多态{polymorphic} 合作,函数的主体都会为每个 唯一类型
    // 分别编译的.通过多态函数,相同的机器代码可以与每种匹配类型一起工作,动态地 调度{dispatching} 正确的方法
    // Monomorphic 生成更快的代码,专用于特定类型,常是 内联{inlined} 起来,所以,
    // 当 sqr(x) 被看到,它会被有效地用 x*x 取代.
    // 缺点是大的泛型函数为每一种可能导致的类型,产生大量的代码,引起 代码膨胀

    // =====泛型函数====
    println!("\n=====泛型函数====");
    // 枚举{enums} 类型具有一些确定的值,比如,一个方向只有四个可能的值
    // 不能假设任何特定的顺序,这里没有默许的"起始"整数值
    #[derive(Debug)]
    enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    impl Direction {
        fn next(&self) -> Direction{
            use Direction::*;
            match *self {
                Up => Right,
                Right => Down,
                Down => Left,
                Left => Up,
            }
        }
    }
    let mut d = Direction::Left;
    for _ in 0..8 {
        println!("d {:?}", d);
        d = d.next();
    }
    // 结果就是,这个特定的任意的顺序中,各个方向一起循环,它实际上是一个非常简单的状态机器
    // 要想比较枚举值,比如 assert_eq!(start, Direction::Left); 
    // 在 enum Direction前面加上 #[derive(Debug,PartialEq)]
    // 要想让枚举值 有一个自然的顺序, 必须 enum 前面加上
    // #[derive(Debug,PartialEq,ParticalOrd)]
    #[derive(Debug)]
    enum Value {
        Number(f64),
        Str(String),
        Bool(bool),
    }
    // impl Value {
    //     fn to_str(self) -> Option<String> {
    //         match self {
    //             Value::Str(s) => Some(s),
    //             _ => None,
    //         }
    //     }
    // }
    use Value::*;
    let n = Number(2.3);
    let s = Str("hello".to_string());
    let b = Bool(true);
    println!("n: {:?} s: {:?} b: {:?}", n, s, b);
    // 同样,这个枚举只能包含这些值的 一个, 其大小将是 最大变体 的大小
    // 枚举的超级跑车,还是与 match 进行结合
    fn eat_and_dump(v: Value) {
        use Value::*;
        match v {
            Number(n) => println!("number is {}", n),
            Str(s) => println!("string is {}", s),
            Bool(b) => println!("boolean is {}", b),
        }
    }
    eat_and_dump(n);
    eat_and_dump(s);
    eat_and_dump(b);
    // 使用引用传递的话,会怎么样呢?好像是无法处理 String , Number和Bool是没有任何问题的
    // 好在rustc编译器自行修正了此问题
    fn dump_ref(v: &Value) {
        use Value::*;
        match *v {
            Number(n) => println!("number is {}", n),
            Str(ref s) => println!("string is {}", s),
            Bool(b) => println!("boolean is {}", b),
        }
    }
    // dump_ref(&s);
    // println!("枚举里面的s? {:?}",s.to_str());

    // =====闭包{Closures}====
    println!("\n=====闭包Closures====");
    // Rust的很多力量来源于 闭包,函数与闭包之间存在很大差异,具体 体现 在明确类型的需要
    // 闭包为一个 结构 和它的借用 来自其环境的值,所以会有一个 liftetime
    // 所有闭包都是独特的类型, 其有共同的 traits
    let m = 2.0;
    let c = 1.0;
    let lin = |x| m * x + c;
    println!("res {} {}", lin(1.0), lin(2.0));
    // 调用一个闭包就是一个 方法调用: 三种函数 trait 对应于三种方法:
    // Fn 结构传递为 &self 
    // FnMut 结构传递为 &mut self 
    // FnOnce 结构传递为 self
    // 所以,闭包可能会改变它的 来自上层 的引用
    fn mutate<F>(mut f:F)
    where F: FnMut() {
        f()
    }
    let mut s = "world";
    mutate(|| s = "hello");
    assert_eq!(s, "hello");

    // filter 是另一种有用的迭代器方法,它只 允许 通过匹配条件的值 
    let tuples = [(10, "ten"), (20, "twenty"), (30, "thirty"), (40, "forty")];
    let iter = tuples
        .iter()
        .filter(|t| t.0 > 20)
        .map(|t| t.1);
    for name in iter {
        println!("{}", name);
    }

    // =====三种迭代器====
    println!("\n=====三种迭代器====");
    // 假设有一个 String 值的 Vec,以下是明确的迭代器类型和 隐式{implicitly}, 以及迭代器返回的实际类型
    // for s in vec.iter() {...}                    // &String 
    // for s in vec.iter_mut() {...}                    // &mut String
    // for s in vec.into_iter() {...}                    // String 
    // 隐式
    // for s in &vec {...}                    // &String 
    // for s in &mut vec {...}                    // &mut String 
    // for s in vec {...}                    // String 
    // into_iter 消耗 Vec ,并提取它的字符串,所以之后的 Vec 不再可用,它已被移动
    // 所以,隐含的形式 for s in &vec 通常才是你想要的,就像 &T 在向函数传递参数时,是一个很好的默认值
    //
    // 理解这三种类型是如何工作很重要的,因为Rust严重依赖于类型推导,在闭包参数中,你不会经常看到明确的类型,这是一件好事
    // 因为如果所有这些类型都明确的话,它的 写法 会很嘈杂
    ////////////////////////////////
    // map 取得迭代器返回的任何值,并将其转换为其他值,但是 filter 需要的是一个该值的 引用
    // 在这种正在使用 iter 的情况下,迭代器 item 的类型是 &String,注意 filter 接收的是这种类型的引用 
    // for n in vec.iter().map(|x: &String| x.len()) {...}          // n是usize
    // for s in vec.iter().filter(|x: &&String) x.len() > 2) {...}  // s是&String
    // 对于以上的表达式,在调用 x.len() 时,Rust会自动解引用,问题不明显,但是 |x:&&String|x == "one" 将无法工作
    // 因为操作符号对 类型匹配 会更加严格, rustc 会抱怨 &&String 和 &str 没有这样进行比较的,所以
    // 需要明确的 解引用, 要让 &&String 变成能 完成 比较的 &String. 看下面的例程
    // for s in vec.iter().filter(|x: &&String| *x == "one") {...}
    // 其等价的隐式表达写法:
    // for s in vec.iter().filter(|x| *x == "one") {...}
    // 如果省略显式类型,则可以修改参数,使 s 的类型就是现在的 &String :
    // for s in vec.iter().filter(|&x| x == "one") {...}
    // 这些语法规则真的是太麻烦了......2023年1月22日11时31分12秒


    // =====具有动态数据的结构====
    println!("\n=====具有动态数据的结构====");
    // 一个强大的技术是 一个包含对自身引用的结构.类似于 链表或者是二叉树
    type NodeBox = Option<Box<Node>>;
    #[derive(Debug)]
    struct Node {
        payload : String,
        left: NodeBox,
        right: NodeBox,
    }
    impl Node {
        fn new(s: &str) -> Node {
            Node{payload: s.to_string(), left: None, right: None}
        }
        fn boxer(node: Node) -> NodeBox {
            Some(Box::new(node))
        }
        fn set_left(&mut self, node: Node) {
            self.left = Self::boxer(node);
        }
        fn set_right(&mut self, node: Node) {
            self.right = Self::boxer(node);
        }
        // 现在为这棵树制定一个用法,按字符串的顺序插入节点的方法.
        // 将新数据与当前节点进行比较,如果较少,则尝试插入左侧,否则尝试插入右侧,左侧可能没有节点,那么就 set_left 
        fn insert(&mut self, data: &str) {
            if data < &self.payload {
                // 提供一个可变的引用给到 box ,如果 Option 是 Some 的话,并应用 insert 方法
                // 否则,需要为左侧创建一个新的 Node 
                match self.left {
                    Some(ref mut n) => n.insert(data),
                    None => self.set_left(Self::new(data)),
                }
            } else {
                match self.right {
                    Some(ref mut n) => n.insert(data),
                    None => self.set_right(Self::new(data))
                }
            }
        }
        // 按顺序遍历,访问左侧,在节点上做点什么,然后访问右侧
        fn visit(&self) {
            if let Some(ref left) = self.left {
                left.visit();
            }
            println!("'{}'", self.payload);
            if let Some(ref right) = self.right {
                right.visit();
            }
        }
    }
    // 下面测试一下
    let mut root = Node::new("root");
    root.set_left(Node::new("left"));
    root.set_right(Node::new("right"));
    // # 表示扩开,表现相当完美 2023年1月22日12时59分21秒
    println!("arr {:#?}", root);
    let mut root = Node::new("root");
    root.insert("one");
    root.insert("two");
    root.insert("three");
    root.insert("four");
    println!("root {:#?}", root);
    // 遍历,左中右,这算是先序遍历
    println!("\n先序遍历");
    root.visit();

    // =====泛型结构====
    println!("\n=====泛型结构====");
    // 为了所有可能的 payload 类型,重写的泛型 Node 与它的类型参数 T
    type GenNodeBox<T> = Option<Box<GenNode<T>>>;
    #[derive(Debug)]
    struct GenNode<T> {
        payload: T,
        left: GenNodeBox<T>,
        right: GenNodeBox<T>,
    }
    impl <T: PartialOrd> GenNode<T> {
        fn new(s: T) -> GenNode<T> {
            GenNode {payload: s, left: None, right: None}
        }
        fn boxer(node: GenNode<T>) -> GenNodeBox<T> {
            Some(Box::new(node))
        }
        fn set_left(&mut self, node: GenNode<T>) {
            self.left = Self::boxer(node);
        }
        fn set_right(&mut self, node: GenNode<T>) {
            self.right = Self::boxer(node);
        }
        fn insert(&mut self, data: T) {
            if data < self.payload {
                match self.left {
                    Some(ref mut n) => n.insert(data),
                    None => self.set_left(Self::new(data)),
                }
            } else {
                match self.right {
                    Some(ref mut n) => n.insert(data),
                    None => self.set_right(Self::new(data)),
                }
            }
        }
    }

    let mut root = GenNode::new("root".to_string());
    root.insert("one".to_string());
    root.insert("two".to_string());
    root.insert("three".to_string());
    root.insert("four".to_string());

    println!("root {:#?}", root);

    // 确实有些难懂,主要是在引用,作用范围,什么时候用 & &mut........
    // 2023年1月22日13时32分23秒
}

fn add_mul(x: f64, y: f64) -> (f64, f64) {
    (x + y, x * y)
}