#![no_std]
#![no_main]

#[nexus_rt::profile]
fn fibonacci(n: u32) -> u32 {
    fib(n)
}

/// profile macro 用来分析guest中的 `fibonacci` 函数的执行情况
/// ```shell
/// benchmark/src/guest ±master⚡ » cargo nexus run
///  Execution Summary:
/// └── Total program cycles: 1073
/// └──  'src/guest/src/main.rs:fibonacci': 895 cycles (83% of total)
/// ```



fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

#[nexus_rt::main]
fn main() {
    let n = 5;
    assert_eq!(5, fibonacci(n));
}
