use cryptonique::htm::htm_model::HTMModel;
use cryptonique::htm::temporal_keys::TemporalKey;
use cryptonique::shared_state::SharedState;
use rand::Rng;
use std::time::{Duration, SystemTime};

#[test]
fn test_temporal_key_generation_and_validation() {
    // Initialize HTM Model and Shared State
    let htm_model = HTMModel::new();
    let shared_state = SharedState::new(htm_model.clone());

    // Generate a temporal key
    let mut rng = rand::thread_rng();
    let initial_key: Vec<u8> = (0..256).map(|_| rng.gen()).collect();
    let evolution_interval = Duration::from_secs(10);
    let temporal_key = TemporalKey::new(initial_key.clone(), htm_model, evolution_interval);

    // Store the key in Shared State
    let mut htm_model = shared_state.get_htm_model().lock().unwrap();
    htm_model.store_key(temporal_key.get_key().clone(), temporal_key.generation_time);

    // Retrieve and validate the key's timestamp
    let retrieved_timestamp = htm_model.get_key(&initial_key).unwrap();
    let now = SystemTime::now();
    let key_age = now.duration_since(*retrieved_timestamp).unwrap();

    assert!(
        key_age < evolution_interval,
        "Key age is greater than evolution interval"
    );

    // Assert that the retrieved timestamp matches the generation time
    assert_eq!(
        *retrieved_timestamp, temporal_key.generation_time,
        "Retrieved timestamp does not match the key's generation time"
    );
}
