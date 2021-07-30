use super::Opcode;

#[test]
fn constant() {
  assert_eq!(
    Opcode::make(Opcode::Constant, 65534),
    vec![Opcode::Constant as u8, 0, 0, 0, 0, 0, 0, 255, 254]
  )
}
