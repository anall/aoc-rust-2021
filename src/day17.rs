#![warn( clippy::pedantic )]
use std::io::BufRead;
use adventlib::aoc;
use std::convert::TryFrom;

struct CalculateExtent(i32);

impl CalculateExtent {
    pub fn evaluate(&self, t : i32) -> i32 {
        -(2*self.0-(t*t))/(2*t)
    }
}

fn main() -> aoc::Result<()> {
    let (dest_x_min,dest_x_max) : (i32,i32) = (288,330);
    let (dest_y_min,dest_y_max) : (i32,i32) = (96,-50);

    let (y_min,y_max) = (CalculateExtent(dest_y_min),CalculateExtent(dest_y_max));

    assert!(dest_x_min > 0 && dest_x_max > 0);
    let (x_min, x_max) = ( CalculateExtent(dest_x_max), CalculateExtent(dest_x_min) );

    let mut best_y_extrema = 0;
    for t in 1 .. 1000 {
        let yp_min = y_min.evaluate(t)-5;
        let yp_max = y_max.evaluate(t)+5;
        println!("for timestep {} trying {} -> {}",t,yp_min,yp_max);
        for y in yp_min ..= yp_max {
            let xp_min = x_min.evaluate(t)-5;
            let xp_max = x_max.evaluate(t)+5;
            //println!(" - for timestep {} y {} trying {} -> {}",t,y,xp_min,xp_max);
            for x in xp_min ..= xp_max {
                //println!("for {} trying {}, {}",t,x,y);
                if x <= 0 {
                    break;
                }
                let mut y_extrema = 0;
                let mut y_cur = 0;
                let mut y_vel = y;

                let mut x_cur = 0;
                let mut x_vel = x;

                for i in 0 ..= t {
                    x_cur += x_vel;
                    y_cur += y_vel;

                    if x_vel > 0 {
                        x_vel -= 1;
                    } else if x_vel < 0 {
                        unreachable!();
                    }

                    y_vel -= 1;

                    if y_cur > y_extrema {
                        y_extrema = y_cur;
                    }

                    /*if y_cur >= dest_y_min && y_cur <= dest_y_max && x_cur >= dest_x_min && x_cur <= dest_x_max {
                        break
                    }*/
                }

                if y_cur >= dest_y_min && y_cur <= dest_y_max && x_cur >= dest_x_min && x_cur <= dest_x_max {
                    println!("{} {} at step {} works: {} {}, capping at {}",x,y,t,x_cur,y_cur,y_extrema);
                    if y_extrema > best_y_extrema {
                        best_y_extrema = y_extrema;
                    }
                }
            }
        }
    }
    println!("best y: {:?}",best_y_extrema);

    Ok( () )
}