use ode_solvers::*;
use std::{path::Path, fs::File, io::Write};

// Defining State to be a 3d-vector, 
// containing the cartesian positional coordinates.
pub type State = Vector3<f64>;

/*
impl Vector3<f64>
{
    fn get_x(&self) -> f64
    {
        return self.x;
    }
    fn get_y(&self) -> f64
    {
        return self.y;
    }
    fn get_z(&self) -> f64
    {
        return self.z;
    }
}
*/
 
type Time = f64;

  

// System is a trait containing the differential
// equations defining a dynamical system.
// It can contain several methods, making it reusable.
pub trait System<State> 
{
    fn system(&self, t: Time, x: &State, dx: &mut State);
    fn solution(&self, _t: Time, _x: &State, _dy: &State) -> bool {false}
}

// The constants in the Lorentz system.
pub struct LorentzSystem
{
    sigma: f64,
    rho: f64,
    beta: f64,
}

// The constants of the Rössler system
pub struct RosslerSystem
{
    a: f64,
    b: f64,
    c: f64,
}

// The initial conditions required by System is
// contained here
pub struct InitialConditions
{
    t_0: Time,
    t_end: Time,
    dt: Time,
    x_0: State,
    rtol: f64,
    atol: f64,
}

pub fn set_init_conditions( 
    t_0: f64, 
    t_end: f64, 
    dt: f64, 
    x_0: f64, 
    y_0: f64, 
    z_0: f64, 
    rtol: f64, 
    atol: f64) -> InitialConditions
{
    let x_0 = State::new(x_0, y_0, z_0);
    let init = InitialConditions 
    {
        t_0,
        t_end,
        dt,
        x_0,
        rtol,
        atol
    };
    return init;
}


// The definition of the Lorentz system, 
// as an instance of the System trait.
impl ode_solvers::System<State> for LorentzSystem
{
    fn system(&self, _t: Time, x: &State, dx: &mut State)
    {
        dx[0] = self.sigma * (x[1] - x[0]);
        dx[1] = x[0] * (self.rho - x[2]) - x[1];
        dx[2] = x[0] * x[1] - self.beta * x[2];
    }
}

// The definition of the Rossler system.
impl ode_solvers::System<State> for RosslerSystem
{
    fn system(&self, _t: Time, x: &State, dx: &mut State)
    {
        dx[0] = - x[1] - x[2];
        dx[1] = x[0] + self.a * x[1];
        dx[2] = self.b + x[2] * (x[0] - self.c);
    }
}

// Save states and times to a file.
// Data is listed on the form 
// t x y z
// with every line containing values for a specific time.
// Values are separated by tabs.
pub fn save_states(times: &Vec<Time>, states: &Vec<State>, filename: &Path)
{
    // Opens file with name filename. 
    // Creates file if none exists.
    let mut buffer = match File::create(filename)
    {
        Err(e) =>
        {
            println!("Could not open file. Error: {:?}", e);
            return;
        }
        Ok(buffer) => buffer,
    };

    // Write state of system for all t to a file
    for (i, state) in states.iter().enumerate()
    {
        buffer.write_fmt(format_args!("{}", times[i])).unwrap();
        for value in state.iter()
        {
            buffer.write_fmt(format_args!("\t{}", value)).unwrap();
        }
        buffer.write_fmt(format_args!("\n")).unwrap();
    }
    //println!("Saved data to: {:?}", filename);
}

// Initiating the computation of the system using the Dormand-Prince method.
// The method could be substituted for the 8th order method
// Dor853 without changes to any other part of the code.
pub fn integrate_lorentz(system: LorentzSystem, init: InitialConditions) -> (Vec<Time>, Vec<f64>, Vec<f64>, Vec<f64>)
{
    let mut stepper = Dopri5::new(system, init.t_0, init.t_end, init.dt, init.x_0, init.rtol, init.atol);

    let res = stepper.integrate();
    let t_out = stepper.x_out();
    let x_out = stepper.y_out();
    let (x, y, z) = state_to_vectors(&x_out, &t_out);

    // Result of computation could be stored in a file
    // for later analysis, possibly in Python's matplotlib.
    match res
    {
        Ok(stats) =>
        {
            //println!("Computation finished successfully.");
            println!("{}", stats);

            let path = Path::new("./data/lorentz.txt");
            save_states(t_out, x_out, path);
        }
        Err(e) => println!("An error occured: {}", e)
    };
    return (t_out.to_vec(), x, y, z); 
}

pub fn integrate_rossler(system: RosslerSystem, init: InitialConditions) -> (Vec<Time>, Vec<f64>, Vec<f64>, Vec<f64>)
{
    let mut stepper = Dopri5::new(system, init.t_0, init.t_end, init.dt, init.x_0, init.rtol, init.atol);

    let res = stepper.integrate();
    let t_out = stepper.x_out();
    let x_out = stepper.y_out();
    let (x, y, z) = state_to_vectors(&x_out, &t_out);

    // Result of computation could be stored in a file
    // for later analysis, possibly in Python's matplotlib.
    match res
    {
        Ok(stats) =>
        {
            //println!("Computation finished successfully.");
            println!("{}", stats);

            let path = Path::new("./data/lorentz.txt");
            save_states(t_out, x_out, path);
        }
        Err(e) => println!("An error occured: {}", e)
    };
    return (t_out.to_vec(), x, y, z); 
}


// Write x_out and t_out to vectors x, y, z and time:
pub fn state_to_vectors(s: &Vec<State>, t: &Vec<Time>) -> (Vec<f64>, Vec<f64>, Vec<f64>)
{
    let mut x: Vec<f64> = [].to_vec();
    let mut y: Vec<f64> = [].to_vec();
    let mut z: Vec<f64> = [].to_vec();
    for i in 0..t.len()
    {
        x.push(s[i][0]);
        y.push(s[i][1]);
        z.push(s[i][2]);
    }
    return (x, y, z);
}

// Intergrating the system using Dormand-Prince 5(4)
pub fn solve_lorentz_system(init: InitialConditions, sys: &str) -> (Vec<Time>, Vec<f64>, Vec<f64>, Vec<f64>)
{
    // Defining the parameters in the Lorentz system
    // as specified from project description.
    match sys
    {
    "lorentz" =>
    {
        let system = LorentzSystem{sigma: 10., rho: 28., beta: 8./3.};
        return integrate_lorentz(system, init);
    }
    "rossler" =>
    {
        let system = RosslerSystem{a: 0.2, b: 0.2, c: 5.7};
        return integrate_rossler(system, init);
    }
    _ => panic!("Error: System not recognized."),
    }
}
