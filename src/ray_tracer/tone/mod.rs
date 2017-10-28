use super::color::Color;

// e ^ (1/n SUM( ln( luminance[x][y] + delta ) ) )
pub fn log_average_luminance(
    color_buffer: &mut Vec<Vec<Color>>,
    width: usize,
    height: usize,
    delta: f64,
) -> f64 {
    let mut sum: f64 = 0f64;
    for x in 0..width {
        for y in 0..height {
            sum += (color_buffer[x][y].to_luminance() + delta).ln();
        }
    }

    (sum / (width * height) as f64).exp()
}

pub fn reinhard_tone_correction(
    color_buffer: &mut Vec<Vec<Color>>,
    width: usize,
    height: usize,
    key_value: f64,
    delta: f64,
) {
    let scale_factor: f64 = key_value / log_average_luminance(color_buffer, width, height, delta);
    for x in 0..width {
        for y in 0..height {
            let color: Color = color_buffer[x][y] * scale_factor;
            color_buffer[x][y] = color / (color + 1f64);
        }
    }
}
