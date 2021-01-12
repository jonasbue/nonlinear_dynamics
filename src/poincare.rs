use std::f64::consts::{E};
use gnuplot::{Figure, TextOffset};
use gnuplot::*;

use crate::plotting::{show_system, plot_system};

fn p(r: f64) -> f64
{
    let r_1 = (1. + E.powi(-1) * (r.powi(-2) - 1.)).powf(-0.5);
    return r_1;
}

fn make_cobweb(p: &dyn Fn(f64) -> f64, n: i32, r_0: f64) -> (Vec<f64>, Vec<f64>)
{
    let mut x: Vec<f64> = Vec::new();
    let mut y: Vec<f64> = Vec::new();
    let mut r: f64 = r_0;

    x.push(r);
    y.push(0.);
    for i in 1..n
    {
        if i%2 != 0
        {
            x.push(r);
            y.push(p(r));
            r = p(r);
        }
        else
        {
            y.push(r);
            x.push(r);
        }
    }
    return (x, y);
}


// Plots a cobweb constriction for a Poincaré mapping.
// Takes a one dimensional Poincaré map as argument.
fn cobweb(p: &dyn Fn(f64) -> f64, n: i32, r_0: f64, r_1: f64)
{
    let mut x: Vec<f64> = Vec::new();
    let mut p_curve: Vec<f64> = Vec::new();

    let dl = 0.01;
    let l = 2.;
    for i in 0..((l/dl) as i32)
    {
        let f = (i as f64)*dl;
        x.push(f);
        p_curve.push(p(f));
    }
    let (cob_x_lower, cob_y_lower) = make_cobweb(p, n, r_0);
    let (cob_x_upper, cob_y_upper) = make_cobweb(p, n, r_1);


    let font = "times new roman";
    let font_size = 40.;

    let mut fig = Figure::new();
    fig.axes2d()
        .set_x_label("r", &[Font(&font, font_size), TextOffset(0., 5.)])
        .set_y_label("r_1", &[Font(&font, font_size)])
        .set_title("Poincaré cobweb construction", &[Font(&font, font_size)])
        .lines(&x, &p_curve, &[LineWidth(2.), Color("black")])
        .lines(&x, &x, &[LineWidth(2.), Color("black")])
        .lines(&cob_x_lower, &cob_y_lower, &[LineStyle(Dot), LineWidth(2.), Color("black")])
        .lines(&cob_x_upper, &cob_y_upper, &[LineStyle(Dot), LineWidth(2.), Color("black")]);
    show_system(&mut fig);

    
    let mut iteration: Vec<f64> = Vec::new();
    for i in 0..n
    {
        iteration.push(i as f64);
    }

    fig = Figure::new();
    plot_system
    (
        &iteration,
        &cob_x_lower,
        "r as a function of iteration number",
        "Iteration number",
        "r",
        font,
        font_size,
        "lines",
        &mut fig
    );
    show_system(&mut fig);
}

pub fn task_3()
{
    cobweb(&p, 30, 0.1, 1.8);
}

