fn main() {
  fn fatorial(valor: u32) -> u32 {
    if valor == 0 {1} else { valor * fatorial(valor - 1) }
  }

  println!("{}", fatorial(5));
}