#![feature(box_syntax)]

use std::env;
use std::panic;

const HELP_MSG: &str = concat!(
    "Usage:\n",
    "dist2steps HEIGHT DISTANCE\n",
    "\n",
    "Args:\n",
    "HEIGHT    -- positive floating point number\n",
    "DISTANCE  -- positive floating point number"
);

type Height = f64;
type Distance = f64;

fn main() {
    panic::set_hook(box |_| {
        println!("{}", HELP_MSG);
    });

    let (height, distance) = read_args();
    
    let step_len = height * 0.414;
    let steps = distance * 100_000.0 / step_len;
    let steps = steps.floor();

    println!("distance:    {:.2} kilometers", distance);
    println!("step length: {:.2} centimeters", step_len);
    println!("steps:       {} ", steps);
}

fn read_args() -> (Height, Distance) {
    let args: Vec<_> = env::args().collect();

    let height = args[1].parse().unwrap();
    let distance = args[2].parse().unwrap();

    (height, distance)
}