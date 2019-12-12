
/**
 * 值匹配简单使用
 */
fn main() {
    let val = Some(3);
    match val {
        Some(4) => println!("等于4"),
        Some(val) => println!("val={}",val),
        None => ()
    }

    let x = 5;
    // 如果都能匹配得到，谁在前面就执行谁
    match x {
        1 ... 6 => println!("在1到6之间"),
        4 | 5 | 7 => println!("是4或是5或是7"),
        5 => println!("等于5"),
        // 默认
        _ => ()
    }

    let s = 'b';
    // 如果都能匹配得到，谁在前面就执行谁
    match s {
        'a' ... 'v' => println!("a到v之间"),
        'v' => println!("等于v"),
        _ => ()
    }
    let p = Print {
        x: 12,
        y: 20
    };
    // 如果都能匹配得到，谁在前面就执行谁
    match p {
        Print{x: 10,y: 10} => println!("x:10,y:10"),
        Print{x: 12,y} => println!("x等于12"),
        Print{x,y} => println!("都没有值")
    }
    // 赋值
    let Print {x,y} = p;
    println!("p={:?}",p);


    let ps = vec![Print{x:10,y:20},Print{x:12,y:45}];
    let total: u32 = ps.iter().map(|&Print{x,y}|  x * x + y * y).sum();
    println!("total={}",total);

    // 复杂tup
    let ((xx,yy),Print{x,y}) = ((2000,10),Print{x:45,y:56});
    println!("xx={}",xx);

    fnn(5);

    let tup = (1,2,4,6,6);

    // _ 线代表可忽略的或者是默认的
    match tup {
        (1,_,_,_,6) => println!("第一个是1，最后一个是6"),
        // .. 表示全部用 _,_
        (2,..) => println!("第一个是2"),
        // .. 表示全部用 _,_，最后一个用last表示_
        (3,..,last) => println!("第一个是3"),
        // 默认
        _ => ()
    }

    let ss = Some(String::from("sdsdsa"));
    match ss {
        // 注意：如果没有ref关键字，println!("最后使用的ss={:?}",ss)是不能使用的
        // 加上 ref 关键字就不会在这里释放内存，如果想修改值的话就在加个 mut 关键字即可（修改的话使用 *sss = "ssdf"）
        Some(ref sss) => println!("ss={}",sss),
        // 匹配再加if
        Some(ref sss) if sss == "dfsd" => println!("ss={}",sss),
        None => ()
    }
    // 注意：如果上面的匹配没有ref关键字，这段代码不能使用
    println!("最后使用的ss={:?}",ss);

    enum Message {
        Hello {id: u32}
    }
    let msg = Message::Hello{id:5};
    match msg {
        Message::Hello {id: id @ 1...5} => println!("id的值在1到5之间"),
        Message::Hello {id: 10...100} => println!("id的值在10到100之间"),
        Message::Hello {id} => println!("默认处理")
    }
}

#[derive(Debug)]
struct Print {
    x: u32,
    y: u32
}

fn fnn (_: u32) {

}
