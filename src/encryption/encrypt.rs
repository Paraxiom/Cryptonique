use crate::shared_state::SharedState;
use crate::encryption::noise::generate_noise;
use crate::encryption::confusion_diffusion::confusion::apply_confusion;
use crate::encryption::confusion_diffusion::diffusion::apply_diffusion;

pub fn encrypt(message: &Vec<u8>, public_key: &Vec<u8>, state: &mut SharedState) -> Vec<u8> {
    let mut encrypted_message = message.clone();

    // Apply confusion
    apply_confusion(&mut encrypted_message, public_key);

    // Apply diffusion
    apply_diffusion(&mut encrypted_message);

    encrypted_message
}
