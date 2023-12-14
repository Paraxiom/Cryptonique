// // src/demo/htm_crypto_demo.rs

// use crate::htm::htm_model::HTMModel;
// use crate::htm::{encrypt_with_noise, decrypt_with_noise};
// use prettytable::*;
// use prettytable::{format, Table};
// pub fn run_htm_crypto_demo() {
//     println!("🔍 Running HTM Crypto Demo...");

//     // Initialize and evolve the HTM model to generate noise
//     let mut htm_model = HTMModel::new();
//     let noise_pattern = htm_model.generate_noise_pattern();

//     // Encrypt a sample message using the generated noise
//     let sample_message = "This is a sample message for encryption."
//         .as_bytes()
//         .to_vec();
//     let encrypted_message = encrypt_with_noise(&sample_message, &htm_model);

//     // Decrypt the message using the same noise pattern
//     let decrypted_message = decrypt_with_noise(&encrypted_message, &htm_model);

//     // Create and print a table for encryption results
//     let mut table = Table::new();
//     table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
//     table.add_row(row!["📝 Description", "📜 Message"]);
//     table.add_row(row![
//         "📄 Original Message",
//         String::from_utf8_lossy(&sample_message)
//     ]);
//     table.add_row(row![
//         "🔒 Encrypted Message",
//         format!("{:?}", encrypted_message)
//     ]);
//     table.add_row(row![
//         "🔓 Decrypted Message",
//         String::from_utf8_lossy(&decrypted_message)
//     ]);
//     table.printstd();

//     // Perform frequency transformation
//     let transformed_message =
//         crate::encryption::frequency_analysis::to_frequency_domain(&sample_message);
//     let mut transformed_message_mut = transformed_message.clone();
//     let transformed_back_message =
//         crate::encryption::frequency_analysis::to_time_domain(&mut transformed_message_mut);

//     // Create and print a table for transformation results
//     let mut transform_table = Table::new();
//     transform_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
//     transform_table.add_row(row!["🔄 Description", "⚡ Message"]);
//     transform_table.add_row(row![
//         "📄 Original Message",
//         String::from_utf8_lossy(&sample_message)
//     ]);
//     transform_table.add_row(row![
//         "⚡ Transformed Message",
//         format!("{:?}", transformed_message)
//     ]);
//     transform_table.add_row(row![
//         "🔄 Transformed Back Message",
//         String::from_utf8_lossy(&transformed_back_message)
//     ]);
//     transform_table.printstd();
// }
pub fn main() {}
