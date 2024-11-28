// use std::collections::HashMap;
// use serde::Deserialize;
// use serde_json::Value;
// use std::path::PathBuf;

// // Dont warn unused imports


// /// Test case structure matching Ethereum's RLP test format
// #[derive(Debug, Deserialize)]
// struct TestCase {
//     #[serde(rename = "in")]
//     input: Value, // This is not strict. This is a flaw in the code.
//     out: Vec<u8>,
// }

// fn get_test_path(filename: &str) -> PathBuf {
//     let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
//     path.push("tests");
//     path.push("ethereum-tests");
//     path.push("RLPTests");
//     path.push(filename);
//     path
// }

// fn hex_to_bytes(hex: &str) -> Vec<u8> {
//     let hex = hex.strip_prefix("0x").unwrap_or(hex);
//     (0..hex.len())
//         .step_by(2)
//         .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
//         .collect()
// }

// /// Convert test input value to bytes for encoding
// fn parse_test_input(input: &Value) -> Vec<u8> {
//     match input {
//         Value::String(s) => {
//             if s.starts_with('#') {
//                 // Handle big integers prefixed with #
//                 let num = &s[1..];
//                 // For now just return empty vec, we'll implement big int handling later
//                 Vec::new()
//             } else {
//                 s.as_bytes().to_vec()
//             }
//         },
//         Value::Number(n) => {
//             if let Some(num) = n.as_u64() {
//                 if num == 0 {
//                     Vec::new()
//                 } else {
//                     vec![num as u8]
//                 }
//             } else {
//                 Vec::new()
//             }
//         },
//         Value::Array(arr) => {
//             // For arrays, we'll implement proper handling later
//             Vec::new()
//         },
//         Value::Null => Vec::new(),
//         _ => panic!("Unsupported test input type"),
//     }
// }

// #[test]
// fn test_valid_rlp() {
//     let path = get_test_path("rlptest.json");
//     let data = std::fs::read_to_string(path).expect("Failed to read test file");
//     let tests: HashMap<String, TestCase> = serde_json::from_str(&data)
//         .expect("Failed to parse test file");

//     for (name, test) in tests {
//         let expected = hex_to_bytes(&test.out);
//         let input = parse_test_input(&test.input);

//         println!("Running test: {}", name);
//         println!("Input: {:?}", test.input);
//         println!("Expected output: 0x{}", test.out);
        
//         // TODO: Once encode is implemented, add:
//         let result = rrlp::encode(&input);
//         assert_eq!(result, expected, "Test case: {}", name);
//     }
// }

// #[test]
// fn test_invalid_rlp() {
//     let path = get_test_path("invalidRLPTest.json");
//     let data = std::fs::read_to_string(path).expect("Failed to read test file");
//     let tests: HashMap<String, TestCase> = serde_json::from_str(&data)
//         .expect("Failed to parse test file");

//     for (name, test) in tests {
//         let invalid_rlp = hex_to_bytes(&test.out);
        
//         println!("Running test: {}", name);
//         println!("Invalid RLP: 0x{}", test.out);
        
//         // TODO: Once decode is implemented, add:
//         // let result = rlp::decode::<Vec<u8>>(&invalid_rlp);
//         // assert!(result.is_err(), "Test case should fail: {}", name);
//     }
// }

// // Helper tests to verify our test infrastructure works
// #[test]
// fn test_hex_conversion() {
//     assert_eq!(hex_to_bytes("0x00"), vec![0]);
//     assert_eq!(hex_to_bytes("0x0f"), vec![15]);
//     assert_eq!(hex_to_bytes("0x1234"), vec![0x12, 0x34]);
//     assert_eq!(hex_to_bytes("1234"), vec![0x12, 0x34]);
// }

// #[test]
// fn test_input_parsing() {
//     use serde_json::json;
    
//     // Test string
//     assert_eq!(parse_test_input(&json!("dog")), b"dog");
    
//     // Test number
//     assert_eq!(parse_test_input(&json!(0)), Vec::<u8>::new());
//     assert_eq!(parse_test_input(&json!(15)), vec![15]);
    
//     // Test null
//     assert_eq!(parse_test_input(&json!(null)), Vec::<u8>::new());
// }