pub fn blackman_window(size: usize) -> Vec<f64> {
    (0..size)
        .map(|i| {
            0.42 - 0.5 * (2.0 * PI * i as f64 / (size as f64 - 1.0)).cos()
                + 0.08 * (4.0 * PI * i as f64 / (size as f64 - 1.0)).cos()
        })
        .collect()
}

pub fn lowpass_filter(cutoff: f64, band: f64) -> Vec<f64> {
    let mut n = (4.0 / band).ceil() as usize;
    if n % 2 == 1 {
        n += 1;
    }

    let sinc = |x: f64| -> f64 { (x * PI).sin() / (x * PI) };

    let sinc_wave: Vec<f64> = (0..n)
        .map(|i| sinc(2.0 * cutoff * (i as f64 - (n as f64 - 1.0) / 2.0)))
        .collect();

    let blackman_window = blackman_window(n);

    let filter: Vec<f64> = sinc_wave
        .iter()
        .zip(blackman_window.iter())
        .map(|tup| *tup.0 * *tup.1)
        .collect();

    // Normalize
    let sum = filter.iter().fold(0.0, |acc, &el| acc + el);

    filter.iter().map(|&el| el / sum).collect()
}

pub fn spectral_invert(filter: &[f64]) -> Vec<f64> {
    assert_eq!(filter.len() % 2, 0);
    let mut count = 0;

    filter
        .iter()
        .map(|&el| {
            let add = if count == filter.len() / 2 { 1.0 } else { 0.0 };
            count += 1;
            -el + add
        })
        .collect()
}

pub fn convolve(filter: &[f64], input: &[f64]) -> Vec<f64> {
    let mut output: Vec<f64> = Vec::new();
    let h_len = (filter.len() / 2) as isize;

    for i in -(filter.len() as isize / 2)..(input.len() as isize - 1) {
        output.push(0.0);
        for j in 0isize..filter.len() as isize {
            let input_idx = i + j;
            let output_idx = i + h_len;
            if input_idx < 0 || input_idx >= input.len() as isize {
                continue;
            }
            output[output_idx as usize] += input[input_idx as usize] * filter[j as usize]
        }
    }

    output
}

pub fn highpass_filter(cutoff: f64, band: f64) -> Vec<f64> {
    spectral_invert(&lowpass_filter(cutoff, band))
}

pub fn bandpass_filter(low_frequency: f64, high_frequency: f64, band: f64) -> Vec<f64> {
    assert!(low_frequency <= high_frequency);
    let lowpass = lowpass_filter(high_frequency, band);
    let highpass = highpass_filter(low_frequency, band);
    convolve(&highpass, &lowpass)
}


pub fn cutoff_from_frequency(frequency: f64, sample_rate: usize) -> f64 {
    frequency / sample_rate as f64
}
