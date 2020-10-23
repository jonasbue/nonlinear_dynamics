use gnuplot::{Figure, Color};
//use gnuplot::LineStyle; 
use gnuplot::*;

pub fn plot_system(x: Vec<f64>, y: Vec<f64>, title: &str, x_label: &str, y_label: &str, font: &str, font_size: f64)
{
    let mut fig = Figure::new();
    fig.axes2d()
        .set_x_label(x_label, &[Font(font, font_size)])
        .set_y_label(y_label, &[Font(font, font_size)])
        .set_title(title, &[Font(font, font_size)])
        .lines(&x, &y, &[Color("black")]);
    match fig.show()
    {
        Ok(show) =>
        {
            println!("Figure rendered correctly");
            drop(show);
        }
        Err(gnu_error) => println!("Figure could not be rendered: {:?}", gnu_error),
    };
}


