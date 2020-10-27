mod ode;
pub use ode::{set_init_conditions, solve_lorentz_system};

mod plotting;
use plotting::{plot_system};

mod analysis;
use analysis::
{
    find_maxima, 
    normalize_z_n, 
    generate_random_point, 
    separation_distance, 
    vectors_to_states, 
    analyze_two_paths,
    ln_of_vec,
    write_number,
};


fn main() 
{   
    task_2("lorentz");
    task_2("rossler");
}


fn task_2(sys: &str)
{
    // Task 2: The Lorentz equations
    // Defining the initial conditions for the Lorentz system.
    let mut init = set_init_conditions( 0., 50., 1e-2, 1e-3, 1e-3, 1e-3, 1e-4, 1e-4);

    let font = "Arial";
    let font_size = 20.;

    let (t, x, _y, z) = solve_lorentz_system(init, sys);

    // 2 Warm up: Plot of x and z positions:
    plot_system(&x, &z, "Lorentz attractor", "x", "z", font, font_size, "lines");

    // 2a: Plot z vs time:
    plot_system(&t, &z, "Lorentz attractor", "t", "z", font, font_size, "lines");

    // Drop all prevoius data. This should be saved already.
    // TODO: Save init conditions somewhere, and reload them for analysis.
    drop(x);
    drop(_y);
    drop(z);
    drop(t);

    // 2b: For a very large t_end, plot the n+1th maximum og z
    // as a function of the nth maximum.

    // Increases t_end, and keeps all other conditions as before.
    // This all would be neater if the first calculations went to
    // t_end = 1e5, and the first plots contained only the times
    // up to t=50.
    init = set_init_conditions( 0., 4e2, 1e-2, 1e-3, 1e-3, 1e-3, 1e-4, 1e-4);

    // Note: ode_solvers stops computing after exceeding 1e5 steps.
    let (_t, _x, _y, z) = solve_lorentz_system(init, sys);

    // Finds the maxima of the z-coordinate.
    let z_n = find_maxima(&z);

    // Plots each maxima as a function of the previous one.
    // Note: The length of z_n and z_n+1 are not equal,
    // so the final z_n is not plotted, as is intended.
    let z_next = normalize_z_n(&z_n);
    plot_system(
        &z_n.to_vec(), &z_n[1..].to_vec(), 
        "Maxima of z in Lorentz system", 
        "z_n", "z_n+1", font, font_size, "points"); 
    plot_system(
        &z_n.to_vec(), &z_next, 
        "Normalized maxima of z in Lorentz system", 
        "z_n", "z_n+1/z_n", font, font_size, "points"); 

    let n = 1;
    let mut t_cut = 30.;
    println!("From the following plots, identify the cutoff time
        where the separation distance no longer grows exponentially");
    trajectories_on_attractor(n, t_cut, sys, font, font_size);

    // Technically, this could be retrieved from user input.
    println!("Enter the cutoff time.");
    t_cut = write_number();
    trajectories_on_attractor(n, t_cut, sys, font, font_size);
}

fn trajectories_on_attractor(n: u8, t_cut: f64, sys: &str, font: &str, font_size: f64)
{
    // Maximum separation distance
    let d: f64 = 1e-6;

    // Initializing the situation. 
    // Creating a particle in the origin, and allowing it to travel
    // for 50 time units around the strange attractor.
    let (init_x_0, init_y_0, init_z_0) = generate_random_point(0., 0., 0., d);
    let mut init_0 = set_init_conditions( 0., 50., 1e-2, init_x_0, init_y_0, init_z_0, 1e-4, 1e-4);
    let (t_0, x_0, y_0, z_0) = solve_lorentz_system(init_0, sys);

    // Then storing the endpoint of the trajectory, and using it as
    // a starting point for two new trajectories: One starting
    // in the endpoint, and one starting very close to it.
    // This is the point called x_0 in the project description.
    let new_init_x = x_0.last().copied().unwrap();
    let new_init_y = y_0.last().copied().unwrap();
    let new_init_z = z_0.last().copied().unwrap();
    //let new_init_t = t_0.last().copied().unwrap();

    for i in 0..n
    {
        // Sets the endpoint of previous computation as starting point of a new one, 
        // and sets a starting point (init_x_1 etc.) as a random point close to the
        // other one.
        let (init_x_1, init_y_1, init_z_1) = generate_random_point(new_init_x, new_init_y, new_init_z, d);
        let init_0 = set_init_conditions( 0., t_cut, 1e-2, new_init_x, new_init_y, new_init_z, 1e-4, 1e-4);
        let mut init_1 = set_init_conditions( 0., t_cut, 1e-2, init_x_1, init_y_1, init_z_1, 1e-4, 1e-4);

        // Propagates the two particles for t_cut time units.
        let (t_1, x_1, y_1, z_1) = solve_lorentz_system(init_1, sys);
        let (t_2, x_2, y_2, z_2) = solve_lorentz_system(init_0, sys);
        //plot_system(&x_2, &x_1, "Lorentz attractor", "x", "z", font, font_size, "lines");
        //plot_system(&z_2, &z_1, "Lorentz attractor", "x", "z", font, font_size, "lines");
        //plot_system(&t_2, &t_1, "t_0 vs t_1", "t_0", "t_1", font, font_size, "lines");
        let r_0 = vectors_to_states(&x_2, &y_2, &z_2);
        let r_1 = vectors_to_states(&x_1, &y_1, &z_1);

        // Calculates the distance between the particles for every time step.
        let sep_dist = analyze_two_paths(&r_0, &r_1, &separation_distance);
        let log_sep_dist = ln_of_vec(&sep_dist);

        // Plots the logarithm of the distance between
        // the particles as a function of time.
        plot_system
        (
            &t_1, 
            &log_sep_dist, 
            &format!("Separation distance between two paths, t_cut = {:?}", t_cut), 
            "Time", 
            "|x_0 - x_1|", 
            font, 
            font_size, 
            "lines"
        );
    }
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
