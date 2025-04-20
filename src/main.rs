fn main() {
    let x: Vec<f64> = (1..=10).map(|x| x as f64).collect();
    println!("input -> {:?}", x);
    let y: Vec<f64> = (1..=10).map(|x| x as f64).collect();
    println!("output -> {:?}", y);

    let x_mean: f64 = x.iter().sum::<f64>() / x.len() as f64;
    println!("input mean -> {:?}", x_mean);
    let y_mean = y.iter().sum::<f64>() / y.len() as f64;
    println!("output mean -> {:?}", y_mean);

    let numerator: f64 = x
        .iter()
        .zip(y.iter())
        .map(|(xi, yi)| (xi - x_mean) * (yi - y_mean))
        .sum();
    println!("numerator -> {}", numerator);
    let denominator: f64 = x.iter().map(|xi| (xi - x_mean).powi(2)).sum();
    println!("denominator -> {}", denominator);

    let m = numerator / denominator;
    let b = y_mean - m * x_mean;
    println!(
        "linear regression model equation -> y = {:.2}x + {:.2}",
        m, b
    );

    let mse = x
        .iter()
        .zip(y.iter())
        .map(|(xi, yi)| {
            let y_pred = predict(*xi, m, b);
            (yi - y_pred).powi(2)
        })
        .sum::<f64>()
        / x.len() as f64;
    println!("mean squared error -> {:.2}", mse);

    let sample_input = 3.7;
    let prediction = predict(sample_input, m, b);
    println!("sample input -> {}", sample_input);
    println!("predicted output -> {:.2}", prediction);
}

fn predict(x: f64, m: f64, b: f64) -> f64 {
    m * x + b
}