// FROM HERE
// https://rust-lang.github.io/async-book/07_workarounds/04_recursion.html

// 2nd
// https://users.rust-lang.org/t/async-fn-recursion-rust-1-77/108779

use futures::future::{BoxFuture, FutureExt};

fn main() {
    println!("Hello, first_recursion ! ");
    recursive().await?;
}



// So this function:
#[allow(dead_code)]
fn recursive() -> BoxFuture<'static, ()> {
    async move {
        recursive().await;
        recursive().await;
    }.boxed()
}

// rustc --version --verbose

