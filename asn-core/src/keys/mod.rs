use std::ops::BitOr;

#[repr(u8)]
pub enum JoystickKeys {
    KeyUp = 0b0000_0001,
    KeyDown = 0b0000_0010,
    KeyLeft = 0b0000_0100,
    KeyRight = 0b0000_1000,
    KeyFire = 0b0001_0000,
}

pub type JoystickKeysSet = u8;

pub trait KeysSetOperations {
    fn is_set(&self, flags: u8) -> bool;
    fn set(&mut self, flags: u8);
    fn reset(&mut self, flags: u8);
}

impl KeysSetOperations for JoystickKeysSet {
    fn is_set(&self, flags: u8) -> bool {
        flags & *self as u8 != 0
    }
    fn set(&mut self, flags: u8) {
        let result = *self | flags as u8;
        *self = result;
    }
    fn reset(&mut self, flags: u8) {
        let r_flags = flags ^ u8::MAX;
        let result = *self & r_flags;
        *self = result;
    }
}

#[cfg(test)]
mod tests {
    use crate::keys::{JoystickKeys, JoystickKeysSet, KeysSetOperations};

    #[test]
    fn it_working() {
        let mut key_set: JoystickKeysSet = JoystickKeysSet::default();
        assert_eq!(key_set as u8, 0);
        key_set.set(JoystickKeys::KeyFire as u8);
        assert_eq!(key_set, JoystickKeys::KeyFire as u8);
        key_set.reset(JoystickKeys::KeyFire as u8);
        assert_eq!(key_set as u8, 0);
        key_set.set(JoystickKeys::KeyRight as u8);
        assert!(key_set.is_set(JoystickKeys::KeyRight as u8));

        let mut key_set: JoystickKeysSet = JoystickKeysSet::default();
        key_set.set(JoystickKeys::KeyDown as u8);
        key_set.reset(JoystickKeys::KeyDown as u8);
        assert_eq!(key_set as u8, 0);
    }
}
