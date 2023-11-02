extern crate rustfft;
use rustfft::algorithm::Radix4;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::Fft;
use rustfft::FftDirection; // Import the Fft trait
use rustfft::FftPlanner;

pub fn select_bands(spectrum: &Vec<Complex<f32>>, num_bands: usize) -> Vec<Complex<f32>> {
    let mut magnitudes: Vec<(usize, f32)> = spectrum
        .iter()
        .enumerate()
        .map(|(i, freq)| (i, freq.norm()))
        .collect();
    magnitudes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut result = vec![Complex::zero(); spectrum.len()];
    for i in 0..num_bands {
        let index = magnitudes[i].0;
        result[index] = spectrum[index];
    }
    result
}

pub fn to_frequency_domain(data: &[u8]) -> Vec<Complex<f32>> {
    // Pad the data to make its length a power of two
    let mut padded_data = data.to_vec();
    while padded_data.len() & (padded_data.len() - 1) != 0 {
        padded_data.push(0);
    }

    let mut input_output: Vec<Complex<f32>> = padded_data
        .iter()
        .map(|&x| Complex::new(x as f32, 0.0))
        .collect();
    let fft = Radix4::new(padded_data.len(), FftDirection::Forward);
    fft.process(&mut input_output);

    input_output
}

pub fn to_time_domain(data: &mut [Complex<f32>]) -> Vec<u8> {
    let mut planner = FftPlanner::new();
    let mut time_data = data.to_vec();

    let ifft = planner.plan_fft(data.len(), FftDirection::Inverse);

    ifft.process(&mut time_data);

    for item in time_data.iter_mut() {
        *item = *item / data.len() as f32;
    }

    let mut byte_data = Vec::new();
    for item in time_data {
        byte_data.push(item.re as u8);
    }

    byte_data
}
