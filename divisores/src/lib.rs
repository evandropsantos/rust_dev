#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic]
  fn zero_is_not_valid() {
    divisores(-10);
  }

  #[test]
  fn dois_divide_dois() {
    assert_eq!(divisores(2), vec![2]);
  }

  #[test]
  fn tres_divide_tres() {
    assert_eq!(divisores(3), vec![3]);
  }

  #[test]
  fn divisores_de_quatro() {
    assert_eq!(divisores(4), vec![2, 4]);
  }

  #[test]
  fn divisores_de_vintequatro() {
    assert_eq!(divisores(24), vec![2, 3, 4, 6, 8, 12, 24])
  }

  #[allow(dead_code)]
  fn divisores(valor: i64) -> Vec<i64> {
    if valor <= 1 {
      panic!("{} <= 0 não é valido", valor);
    } else {
      (2..valor + 1)
        .filter(|x| valor % x == 0)
        .collect::<Vec<i64>>()
    }
  }
}
