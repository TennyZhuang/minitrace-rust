use minitrace::trace;

#[trace("test-span")]
fn f(a: u32) -> u32 {
    a
}

fn main() {
    f(1);
}
