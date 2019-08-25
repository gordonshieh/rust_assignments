use std::io;

fn calculate_slope(x1: f32, x2: f32, y1: f32, y2: f32) -> f32 {
    (y2 - y1) / (x2 - x1)
}

fn calculate_intercept(slope: f32, x: f32, y: f32) -> f32 {
    y - (slope * x)
}

fn calculate_x_coordinate(slope: f32, intercept: f32, y: f32) -> f32 {
    (y - intercept) / slope
}

fn calculate_y_coordinate(slope: f32, intercept: f32, x: f32) -> f32 {
    (slope * x) + intercept
}

fn main() {
    loop {
        let mut input_buffer = String::new();
        println!("Enter x1: ");
        io::stdin().read_line(&mut input_buffer)
            .expect("failed to read x1");

        let x1: f32 = input_buffer.trim().parse().unwrap();

        input_buffer.clear();
        println!("Enter x2: ");
        io::stdin().read_line(&mut input_buffer)
            .expect("failed to read x2");

        let x2: f32 = input_buffer.trim().parse().unwrap();

        input_buffer.clear();
        println!("Enter y1: ");
        io::stdin().read_line(&mut input_buffer)
            .expect("failed to read y1");

        let y1: f32 = input_buffer.trim().parse().unwrap();

        input_buffer.clear();
        println!("Enter y2: ");
        io::stdin().read_line(&mut input_buffer)
            .expect("failed to read y2");

        let y2: f32 = input_buffer.trim().parse().unwrap();

        let slope = calculate_slope(x1, x2, y1, y2);
        let intercept = calculate_intercept(slope, x2, y2);
        println!("Equation of the graph:\ny = {}x + {}", slope, intercept);

        input_buffer.clear();
        println!("Enter a y-coordinate:");
        io::stdin().read_line(&mut input_buffer)
            .expect("failed to read y-coordinate");

        let input_y: f32 = input_buffer.trim().parse().unwrap();
        println!("x: {}", calculate_x_coordinate(slope, intercept, input_y));

        input_buffer.clear();
        println!("Enter a x-coordinate:");
        io::stdin().read_line(&mut input_buffer)
            .expect("failed to read x-coordinate");

        let input_x: f32 = input_buffer.trim().parse().unwrap();
        println!("y: {}", calculate_y_coordinate(slope, intercept, input_x));

        input_buffer.clear();
        println!("Press 1 to continue, 0 to exit.");
        io::stdin().read_line(&mut input_buffer)
            .expect("failed to read input");
        match input_buffer.trim().parse().unwrap() {
            0 => { break },
            1 => { continue; },
            _ => { continue; }
        };
    }
}