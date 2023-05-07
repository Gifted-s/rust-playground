struct GoFmt {}

impl GoFmt {
    fn println(&self, msg: String) {
        println!("{}", msg)
    }
}
fn main() {
    let fmt = GoFmt {};
     // Just flexing please
    let msg = String::from( "This is complete madness, Daniel :)))");
    fmt.println(msg);
}

