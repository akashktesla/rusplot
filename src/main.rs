#![allow(warnings)]
use opencv::{
    prelude::*,
    core::{VecN,Mat,Point_, CV_8U,Rect},
    imgcodecs::{imread,IMREAD_COLOR,IMREAD_GRAYSCALE}, highgui, 
    imgproc::{arrowed_line,rectangle,line, LINE_8},
};

pub fn main(){
    let x = vec![1.,2.,3.,4.,5.,6.,7.,8.,9.,10.];
    let y = vec![5.,2.,6.,1.,2.,3.,5.,9.,2.,1.];
    let plot_img = plot(x,y);
    loop{
        highgui::named_window("plot", 2);
        highgui::imshow("plot", &plot_img);
        highgui::wait_key(0);
    }

}

fn plot(x:Vec<f64>,y:Vec<f64>)->Mat{
    let size = (768,1366);
    let mut _plot = Mat::eye(size.0,size.1,CV_8U).unwrap().to_mat().unwrap();
    let color = VecN::from([255.,0.,0.,1.],);
    arrowed_line(&mut _plot, Point_::new(20,748),Point_::new(20,20),color,2,1,0,0.02);
    arrowed_line(&mut _plot, Point_::new(20,748),Point_::new(1346,748),color,2,1,0,0.01);
    let origin = Point_::new(20,748);
    // let rect = Rect::new(20,20,1326,728);
    // rectangle(&mut _plot, rect, color, 1, 1, 0);
    let _x = normalization_minmax(&x,1326);
    let _y = normalization_minmax(&y,728);
    let scalar = 4.;
    println!("{:?}",_x);
    println!("{:?}",_y);
    let _len = _x.len();
    let color = VecN::from([255.,0.,0.,1.]);
    for i in 1.._len{
        let pt1 = Point_::new(_x[i-1]+20,748-_y[i-1]);
        let pt2 = Point_::new(_x[i]+20,748-_y[i]);
        line(&mut _plot, pt1,pt2,color, 1, LINE_8, 0);
    }
    _plot

}


fn normalization_minmax(data:&Vec<f64>,scalar:i32)->Vec<i32>{
    let min = data.into_iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max = data.into_iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let mut returns = Vec::new();
    for i in data{
        returns.push((((i-min)/(max-min)) *scalar as f64)as i32)
    }
    returns
}
