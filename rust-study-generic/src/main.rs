/**
 * 泛型简单使用
 */
fn main() {
    let array = ["wqwq","asasd","fdsfsd"];
    let returnValue = iterator(&array);
    println!("迭代器返回的结果returnValue={}",returnValue);
    let array1 = [1,2,3,4];
    let returnValue1 = iteratorP(&array1);
    println!("迭代器返回的结果returnValue1={}",returnValue1);
    let array2 = [1,2,3,4];
    let returnValue2 = iteratorE(&array2);
    println!("迭代器返回的结果returnValue2={}",returnValue2);

    let array3 = [45,45,2,45,2,45,51,51];
    let returnValue3 = iteratorS(&array3);
    println!("迭代器返回的结果returnValue3={}",returnValue3);
}

/**
 * 因为Copy效率低，不推荐使用，所以这里直接使用引用去取值
 */
fn iteratorS<T: PartialOrd> (array: &[T]) -> &T {
    let i = 0;
    for item in array.iter() {
        if item < &array[i] {
            return &item;
        }
    }
    return &array[i];
}

/**
 * 泛型函数简单使用
 * 数组参数里面是什么类型就放回什么类型
 * 注意：Copy标识泛型的类型实现了Copy接口（注意：Copy效率低，不推荐使用（建议使用引用取值））
 */
fn iterator<T: Copy> (array: &[T]) -> T {
    // 注意：如果上面泛型里面没有加Copy标识，这段代码是编译不过的（因为泛型里面无法知道类型，要赋值的话，就一定要实现Copy）
    let mut largest = array[0];
    return largest;
}

/**
 * 注意：Copy标识泛型的类型实现了Copy接口。PartialOrd标识泛型的类型实现了PartialOrd接口
 * 注意：Copy效率低，不推荐使用（建议使用引用取值）
 */
fn iteratorP<T: PartialOrd+Copy>(array: &[T]) -> T {
    // 注意：如果上面泛型里面没有加Copy标识，这段代码是编译不过的（因为泛型里面无法知道类型，要赋值的话，就一定要实现Copy）
    let largest = array[0];
    for &item in array.iter() {
        // 注意：如果上面泛型里面没有加PartialOrd标识，这段代码是编译不过的（因为泛型里面无法知道类型，要比较的话，该类型就要实现PartialOrd）
        if (item > largest) {
            return item;
        }
    }
    return largest;
}

/**
 * 注意：Copy标识泛型的类型实现了Copy接口。PartialEq标识泛型的类型实现了PartialEq接口
 * 注意：Copy效率低，不推荐使用（建议使用引用取值）
 */
fn iteratorE<T: PartialEq+Copy>(array: &[T]) -> T {
    // 注意：如果上面泛型里面没有加Copy标识，这段代码是编译不过的（因为泛型里面无法知道类型，要赋值的话，就一定要实现Copy）
    let largest = array[0];
    for &item in array.iter() {
        // 注意：如果上面泛型里面没有加PartialOrd标识，这段代码是编译不过的（因为泛型里面无法知道类型，要看等不等的话，该类型就要实现PartialEq）
        if (item == largest) {
            return item;
        }
    }
    return largest;
}


/**
 * 泛型枚举简单使用
 */
enum Res<T,E> {
    Ok(T),
    Err(E)
}

/**
 * 类里面使用泛型
 */
struct User<T> {
    username: T,
    age: u32
}

/**
 * 多泛型简单使用
 */
struct Dept<T,U> {
    x: T,
    y: U
}
