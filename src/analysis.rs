use rand::Rng;
use num::pow;
use std::io;

use crate::ode::{State};

pub type Point =  State;

pub fn find_maxima(x: &Vec<f64>) -> Vec<f64>
{
    let mut maxima: Vec<f64> = [].to_vec();
    maxima.push(0.);
    for i in 1..x.len()-1
    {
        if x[i-1] < x[i] && x[i+1] < x[i]
        {
            maxima.push(x[i]);
        }
    }
    return maxima;
}

pub fn normalize_z_n(z_n: &Vec<f64>) -> Vec<f64>
{
    let mut z_next: Vec<f64> = [].to_vec();
    for i in 0..z_n.len()-1
    {
        z_next.push(z_n[i+1]/z_n[i]);
    }
    return z_next;
}

// Generate a point within distance d of some other point.
// Probability distrubution around x_0 is not entirely uniform
// in the sphere of radius d, but is uniform in a cube around x_0.
// TODO: ode::State could have methods get_x, get_y etc.
pub fn generate_random_point(x_0: f64, y_0: f64, z_0: f64, d: f64) -> (f64, f64, f64)
{
    let mut rng = rand::thread_rng();

    // Generaate three random numbers in range -1/3..1/3.
    // TODO: There must be a neat function that does this.
    let x: f64 = x_0 + (2.*rng.gen::<f64>()/3. - 1./3.)*d;
    let y: f64 = y_0 + (2.*rng.gen::<f64>()/3. - 1./3.)*d;
    let z: f64 = z_0 + (2.*rng.gen::<f64>()/3. - 1./3.)*d;

    assert!(pow(x - x_0, 2) + pow(y - y_0, 2) + pow(z - z_0, 2) < 1e-12);
    return (x, y, z);
}

// Returns the distance between two points.
pub fn separation_distance(r_0: &Point, r_1: &Point) -> f64
{
    let d:f64 = pow(r_0.x - r_1.x, 2) + pow(r_0.y - r_1.y, 2) + pow(r_0.z - r_1.z, 2);
    return d.sqrt();
}

// Transforms three vectors of floats to one vector of Points.
pub fn vectors_to_states(x: &Vec<f64>, y: &Vec<f64>, z: &Vec<f64>) -> Vec<Point>
{
    assert!(x.len() == y.len() && y.len() == z.len());
    let mut points: Vec<Point> = [].to_vec();
    for i in 0..x.len()
    {
        let r_i = Point::new(x[i], y[i], z[i]);
        points.push(r_i);
    }
    return points;
}

// For each value in paths 1 and 2, this calls a
// function f, and returns the values produced
// by f as a function.
pub fn analyze_two_paths(
    path_1: &Vec<Point>, 
    path_2: &Vec<Point>, 
    f: &dyn Fn(&Point, &Point) -> f64) -> Vec<f64>
{
    assert!(path_1.len() == path_2.len());
    let mut values: Vec<f64> = Vec::new();
    for i in 0..path_1.len()
    {
        values.push(f(&path_1[i], &path_2[i]));
    }
    return values;
}

pub fn ln_of_vec(v: &Vec<f64>) -> Vec<f64>
{
    let mut log_v = Vec::new();
    for el in v.iter()
    {
        log_v.push(el.ln());
    }
    return log_v;
}

pub fn write_number() -> f64
{
    println!("Please enter a number: ");
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read input");
    let num: f64 = line.trim().parse().expect("Invalid input");
    println!("{} is accepted as input.", num);
    return num;
}

