use gnuplot::{Figure, Color, TextOffset};
//use gnuplot::LineStyle; 
use gnuplot::*;

pub fn plot_system(x: &Vec<f64>, y: &Vec<f64>, title: &str, x_label: &str, y_label: &str, font: &str, font_size: f64, style: &str, fig: &mut Figure)
{
    if style == "lines"
    {
        fig.axes2d()
            .set_x_label(x_label, &[Font(font, font_size), TextOffset(0., 5.)])
            .set_y_label(y_label, &[Font(font, font_size)])
            .set_title(title, &[Font(font, font_size)])
            .lines(x, y, &[Color("black"), LineWidth(2.5)]);
    }
    else if style == "points"
    {
        fig.axes2d()
            .set_x_label(x_label, &[Font(font, font_size), TextOffset(0., 2.)])
            .set_y_label(y_label, &[Font(font, font_size)])
            .set_title(title, &[Font(font, font_size)])
            .points(x, y, &[Color("black"), PointSize(1.5), PointSymbol('O')]);
    }
    else
    {
        let e = "Error: Plot style argument not correctly given";
        println!("{}", e);
    }
}

pub fn show_system(fig: &mut Figure)
{
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


