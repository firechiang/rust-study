use std::fmt::Display;
/**
 * 类与类属性标识了相同的生命周期，那么类扩展impl也需要标识生命周期（就是写类函数的地方也要标识生命周期）
 */
fn main() {
    let context = Context("asdasda");
    println!("context={:?}",context);
    let parse = Parse{context: &context};
    println!("parse={:?}",parse);
    let value = parse.parse();
    println!("value={:?}",value);
    longs("qwq","asdas","我事泛型的值");

}

#[derive(Debug)]
struct Context<'a>(&'a str);

#[derive(Debug)]
struct Parse<'a> {
    context: &'a Context<'a>
}
//类与类属性标识了相同的生命周期，那么类扩展impl也需要标识生命周期（就是写类函数的地方也要标识生命周期）
impl<'a> Parse<'a> {
    // 注意：() 表示OK的泛型，&'a str 表示Err的泛型
    fn parse (&self) -> Result<(),&'a str>{
        return Err(&self.context.0[1..]);
    }
}
/**
 * 函数有标识生命周期，还有泛型（Display类型好像只要是基础类型都可以传）
 * 注意：泛型只能泛型接口类型
 */
fn longs<'a,T> (x: &'a str,y: &'a str,t: T) -> &'a str where T: Display {
    println!("t={}",t);
    if x.len() > y.len() {
        x
    }else {
        y
    }
}