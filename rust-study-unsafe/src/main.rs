// 声明全局可变的变量（直接放到内存里面）
static mut COUNTER: u32 = 0;

// 直接修改内存
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

/**
 * unsafe关键字直接操作内存简单使用
 * 调用 C 代码简单使用：https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#using-extern-functions-to-call-external-code
 */
fn main() {
    // 指向一个内存的地址
    let address = 0x012345usize;
    // 将其转换成指向一个静态变量
    let r = address as *const i32;
    println!("address={},r={:?}",address,r);
    let mut num = 5;
    // r1 指向一个静态变量的指针
    let r1 = &num as *const i32;
    // r2 指向一个可变的指针
    let r2 = &mut num as *mut i32;
    // 如果不使用 unsafe 关键字包起来，程序无法编译通过
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    // 注意：调用 unsafe 关键字修饰的函数，必须使用unsafe关键字包起来
    unsafe {
        test_unsafe();
    }

    add_to_count(3);

    unsafe {
        println!("直接修改内存后的结果: {}", COUNTER);
    }

    let address1 = 0x01234usize;
    let r1 = address1 as *mut i32;
    // 直接获取内存里面的数据（这个地址在内存里面可能没有数据，就会报错）
    let slice1: &[i32] = unsafe {
        slice::from_raw_parts_mut(r1, 10000)
    };
    //println!("slice={:?}",slice1);
}

use std::slice;

/**
 * 函数没有使用 unsafe 关键字修饰，那么调用这个函数也不需要使用 unsafe 关键字修饰
 */
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

unsafe fn test_unsafe () {
    println!("调用了内存不安全的函数");
}

// 对象直接分配到内存
unsafe trait Foo {
    // methods go here
}
// 对象直接分配到内存
unsafe impl Foo for i32 {
    // method implementations go here
}
