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


//in the feature density functions will not be present at all times (as the world can be saved to disk, generator-density function is not saved)
//so the algorithm should use some interpolation methods assuming the surface is smooth(does not change too much within one cube of the grid)
//interpolation can operate on 8 corner vertices of the cube
//TODO or maybe save generator to disk ??, in case of random(presudo-random) generator - its seed can be saved
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

//voxel grid is an array like structure (in the feature it should be upgraded to an octree) that contains density information at each vertex of each cube of the grid

//feature is a vertex that may or may not be calculated for each cube of the grid. It is calculated for each cube that exhibits a sign change(this means that the cube
// intersects the surface) and not calculated otherwise
fn calc_feature(vg : &VoxelGrid3<f32>, x : usize, y : usize, z : usize,
               f : &DenFn3<f32>, accuracy : usize, contour_data : &mut ContourData) -> Option<Vector3<f32>>{
    let epsilon = vg.a / accuracy as f32;

    let p00 = vg.get(x, y, z);
    let p01 = vg.get(x + 1, y, z);
    let p02 = vg.get(x, y + 1, z);
    let p03 = vg.get(x + 1, y + 1, z);

    let p10 = vg.get(x, y, z + 1);
    let p11 = vg.get(x + 1, y, z + 1);
    let p12 = vg.get(x, y + 1, z + 1);
    let p13 = vg.get(x + 1, y + 1, z + 1);

    let v00 = vg.get_point(x, y, z);
    let v01 = vg.get_point(x + 1, y, z);
    let v02 = vg.get_point(x, y + 1, z);
    let v03 = vg.get_point(x + 1, y + 1, z);

    let v10 = vg.get_point(x,y, z + 1);
    let v11 = vg.get_point(x + 1, y, z + 1);
    let v12 = vg.get_point(x, y + 1, z + 1);
    let v13 = vg.get_point(x + 1, y + 1, z + 1);

    let mut edge_info = 0;

    if !const_sign(p00, p01){edge_info |= 1;}
    if !const_sign(p01, p03){edge_info |= 2;}
    if !const_sign(p03, p02){edge_info |= 4;} //z
    if !const_sign(p02, p00){edge_info |= 8;}

    if !const_sign(p10, p11){edge_info |= 16;}
    if !const_sign(p11, p13){edge_info |= 32;} //z + 1
    if !const_sign(p13, p12){edge_info |= 64;}
    if !const_sign(p12, p10){edge_info |= 128;}

    if !const_sign(p00, p10){edge_info |= 256;}
    if !const_sign(p01, p11){edge_info |= 512;}
    if !const_sign(p02, p12){edge_info |= 1024;} //edges in between of 2 z-levels
    if !const_sign(p03, p13){edge_info |= 2048;}

    let rad_for_normal = vg.a / 100.0; //TODO will not work if vg.a is too small (f32 precision)

    if edge_info > 0{
        let mut planes = Vec::<Plane<f32>>::new();

        {
            let mut worker = |edge_id : usize, v_a : Vector3<f32>, v_b : Vector3<f32>, p_a : f32, p_b : f32|{//goes through each edge of the cube
                if (edge_info & edge_id) > 0{
                    let ip = sample_intersection_brute(Line3{start : v_a, end : v_b}, accuracy, f);//intersecion point
                    let full = if p_a <= 0.0 {v_a} else {v_b};
                    let normal = sample_normal(&Sphere{center : ip, rad : rad_for_normal}, accuracy, f);
                    planes.push(Plane{point : ip, normal});

                    //TODO calculate feature vertices of 3 other cubes containing this edge then create a quad from maximum of 4 those feature vertices.
                }
            };

            worker(1, v00, v01, p00, p01);
            worker(2, v01, v03, p01, p03);
            worker(4, v03, v02, p03, p02);
            worker(8, v02, v00, p02, p00);

            worker(16, v10, v11, p10, p11);
            worker(32, v11, v13, p11, p13);
            worker(64, v13, v12, p13, p12);
            worker(128, v12, v10, p12, p10);

            worker(256, v00, v10, p00, p10);
            worker(512, v01, v11, p01, p11);
            worker(1024, v02, v12, p02, p12);
            worker(2048, v03, v13, p03, p13);
        }

        let feature_vertex = sample_qef_brute(vg.square3(x,y,z), accuracy, &planes);


        contour_data.features[z * vg.size_y * vg.size_x + y * vg.size_x + x] = Some(feature_vertex);

        Some(feature_vertex)
    }else{
        None
    }
}


struct ContourData{ // + hermite data ? (exact points of intersection of the surface with each edge that exhibits a sign change + normals for each of those points)
    pub lines : Vec<Line3<f32>>,
    pub triangles : Vec<Triangle3<f32>>,
    pub features : Vec<Option<Vector3<f32>>>,
}