fn main() {
  struct Fatorial<'s> { f: &'s dyn Fn (&Fatorial, u32) -> u32 }

  let fact = Fatorial {
    f: &|fact, valor| if valor == 0 {1} else { valor * (fact.f)(fact, valor - 1) }
  };

  println!("{}", (fact.f)(&fact, 5));
}