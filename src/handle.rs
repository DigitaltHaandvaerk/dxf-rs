#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct Handle(pub u64);

// We have reserved the u64::MAX - 100 handles for internal use

impl Handle {
    pub fn empty() -> Self {
        Handle(0)
    }
    pub fn next_handle_value(self) -> Self {
        if self.0 >= u64::MAX - 100 {
            panic!("Handle overflow");
        }

        // Increment handle value
        let mut next = self.0 + 1;

        // Skip reserved values (e.g., 1 and values below 3)
        if next < 3 {
            next = 3;
        }

        Handle(next)
    }
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }
    pub fn as_string(self) -> String {
        format!("{:X}", self.0)
    }
}
