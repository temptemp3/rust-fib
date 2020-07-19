
fn fib_recursive(n: u64) -> u64 {
    if n==0 { 0 }
    else if n==1 { 1 }
    else if n==2 { 1 }
    else { fib_recursive(n-1)+fib_recursive(n-2) }
}

fn fib_iterative_vector(n: u128) -> u128 {
    if n == 1 { 1 }
    else if n == 2 { 1 }
    else {
      let mut v: Vec<u128> = vec![1, 1];
      let mut r = n - 2;
      while r > 0 {
        v.push(v[v.len()-2]+v[v.len()-1]);
        r -= 1
      }
      v[v.len()-1]
    }
}

fn main() {
    let mut i = 1;
    loop {
        println!("fib{} = {}",i, fib_recursive(i));
        i += 1;
    }
    println!("Hello, world!");
}
