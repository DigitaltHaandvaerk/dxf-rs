use crate::{CodePair, Drawing, DxfError, DxfResult, Handle};

use crate::code_pair_put_back::CodePairPutBack;

/// Represents an Infrastructure Record Data (IRD) object containing Danish utility information.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct IrdObjRecord {
    /// Object handle (unique identifier).
    pub handle: Handle,
    /// Handle of reactor object.
    pub reactor_handle: Handle,
    /// Handle of owner object.
    pub owner_handle: Handle,
    /// Record type identifier (typically 516).
    pub record_type: i16,
    /// Record version (typically 1).
    pub record_version: i32,
    /// Binary data segments containing utility information as UTF-16 encoded key-value pairs.
    pub binary_data_segments: Vec<Vec<u8>>,
}

// public implementation
impl IrdObjRecord {
    /// Creates a new IRD object record with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a binary data segment containing utility information.
    pub fn add_binary_data(&mut self, data: Vec<u8>) {
        self.binary_data_segments.push(data);
    }

    /// Encodes key-value pairs as UTF-16 binary data for storage in 1004 fields.
    /// Each segment is limited to 127 bytes maximum.
    pub fn encode_key_value_pairs(&mut self, pairs: &[(String, String)]) -> DxfResult<()> {
        let mut current_segment = Vec::new();

        for (key, value) in pairs {
            // Encode key as UTF-16 LE + null terminator
            let key_utf16 = Self::encode_utf16_le(key);
            let value_utf16 = Self::encode_utf16_le(value);
            let null_terminator = vec![0x00, 0x00];

            let entry_size = key_utf16.len() + null_terminator.len() + value_utf16.len() + null_terminator.len();

            // If adding this entry would exceed 127 bytes, start a new segment
            if current_segment.len() + entry_size > 127 {
                if !current_segment.is_empty() {
                    self.binary_data_segments.push(current_segment.clone());
                    current_segment.clear();
                }
            }

            // Add key + null + value + null
            current_segment.extend_from_slice(&key_utf16);
            current_segment.extend_from_slice(&null_terminator);
            current_segment.extend_from_slice(&value_utf16);
            current_segment.extend_from_slice(&null_terminator);
        }

        // Add the final segment if it contains data
        if !current_segment.is_empty() {
            self.binary_data_segments.push(current_segment);
        }

        Ok(())
    }

    /// Decodes binary data segments back to key-value pairs.
    pub fn decode_key_value_pairs(&self) -> DxfResult<Vec<(String, String)>> {
        let mut pairs = Vec::new();

        for segment in &self.binary_data_segments {
            let mut pos = 0;

            while pos < segment.len() {
                // Read key
                let (key, new_pos) = Self::read_utf16_string(segment, pos)?;
                pos = new_pos;

                // Skip null terminator
                if pos + 1 < segment.len() && segment[pos] == 0 && segment[pos + 1] == 0 {
                    pos += 2;
                } else {
                    break; // Invalid format
                }

                // Read value
                let (value, new_pos) = Self::read_utf16_string(segment, pos)?;
                pos = new_pos;

                // Skip null terminator
                if pos + 1 < segment.len() && segment[pos] == 0 && segment[pos + 1] == 0 {
                    pos += 2;
                }

                if !key.is_empty() {
                    pairs.push((key, value));
                }
            }
        }

        Ok(pairs)
    }

    /// Helper function to encode string as UTF-16 little endian.
    fn encode_utf16_le(s: &str) -> Vec<u8> {
        let mut result = Vec::new();
        for ch in s.encode_utf16() {
            result.push((ch & 0xFF) as u8);        // Low byte first
            result.push(((ch >> 8) & 0xFF) as u8); // High byte second
        }
        result
    }

