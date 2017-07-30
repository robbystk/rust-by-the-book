use std::io;

fn main() {
    loop {
        // get temperature
        println!("Enter temperature");

        let mut temp = String::new();

        io::stdin().read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_)  => {
                println!("Please type a number!");
                continue;
            },
        };

        println!("Enter units (F/C)");

        let mut unit = String::new();

        io::stdin().read_line(&mut unit)
            .expect("Failed to read line");

        let unit = unit.trim();

        if unit == "F" {
            println!("{} degrees C", f_to_c(temp));
        } else if unit == "C" {
            println!("{} degrees F", c_to_f(temp));
        } else {
            println!("Please type 'F' or 'C'");
        };
    }
}

fn f_to_c(temp: f32) -> f32 {
    (temp - 32f32) / 1.8
}

fn c_to_f(temp: f32) -> f32 {
    temp * 1.8 + 32f32
}
