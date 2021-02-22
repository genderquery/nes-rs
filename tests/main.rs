use hex;
use hex::ToHex;
use nes::console::Console;
use std::fmt;
use std::fmt::Debug;
use std::ops::Deref;

#[derive(PartialEq)]
struct DebugLowerHex<T>(T);

impl<T: AsRef<[u8]>> fmt::Debug for DebugLowerHex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.0))
    }
}

impl<T> Deref for DebugLowerHex<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

#[test]
fn implied() {
    let expected = hex::decode(concat!(
        "00DEB0610A30312D696D706C6965640A",
        "0A5061737365640A0000000000000000",
    ))
    .unwrap();
    let mut console = Console::from_file("test_roms/01-implied.nes").unwrap();
    console.reset();
    for _ in 1..100000 {
        console.step();
    }
    let result = console.read_range(0x6000..0x6000 + expected.len() as u16);
    assert_eq!(DebugLowerHex(expected), DebugLowerHex(result));
}