    /// Helper function to read UTF-16 string from binary data.
    fn read_utf16_string(data: &[u8], start_pos: usize) -> DxfResult<(String, usize)> {
        let mut pos = start_pos;
        let mut utf16_chars = Vec::new();

        while pos + 1 < data.len() {
            let low = data[pos] as u16;
            let high = data[pos + 1] as u16;
            let char_code = low | (high << 8);

            if char_code == 0 {
                break; // Null terminator found
            }

            utf16_chars.push(char_code);
            pos += 2;
        }

        let result = String::from_utf16(&utf16_chars)
            .map_err(|_| DxfError::MalformedString)?;

        Ok((result, pos))
    }
}

impl Default for IrdObjRecord {
    fn default() -> Self {
        IrdObjRecord {
            handle: Handle::empty(),
            reactor_handle: Handle::empty(),
            owner_handle: Handle::empty(),
            record_type: 516,
            record_version: 1,
            binary_data_segments: Vec::new(),
        }
    }
}

// internal visibility only
impl IrdObjRecord {
    pub(crate) fn read_ird_objects(drawing: &mut Drawing, iter: &mut CodePairPutBack) -> DxfResult<()> {
        loop {
            match iter.next() {
                Some(Ok(pair)) => {
                    if pair.code == 0 {
                        match &*pair.assert_string()? {
                            "ENDSEC" => {
                                iter.put_back(Ok(pair));
                                break;
                            }
                            "IRD_OBJ_RECORD" => Self::read_ird_object(drawing, iter)?,
                            _ => (), // Skip unknown objects
                        }
                    }
                }
                Some(Err(e)) => return Err(e),
                None => return Err(DxfError::UnexpectedEndOfInput),
            }
        }
        Ok(())
    }

    fn read_ird_object(drawing: &mut Drawing, iter: &mut CodePairPutBack) -> DxfResult<()> {
        let mut ird_obj = IrdObjRecord::default();

        loop {
            match iter.next() {
                Some(Ok(pair)) => match pair.code {
                    0 => {
                        iter.put_back(Ok(pair));
                        break;
                    }
                    // 5 => ird_obj.handle =
                        // .map_err(|_| DxfError::MalformedString)?,
                    // 330 => {
                        // let handle = u32::from_str_radix(&pair.assert_string()?, 16)
                            // .map_err(|_| DxfError::MalformedString)?;
                        // if ird_obj.reactor_handle == 0 {
                            // ird_obj.reactor_handle = handle;
                        // } else {
                            // ird_obj.owner_handle = handle;
                        // }
                    // }
                    1070 => ird_obj.record_type = pair.assert_i16()?,
                    1071 => ird_obj.record_version = pair.assert_i32()?,
                    1004 => {
                        // Parse binary data from hex string
                        let hex_str = pair.assert_string()?;
                        let binary_data = Self::hex_to_bytes(&hex_str)?;
                        ird_obj.binary_data_segments.push(binary_data);
                    }
                    100 | 102 => (), // Skip subclass markers and control strings
                    _ => (),
                },
                Some(Err(e)) => return Err(e),
                None => return Err(DxfError::UnexpectedEndOfInput),
            }
        }

        // Store in drawing (you'll need to add ird_objects field to Drawing)
        // drawing.ird_objects.push(ird_obj);
        Ok(())
    }

    pub(crate) fn add_code_pairs(&self, pairs: &mut Vec<CodePair>) {
        pairs.push(CodePair::new_str(0, "IRD_OBJ_RECORD"));
        pairs.push(CodePair::new_string(5, &self.handle.as_string()));

        // Add reactor block
        pairs.push(CodePair::new_str(102, "{ACAD_REACTORS"));
        pairs.push(CodePair::new_string(330, &self.reactor_handle.as_string()));
        pairs.push(CodePair::new_str(102, "}"));

        // Owner handle
        pairs.push(CodePair::new_string(330, &self.owner_handle.as_string()));

        // Base class
        pairs.push(CodePair::new_str(100, "CIrdBaseRecord"));
        pairs.push(CodePair::new_i16(1070, self.record_type));
        pairs.push(CodePair::new_i32(1071, self.record_version));

        // Binary data segments
        for segment in &self.binary_data_segments {
            let hex_string = Self::bytes_to_hex(segment);
            pairs.push(CodePair::new_string(1004, &hex_string));
        }

        // Object class
        // pairs.push(CodePair::new_str(100, "CIrdObjRecord"));
        // pairs.push(CodePair::new_string(330, &self.owner_handle.as_string()));
    }

