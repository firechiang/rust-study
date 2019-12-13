fn main() {
    println!("Hello, world!");
}

struct Context<'a>(&'a str);

struct Parse<'a> {
    context: &'a Context<'a>
}

impl<'a> Parse<'a> {
    fn parse (&self) -> Result<(),&'a str>{
        return Err(&self.context.0[1..]);
    }
}