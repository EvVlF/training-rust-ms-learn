use crate::Colors::{Blue, Green, Red, Silver};
use crate::Transmission::{Automatic, Manual};

#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission { Manual, SemiAuto, Automatic }

#[derive(PartialEq, Debug)]
enum Age { New, Used }

#[derive(PartialEq, Debug)]
enum Colors { Blue, Green, Red, Silver }

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Create a tuple for the car quality with the Age ("New" or "Used") and mileage
// Return a tuple with the arrow `->` syntax
fn car_quality(miles: u32) -> (Age, u32) {

    // Declare and initialize the return tuple value
    // For a new car, set the miles to 0
    let quality: (Age, u32) = (Age::New, miles);

    // Return the completed tuple to the caller
    quality
}

// Build a new "Car" using the values of four input arguments
// - color (String)
// - motor (Transmission enum)
// - roof (boolean, true if the car has a hard top roof)
// - miles (u32)
// Call the car_quality(miles) function to get the car age
// Return an instance of a "Car" struct with the arrow `->` syntax
fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    // Create a new "Car" instance as requested
    // - Bind first three fields to values of input arguments
    // - "age" field calls "car_quality" function with "miles" input argument
    Car { color, motor, roof, age: car_quality(miles) }
}

fn main() {
    // Create car color array
    let colors: [Colors; 4] = [Blue, Green, Red, Silver];

    // Declare the car type and initial values
    let mut car: Car = Car {
        color: "Blue".to_string(),
        motor: Automatic,
        roof: false,
        age: (Age::New, 2),
    };
    let mut engine: Transmission = Manual;
}