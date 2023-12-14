use cryptonique::htm::htm_model::HTMModel;
use cryptonique::htm::temporal_keys::TemporalKey;
use cryptonique::shared_state::SharedState;
use rand::Rng;
use std::time::{Duration, SystemTime};

#[test]
fn test_temporal_key_generation_and_validation() {
    // Step 1: Initialize the test environment and instantiate the `SharedState` object.
    let htm_model = HTMModel::new();
    let shared_state = SharedState::new(htm_model.clone());

    // Step 2: Generate a temporal key seeded with the current time using a pseudo-random number generator.
    let mut rng = rand::thread_rng();
    let initial_key: Vec<u8> = (0..256).map(|_| rng.gen::<u8>()).collect();
    let evolution_interval = Duration::from_secs(10);
    let mut temporal_key = TemporalKey::new(initial_key.clone(), htm_model, evolution_interval);

    // Step 3: Store this key along with its timestamp in the `SharedState` object.
    let mut htm_model = shared_state.get_htm_model().lock().unwrap();
    htm_model.store_key(temporal_key.get_key().clone(), temporal_key.generation_time);

    // Step 4: Retrieve the key from `SharedState` and validate its timestamp to ensure it's within the expected validity period.
    let timestamp = htm_model.get_key(&initial_key).unwrap();
    let now = SystemTime::now();
    let key_age = now.duration_since(*timestamp).unwrap();
    assert!(key_age < evolution_interval);

    // Step 5: Assert that the retrieved timestamp matches the original key's generation time.
    assert_eq!(timestamp, &temporal_key.generation_time);
}