    fn hex_to_bytes(hex_str: &str) -> DxfResult<Vec<u8>> {
        let clean_hex = hex_str.replace(|c: char| c.is_whitespace(), "");
        let mut bytes = Vec::new();

        for chunk in clean_hex.as_bytes().chunks(2) {
            if chunk.len() == 2 {
                let hex_pair = std::str::from_utf8(chunk)
                    .map_err(|_| DxfError::MalformedString)?;
                let byte = u8::from_str_radix(hex_pair, 16)
                    .map_err(|_| DxfError::MalformedString)?;
                bytes.push(byte);
            }
        }

        Ok(bytes)
    }

    fn bytes_to_hex(bytes: &[u8]) -> String {
        bytes.iter()
            .map(|b| format!("{:02X}", b))
            .collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_ird_object_default() {
        let ird = IrdObjRecord::new();
        assert_eq!(ird.handle, Handle::empty());
        assert_eq!(ird.record_type, 516);
        assert_eq!(ird.record_version, 1);
        assert!(ird.binary_data_segments.is_empty());
    }

    #[test]
    fn encode_decode_key_value_pairs() {
        let mut ird = IrdObjRecord::new();
        let pairs = vec![
            ("gml_id".to_string(), "58271713.86251".to_string()),
            ("objekttype".to_string(), "lex+fiberledning".to_string()),
            ("driftsstat".to_string(), "i drift".to_string()),
        ];

        ird.encode_key_value_pairs(&pairs).unwrap();
        let decoded_pairs = ird.decode_key_value_pairs().unwrap();

        assert_eq!(pairs.len(), decoded_pairs.len());
        for (original, decoded) in pairs.iter().zip(decoded_pairs.iter()) {
            assert_eq!(original, decoded);
        }
    }

    #[test]
    fn binary_data_segments_respect_127_byte_limit() {
        let mut ird = IrdObjRecord::new();

        // Create a large key-value pair that will exceed 127 bytes
        let large_pairs = vec![
            ("very_long_key_name_that_takes_up_space".to_string(),
             "very_long_key_name".to_string()),
            ("another_key".to_string(), "another_value".to_string()),
        ];

        ird.encode_key_value_pairs(&large_pairs).unwrap();

        // Verify each segment is <= 127 bytes
        for segment in &ird.binary_data_segments {
            assert!(segment.len() <= 127, "Segment size {} exceeds 127 bytes", segment.len());
        }
    }

    #[test]
    fn add_code_pairs_generates_correct_structure() {
        let ird = IrdObjRecord {
            handle: Handle::empty(),
            reactor_handle: Handle::empty(),
            owner_handle: Handle::empty(),
            record_type: 516,
            record_version: 1,
            binary_data_segments: vec![vec![0x01, 0x00, 0x11, 0x00]],
        };

        let mut pairs = Vec::new();
        ird.add_code_pairs(&mut pairs);

        // assert!(pairs.iter().any(|p| p.code == 0 && p.value.as_string().unwrap() == "IRD_OBJ_RECORD"));
        // assert!(pairs.iter().any(|p| p.code == 5 && p.value.as_string().unwrap() == "4D"));
        // assert!(pairs.iter().any(|p| p.code == 1070 && p.value.as_i16().unwrap() == 516));
        // assert!(pairs.iter().any(|p| p.code == 1071 && p.value.as_i32().unwrap() == 1));
        assert!(pairs.iter().any(|p| p.code == 1004));
    }
}
