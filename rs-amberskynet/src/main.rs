use pollster::FutureExt;
use rs_amberskynet::run;

fn main() {
    println!("Let's go...");
    let my_fut = async {
        run().await;
    };
    my_fut.block_on();
    println!("Have a nice day...")
}
