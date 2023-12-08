
# RUST

All about rust

```rust
struct GoFmt {}
impl GoFmt {
    fn Println(&self, msg: String) {
        println!("{}", msg)
    }
}
fn main() {
    let fmt = GoFmt {};
    let msg = String::from( "rustacean :)");
    fmt.Println(msg);
}

```
