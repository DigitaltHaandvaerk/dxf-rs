#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct Handle(pub u64);

// Make a const array of reserved handles values u64 these are reserved for the
// table handles. There are 9 tables in the spec and the first is reserved for
// APPID 1.
pub const RESERVED_HANDLES: [u64; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

impl Handle {
    pub fn empty() -> Self {
        Handle(0)
    }
    pub fn next_handle_value(self) -> Self {
        // Increment handle value
        let next_value = self.0 + 1;

        // If the next value is in the reserved range, skip the reserved handles
        if next_value <= RESERVED_HANDLES.len() as u64 {
            Handle(11)
        } else {
            Handle(next_value)
        }
    }
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub fn as_string(self) -> String {
        format!("{:X}", self.0)
    }
}
