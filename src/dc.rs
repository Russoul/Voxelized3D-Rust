use std;
use na::*;
use math::*;

fn calc_qef(point : &Vector3<f32>, planes : &Vec<Plane<f32>>) -> f32{
    let mut qef : f32 = 0.0;
    for plane in planes{
        let dist = distance_point3_plane(point, plane);
        qef += dist * dist;
    }

    qef
}

fn const_sign(a : f32, b : f32) -> bool {
    if a > 0.0 { b > 0.0} else {b <= 0.0}
}


fn sample_qef_brute(square : Square3<f32>, n : usize, planes : &Vec<Plane<f32>>) -> Vector3<f32> {
    let ext = Vector3::new(square.extent, square.extent, square.extent);
    let min = square.center - ext;

    let mut best_qef = std::f32::MAX;
    let mut best_point = min;

    for i in 0..n{
        for j in 0..n{
            for k in 0..n{
                let point = min + Vector3::new(ext.x * (2.0 * (i as f32) + 1.0) / (n as f32),
                                               ext.y * (2.0 * (j as f32) + 1.0) / (n as f32),
                                               ext.z * (2.0 * (k as f32) + 1.0) / (n as f32));
                let qef = calc_qef(&point, &planes);

                if qef < best_qef{
                    best_qef = qef;
                    best_point = point;
                }
            }
        }
    }

    best_point
}

fn sample_intersection_brute(line : Line3<f32>, n : usize, f : &DenFn3<f32>) -> Vector3<f32>{
    let ext = line.end - line.start;

    let mut best_abs = std::f32::MAX;
    let mut best_point : Option<Vector3<f32>> = None;

    for i in 0..n {
        let point = line.start + ext * ((i as f32 + 0.5) / n as f32);
        let den = f(point);
        let abs = den.abs();

        if abs < best_abs {
            best_abs = abs;
            best_point = Some(point);
        }
    }

    best_point.unwrap()
}


pub fn sample_normal(sphere : &Sphere<f32>, n : usize, f : &DenFn3<f32>) -> Vector3<f32>{

    let den_at_center = f(sphere.center);

    let mut best = std::f32::MIN;
    let mut normal_point = sphere.center;

    let slice2_ = std::f32::consts::PI / n as f32;
    let slice1_ = slice2_ * 2.0;


    for i in 0..n{

        let slice1 = slice1_ * (i as f32);

        for j in 0..n{
            let slice2 = slice2_ * (j as f32);
            let y = slice2.cos() * sphere.rad;
            let x = slice1.cos() * slice2.sin().abs() * sphere.rad;
            let z = -slice1.sin() * slice2.sin().abs() * sphere.rad;

            let point = sphere.center + Vector3::new(x,y,z);
            let den = f(point);
            let attempt = (den - den_at_center);
            if attempt > best{
                best = attempt;
                normal_point = point;
            }
        }
    }



    (normal_point - sphere.center).normalize()
}

pub fn test_sample_normal(){
    let test_sph = Sphere{center : Vector3::new(0.0, 0.0, 0.0), rad : 1.0};
    let test_point = Sphere{center : Vector3::new(0.0, 0.0, 1.0), rad : 0.01};
    let test_solid = mk_sphere(test_sph);
    let res = sample_normal(&test_point, 100, &test_solid);

    println!("{}", res); //result should approach {0.0,0.0,1.0} increasing accuracy
}