fn filtra_multiplos_de_seis (quantity: i32) -> Vec<i32> {
  (1i32..)
    .filter(|&x| x % 2i32 == 0i32)
    .filter(|&x| x % 3i32 == 0i32)
    .take(quantity as usize)
    .collect::<Vec<i32>>()
}

fn main() {
  println!("Multiplos de seis -> {:?}", filtra_multiplos_de_seis(5));
}