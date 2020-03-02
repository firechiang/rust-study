use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

/**
 * 完整线程池的实现（包含线程池的关闭代码）
 *
 * 线程池实现原理：
 * 在线程池初始化的时候创建一个拥有n个线程的线程池，每个线程里面都有一个死循环在获取任务执行函数的锁，线程获取到了锁就跑任务执行函数。
 * 注意：每一个任务执行函数都是一把新的锁，每启动一个任务执行函数都会有一个空闲的线程，获取到并执行
 */
fn main() {
    // 初始化一个线程池
    let mut pool = ThreadPool::new(4);

    for i in 0..10 {
        pool.execute(|| {
            println!("打发顺丰水电费水电费是的是的发生的方式");
            thread::sleep(Duration::from_secs(2));
        });
    }
    thread::sleep(Duration::from_secs(4));
}

// 任务类型（它是Box的引用类型）
type Job = Box<dyn FnBox + Send + 'static>;

// 工作任务消息
enum Message {
    // 启动一个任务
    NewJob(Job),
    // 关闭线程
    Terminate,
}

// 工作函数接口
trait FnBox {

    fn call_box(self: Box<Self>);
}
// 实现工作函数接口
impl<F: FnOnce()> FnBox for F {

    fn call_box(self: Box<F>) {
        // 调用自身（注意：当前类本身就是一个函数的引用）
        (*self)()
    }
}

// 工作类
pub struct Worker {
    id: usize,
    // 工作线程（用来执行任务的线程）
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    /**
     * id       任务ID
     * receiver 是一个原子指针（可以简单理解是锁），多个线程同时获取同一个引用的话只能有一个线程能获取到原子指针里面的引用（锁）
     */
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        // 创建一个线程
        let thread = thread::spawn(move || {
            println!("线程 {} 启动了",id);
            // 死循环（一直在接收任务函数）
            loop {
                /**
                 * 获取锁并接收其它线程发送过来的任务函数
                 */
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        print!("任务 {} 开始执行：", id);
                        // 执行其它线程发送过来的函数
                        job.call_box();
                    },
                    Message::Terminate => {
                        println!("线程 {} 关闭了",id);
                        break;
                    },
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

// 线程池类
pub struct ThreadPool{
    // 任务集合
    workers: Vec<Worker>,
    // 线程间的通信工具（用于发送要执行的任务到其它线程）
    sender: mpsc::Sender<Message>,
}
impl ThreadPool {
    
    // 创建线程池
    fn new (size: usize) -> ThreadPool {
        /**
         * 线程间的通信管道
         * sender 用于发送信息
         * receiver 用于接收信息
         */
        let (sender, receiver) = mpsc::channel();
        /**
         * Arc类型可以让多个线程接收消息，而Mutex则确保只有一个线程能接收到消息
         */
        let receiver = Arc::new(Mutex::new(receiver));
        // 创建一个规定大小的数组对象用来装线程任务
        let mut workers = Vec::with_capacity(size);
        // 将线程池里面的线程任务填满
        for i in 0..size {
            workers.push(Worker::new(i,Arc::clone(&receiver)));
        }
        return ThreadPool {workers,sender};
    }

    // 执行任务
    fn execute<F> (&self,f: F) where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}
// 线程池关闭以及内存释放实现（注意：这个函数是Rust自动调用的，出作用域自动调用）
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers.");
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
