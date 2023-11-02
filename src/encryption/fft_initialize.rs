use rustfft::algorithm::Radix4;
use rustfft::num_complex::Complex;
use rustfft::Fft;

pub fn fft_initialize(key: &[u8]) -> Vec<Complex<f32>> {
    // Pad the key to make its length a power of two
    let mut padded_key = key.to_vec();
    while padded_key.len() & (padded_key.len() - 1) != 0 {
        padded_key.push(0);
    }

    // Perform FFT on the padded_key
    let mut input_output: Vec<Complex<f32>> = padded_key
        .iter()
        .map(|&x| Complex::new(x as f32, 0.0))
        .collect();

    let fft = Radix4::new(padded_key.len(), rustfft::FftDirection::Forward);
    fft.process(&mut input_output);

    input_output
}
