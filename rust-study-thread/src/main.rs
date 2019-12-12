use std::thread;
use std::time::Duration;
/**
 * 线程相关
 */
fn main() {
    // 启动一个新的线程（spawn函数就是启动一个新的线程，它的参数是一个匿名函数）
    let t = thread::spawn(|| {
        // 当前线程等待两秒
        thread::sleep(Duration::from_secs(2));
        println!("新的线程执行完成");
    });
    // 先跑子线程再跑主线程
    t.join();
    println!("主线程正在执行");
    // 先跑主线程再跑子线程
    //t.join();

    let vec = vec![1,2,3];
    // 注意：如果没有move关键字，下面这段代码无法编译通过（因为线程外的变量要在线程里面使用，必须使用move关键字取得所有权）
    let t2 = thread::spawn(move || {
        println!("打印数组元素={:?}",vec);
    });
    send_msg1();
    send_msg2();
    mutex_test1();
    mutex_test2();
    t2.join();
}

use std::sync::{Mutex, Arc};

/**
 * 死锁和锁的简单使用
 */
fn mutex_test1() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        // 获取 6 这把锁（下面写获取到锁之后的逻辑）
        *num = 6;
        // 注意：把注释打开将会产生死锁（原因：这里是获取到锁之后的逻辑，就是说这里本生就是获取到6这把锁之后的逻辑，但是在这里又要获取6这把锁，所以互斥产生了死锁）
        /*let mut num = m.lock().unwrap();
        *num = 6;*/
    }
    println!("m = {:?}", m);
}

/**
 * 多线程之间的锁简单使用
 */
fn mutex_test2 () {
    // 注意 Arc 是原子指针
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            // 获取到锁（注意：如果写 *num = 1，就是说明所有线程都要获取 1 这把锁（最后线程将同步执行））
            *num += 1;
            //thread::sleep(Duration::from_secs(1));
            println!("i={},num={}",i,*num);

        });
        handles.push(handle);
    }

    for handle in handles {
        println!("handle={:?}",handle);
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}


use std::sync::mpsc;
/**
 * 线程间的消息传递（多消息）
 */
fn send_msg1 () {
    /**
     * tx : 消息发送对象
     * rx : 消息接收对象
     */
    let (tx, rx) = mpsc::channel();
    // 注意：如果没有move关键字，下面这段代码无法编译通过（因为线程外的变量要在线程里面使用，必须使用move关键字取得所有权）
    let t = thread::spawn(move || {
        let vals = vec![
            String::from("你是毛毛吗？"),
            String::from("你不是毛毛！"),
            String::from("我知道你是谁了！"),
            String::from("原来是你啊！"),
        ];

        for val in vals {
            // 发送消息
            tx.send(val).unwrap();
            // 线程暂停1秒
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("主线程接收到了消息: {}", received);
    }
}
/**
 * 线程间的消息传递（单消息）
 */
fn send_msg2() {
    /**
     * tx : 消息发送对象
     * rx : 消息接收对象
     */
    let (tx, rx) = mpsc::channel();
    // 注意：如果没有move关键字，下面这段代码无法编译通过（因为线程外的变量要在线程里面使用，必须使用move关键字取得所有权）
    thread::spawn(move || {
        let val = String::from("贱贱还好吗？");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("主线程接收到了消息: {}", received);
}

