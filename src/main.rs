mod ode;
use ode::{set_init_conditions, solve_lorentz_system};

mod plotting;
use plotting::{plot_system};

fn main() 
{
    // Defining the initial conditions for the Lorentz system.
    let init = set_init_conditions( 0., 50., 1e-2, 1e-3, 1e-3, 1e-3, 1e-4, 1e-4);

    let font = "Arial";
    let font_size = 20.;

    let (t, x, y, z) = solve_lorentz_system(init);
    plot_system(x, z, "Lorentz attractor", "x", "z", font, font_size)
}


// Read data from file into plottable vectors:
/*
let mut buffer = match File::open("lorentz.txt")
{
    Err(e) =>
    {
        println!("Could not open file. Error: {:?}", e);
        return;
    }
    Ok(buffer) => buffer,
};
let file_reader = BufReader::new(buffer);
let stats_from_file = file_reader.lines().filter_map(std::io::Result::ok).collect();
println!("", stats_from_file[..][0]);
*/
