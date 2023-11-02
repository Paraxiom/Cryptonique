pub struct Transaction {
    sender: String,
    recipient: String,
    amount: u64,
    timestamp: u64,
    // Other fields can be added as necessary.
}



pub fn process_user_action(sender: &str, recipient: &str, amount: u64) -> Transaction {
    Transaction {
        sender: sender.to_string(),
        recipient: recipient.to_string(),
        amount,
        timestamp: get_current_timestamp(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation() {
        let tx = Transaction {
            sender: String::from("Alice"),
            receiver: String::from("Bob"),
            amount: 50.0,
            timestamp: 1234567890,
        };

        assert_eq!(tx.sender, "Alice");
        assert_eq!(tx.receiver, "Bob");
        assert_eq!(tx.amount, 50.0);
        assert_eq!(tx.timestamp, 1234567890);
    }

    #[test]
    fn test_process_user_action() {
        let raw_data = process_user_action("Alice", "Bob", 50.0, 1234567890);

        // We're assuming that the raw_data is a serialized version of the Transaction.
        // Here's a simple check to see if it contains some expected data.
        assert!(raw_data.contains("Alice"));
        assert!(raw_data.contains("Bob"));
    }
}



fn get_current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    since_the_epoch.as_secs()
}