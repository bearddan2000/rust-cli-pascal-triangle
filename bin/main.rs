fn pascal_triangle(n: u64)
{
    if n > 10 {
        println!("[ERROR] {} is too big", n);
        return;
    }

    println!("[OUTPUT] ");
  for i in 0..n {
    let mut c:u64 = 1;
    for k in 0..i+1 {
      print!("{} ", c);
      c = c * (i-k)/(k+1);
    }
    println!();
  }
}

// Driver Program to test above function
fn main() {
    let input:u64 = 10;
    println!("[INPUT] {}", input);
    pascal_triangle(input);
}
