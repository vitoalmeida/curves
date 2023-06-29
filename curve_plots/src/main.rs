// Import required modules from the plotters crate for plotting
use plotters::prelude::*;

// Import the Complex type from the num-complex crate for handling complex numbers
use num_complex::Complex;

// The main function that returns a Result type, allowing the use of the ? operator for error handling
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a drawing area with a bitmap backend, and set the size to 800x600 pixels
    let root = BitMapBackend::new("plot.png", (800, 600)).into_drawing_area();
    // Fill the drawing area with a white background
    root.fill(&WHITE)?;

    // Create a chart builder on the drawing area with specified margins and label area sizes
    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(-10f32..10f32, -10f32..10f32)?;

    // Draw the grid (mesh) on the chart
    chart.configure_mesh().draw()?;

    // Define the elliptic curve equation parameters
    let a = 2.0;
    let b = 2.0;

    // Calculate the points on the elliptic curve using the given equation
    let series: Vec<(f32, f32)> = (0..1000)
        .flat_map(|x| {
            // Convert the integer x to a floating-point number and map it to the range [-5, 5]
            let x = x as f32 / 100.0 - 5.0;
            // Calculate the square of the y-coordinate using the elliptic curve equation
            let y_squared = Complex::new(x.powi(3) + a * x + b, 0.0);
            // Calculate the square root of y_squared
            let y = y_squared.sqrt();
            // If the imaginary part of y is 0, include the point (x, y) and its symmetric point (x, -y)
            if y.im == 0.0 {
                vec![(x, y.re), (x, -y.re)]
            } else {
                vec![]
            }
        })
        .collect();

    // Draw each point in the series as a circle with the specified size and color
    for point in series {
        chart.draw_series(std::iter::once(Circle::new(point, 2, BLUE.filled())))?;
    }
    // Return the result
    Ok(())
}
