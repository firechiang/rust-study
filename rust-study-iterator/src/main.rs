/**
 * 循环迭代器和自定义迭代器以及Stream流处理还有改变集合元素的引用简单使用
 */
fn main() {
    let vec = vec![32,54,68];
    // 获取常规的迭代器
    let mut iter = vec.iter();
    // 获取下一个元素（注意：next()函数会将元素，从迭代器里面取出来）
    //println!("iter.next_1={:?}",iter.next());
    //println!("iter.next_2={:?}",iter.next());
    //println!("iter.next_3={:?}",iter.next());
    // 迭代器里面所有元素求和
    //let total: i32 = iter.sum();
    // 迭代器里面元素的个数
    let count = iter.count();
    //println!("求和结果：total={}",total);
    println!("元素的个数count={}",count);
    // Stream流的方式使用迭代器(map里面处理每个元素加1)（注意：这个不能改变引用地址）
    let vec2: Vec<_> = vec.iter().map(|x| x + 1).collect();
    println!("每个元素加1后，vec2={:?}",vec2);
    // into_iter()函数返回的迭代器，可以抽离元素
    let vec3: Vec<_> = vec.into_iter().filter(|x| x > &32).collect();

    let mut vec5 = vec![32,54,68];
    // iter_mut()函数是迭代可变引用的数组
    let vec6: Vec<_> = vec5.iter_mut().map(|&mut x| x + 1).collect();
    println!("vec6={:?}",vec6);

    println!("只要大于32的所有元素，，vec3={:?}",vec3);
    test_for_array();

    // 自定义的迭代
    let mut counter = Counter::new();
    println!("第一个位置={:?}",counter.next());
    println!("第二个位置={:?}",counter.next());
    println!("第三个位置={:?}",counter.next());
    println!("第四个位置={:?}",counter.next());
    println!("第五个位置={:?}",counter.next());
    println!("第六个位置={:?}",counter.next());

    // 自定义的结构体，实现迭代器
    // zip() 函数是将调用者迭代器和zip的参数迭代器合并（注意：这个合并是以最小个数的元素为基准，多出来的元素直接忽略掉），还有一个chain()函数也是组合迭代器，可以自行百度一下
    // skip()函数是跳过第1个元素
    // 说明：
    // zip的元素是                                   1 2 3 4 5
    // zip参数的元素是（因为跳过了第一个元素，所以少了一个） 2 3 4 5
    // 最后就是 1,2调用map；2,3调用map；3,4调用map；4,5调用map；
    // map有两个参数就是因为zip函数（注意：如果没有zip函数，map是只有一个参数的）
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(a, b)| {
                                     println!("a={},b={}",a,b);
                                     a * b
                                 })
                                 .filter(|x| x % 3 == 0)
                                 .sum();
    println!("sum={}",sum);
}

/**
 * for循环数组简单使用（有打印下标）
 */
fn test_for_array() {
    let array = ["1","2","sdsa","sdfsd"];
    for (i,item) in array.iter().enumerate() {
        println!("循环数组元素下标{}={}",i,item);
    }
}

#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
/**
 * 自定义的类实现迭代器
 */
impl Iterator for Counter {
    // 定义Item的类型是u32
    type Item = u32;
    // self表示调用方法的对象，相当于this
    // Self表示调用者的类型，相当于this的类型
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        // 只要count小于6就会有下一个元素，所以只要迭代Counter对象就会返回元素1,2,3,4,5
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
          // 过滤
         .filter(|s| s.size == shoe_size)
          // 转换成集合
         .collect()
}

//#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);
    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );
}
