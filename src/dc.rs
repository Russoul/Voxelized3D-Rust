use std;
use na::*;
use math::*;
use renderer::*;
use alga::general::SupersetOf;

pub struct VoxelGrid3<T : Real + Copy>{
    pub a : T,
    pub size_x : usize,
    pub size_y : usize,
    pub size_z : usize,
    pub grid : Vec<T>,
}


impl<T : Real + SupersetOf<f32>> VoxelGrid3<T>{

    pub fn vertices_x(&self) -> usize {self.size_x + 1}
    pub fn vertices_y(&self) -> usize {self.size_y + 1}
    pub fn vertices_z(&self) -> usize {self.size_z + 1}

    pub fn new(a : T, size_x : usize, size_y : usize, size_z : usize) -> VoxelGrid3<T>{
        let grid = vec![convert(0.0);(size_x + 1) * (size_y + 1) * (size_z + 1)];

        VoxelGrid3{a,size_x, size_y, size_z, grid}
    }

    pub fn get(&self, x : usize, y : usize, z : usize) -> T{
        self.grid[z * self.vertices_y() * self.vertices_x() + y * self.vertices_x() + x]
    }

    pub fn set(&mut self, x : usize, y : usize, z : usize, value : T){
        let vx = self.vertices_x();
        let vy = self.vertices_y();
        self.grid[z * vy * vx + y * vx + x] = value;
    }


    pub fn get_point(&self, x : usize, y : usize, z : usize) -> Vector3<T>{
        Vector3::new(self.a * convert::<f32, T>(x as f32), self.a * convert::<f32, T>(y as f32), self.a * convert::<f32, T>(z as f32))
    }

    //bounding box of the cube
    pub fn square3(&self, x : usize, y : usize, z : usize) -> Square3<T>{
        Square3{center : Vector3::new(convert::<f32,T>(x as f32 + 0.5) * self.a, convert::<f32,T>(y as f32 + 0.5) * self.a, convert::<f32,T>(z as f32 + 0.5) * self.a), extent: self.a / convert(2.0)}
    }
}

fn calc_qef(point : &Vector3<f32>, planes : &Vec<Plane<f32>>) -> f32{
    let mut qef : f32 = 0.0;
    for plane in planes{
        let dist_signed = plane.normal.dot(&(point - plane.point));
        qef += dist_signed * dist_signed;
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
fn sample_intersection_brute(line : Line3<f32>, n_ : usize, f : &DenFn3<f32>) -> Vector3<f32>{
    let ext = line.end - line.start;
    let norm = ext.norm();
    let dir = ext / norm;

    //let mut best_abs = std::f32::MAX;
    //let mut best_point : Option<Vector3<f32>> = None;

    let mut center = line.start + ext * 0.5;
    let mut cur_ext = norm * 0.25;
    
    let n = (n_ as f32).log2() as usize + 1;

    for i in 0..n {
        let point1 = center - dir * cur_ext;
        let point2 = center + dir * cur_ext;
        let den1 = f(point1).abs();
        let den2 = f(point2).abs();

        if den1 <= den2 {
            center = point1;
        }else{
            center = point2;
        }
        cur_ext *= 0.5;
    }

    center
}


//why haven't I come up with this one at the start ? :)
pub fn sample_normal(point : &Vector3<f32>, eps : f32, f : &DenFn3<f32>) -> Vector3<f32>{
    Vector3::new( f(Vector3::new(point.x + eps, point.y, point.z)) - f(Vector3::new(point.x - eps, point.y, point.z)),
                  f(Vector3::new(point.x, point.y + eps, point.z)) - f(Vector3::new(point.x, point.y - eps, point.z)),
                  f(Vector3::new(point.x, point.y, point.z + eps)) - f(Vector3::new(point.x, point.y, point.z - eps)) ).normalize()
}

//works not so well
pub fn sample_normal1(sphere : &Sphere<f32>, n : usize, f : &DenFn3<f32>) -> Vector3<f32>{

    let den_at_center = f(sphere.center);

    let mut best = 0.0;
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
            let attempt = den - den_at_center;
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
    let res = sample_normal(&test_point.center, 0.001, &test_solid);

    println!("{}", res); //result should approach {0.0,0.0,1.0} increasing accuracy
}

//voxel grid is an array like structure (in the feature it should be upgraded to an octree) that contains density information at each vertex of each cube of the grid

//feature is a vertex that may or may not be calculated for each cube of the grid. It is calculated for each cube that exhibits a sign change(this means that the cube
// intersects the surface) and not calculated otherwise
fn calc_feature(vg : &VoxelGrid3<f32>, x : usize, y : usize, z : usize,
               f : &DenFn3<f32>, accuracy : usize, contour_data : &mut ContourData, debug_render : &mut RendererVertFragDef) -> Option<Vector3<f32>>{
    //let epsilon = vg.a / accuracy as f32;

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

        //let mut normals = Vec::<f32>::new();
        //let mut intersections = Vec::<f32>::new();
        let mut planes = Vec::new();


        {
            let mut worker = |edge_id : usize, v_a : Vector3<f32>, v_b : Vector3<f32>, p_a : f32, p_b : f32|{//goes through each edge of the cube
                if (edge_info & edge_id) > 0{
                    let ip = sample_intersection_brute(Line3{start : v_a, end : v_b}, accuracy, f);//intersecion point
                    //let full = if p_a <= 0.0 {v_a} else {v_b};
                    //let normal = sample_normal(&Sphere{center : ip, rad : rad_for_normal}, accuracy, f);
                    let normal = sample_normal(&ip, rad_for_normal, f);
                    //intersections.push(ip.x);
                    //intersections.push(ip.y);
                    //intersections.push(ip.z);
                    //normals.push(normal.x);
                    //normals.push(normal.y);
                    //normals.push(normal.z);
                    planes.push(Plane{point : ip, normal});
                    //calculate feature vertices of 3 other cubes containing this edge then create a quad from maximum of 4 those feature vertices.
                    //this is done in make_contour
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

       /*  let mut product = Vec::with_capacity(normals.len());
        for i in 0..normals.len()/3{
            product.push(normals[3 * i] * intersections[3 * i] + normals[3 * i + 1] * intersections[3 * i + 1] + normals[3 * i + 2] * intersections[3 * i + 2]);
        }

        //let feature_vertex = Vector3::new(0.0,0.0,0.0);//sample_qef_brute(vg.square3(x,y,z), accuracy, &normals.zip);//TODO
        let A = DMatrix::from_row_slice(normals.len() / 3, 3, normals.as_slice());
        let ATA = (&A).transpose() * &A;
        let b = DMatrix::from_row_slice(product.len(), 1, product.as_slice());
        let ATb = (&A).transpose() * &b; */

        //println!("{:?} {}", normals.as_slice(), A);

        let feature_vertex = sample_qef_brute(vg.square3(x, y, z), accuracy, &planes);
                

        let t = z * vg.size_y * vg.size_x + y * vg.size_x + x;

        contour_data.features[t] = Some(feature_vertex);
        //contour_data.normals[t] = Some(sample_normal(&Sphere{center : feature_vertex, rad : rad_for_normal}, accuracy, f));
        contour_data.normals[t] = Some(sample_normal(&feature_vertex, vg.a / 100.0, f));


        Some(feature_vertex)
        
    }else{
        None
    }
}

//TODO debug_renderer is for debug only
pub fn make_contour(vg : &VoxelGrid3<f32>, f : &DenFn3<f32>, accuracy : usize, debug_renderer : &mut RendererVertFragDef) -> ContourData{

    //TODO inefficient Vec::new() creation vvv
    let mut contour_data = ContourData{lines : Vec::new(), triangles : Vec::new(), triangle_normals : Vec::new(), features : vec![None;vg.size_x * vg.size_y * vg.size_z], normals : vec![None;vg.size_x * vg.size_y * vg.size_z]};
    let mut cache_already_calculated = vec![false;vg.size_x * vg.size_y * vg.size_z]; //this cache is used to mark cubes that have already been calculated for feature vertex

    {
        //&mut contour_data, cache_already_calculated
        let mut cached_make = |x: usize, y: usize, z : usize, contour_data : &mut ContourData| -> Option<Vector3<f32>>{
            let t = z * vg.size_y * vg.size_x + y * vg.size_x + x;

            if cache_already_calculated[t] {
                contour_data.features[t]
            }else{
                cache_already_calculated[t] = true;
                calc_feature(&vg, x, y, z, f, accuracy, contour_data, debug_renderer)
            }


        };

        for z in 0..vg.size_z{
            for y in 0..vg.size_y {
                for x in 0..vg.size_x {
                    //let p00 = vg.get(x, y, z);
                    //let p01 = vg.get(x + 1, y, z);
                    //let p02 = vg.get(x, y + 1, z);
                    let p03 = vg.get(x + 1, y + 1, z);

                    //let p10 = vg.get(x, y, z + 1);
                    let p11 = vg.get(x + 1, y, z + 1);
                    let p12 = vg.get(x, y + 1, z + 1);
                    let p13 = vg.get(x + 1, y + 1, z + 1);

                    /*let v00 = vg.get_point(x, y, z);
                    let v01 = vg.get_point(x + 1, y, z);
                    let v02 = vg.get_point(x, y + 1, z);
                    let v03 = vg.get_point(x + 1, y + 1, z);

                    let v10 = vg.get_point(x,y, z + 1);
                    let v11 = vg.get_point(x + 1, y, z + 1);
                    let v12 = vg.get_point(x, y + 1, z + 1);
                    let v13 = vg.get_point(x + 1, y + 1, z + 1);*/


                    let possible_feature_vertex = cached_make(x, y, z, &mut contour_data);

                    match possible_feature_vertex{
                        None => (),
                        Some(f0) => {
                            let t = z * vg.size_y * vg.size_x + y * vg.size_x + x;
                            let normal = contour_data.normals[t].unwrap();


                            
                            
                            //TODO incorrect normals in some places 
                            if !const_sign(p03, p13){

                                let f1 = cached_make(x + 1, y, z, &mut contour_data).unwrap();
                                let f2 = cached_make(x + 1, y + 1, z, &mut contour_data).unwrap();
                                let f3 = cached_make(x, y + 1, z, &mut contour_data).unwrap();
                                //f1 && f2 && f3 all should be non-empty, as they all exhibit a sign change at least on their common edge

                                //this is needed to calculate the direction of the resulting quad correctly
                                let dir = (f2 - f0).cross(&(f3 - f0)).normalize();
                                
                                if dir.dot(&normal) > 0.0{ //should not be zero at any time
                                    contour_data.triangles.push(Triangle3{p1 : f0, p2 : f2, p3 : f3});
                                    contour_data.triangles.push(Triangle3{p1 : f0, p2 : f1, p3 : f2});
                                    //add_line3_color(debug_renderer, Line3{start : f0, end : f0 + dir * 0.1}, Vector3::new(1.0, 1.0, 1.0));
                                    //TODO debug
                                    /* if (dir.dot(&debug_real_normal) <= 0.0) {
                                        println!("bad normal at {} {} {} {}", 1, x, y, z);
                                    } */
                                    contour_data.triangle_normals.push(dir); //TODO inefficient
                                }else{
                                    contour_data.triangles.push(Triangle3{p1 : f0, p2 : f3, p3 : f2});
                                    contour_data.triangles.push(Triangle3{p1 : f0, p2 : f2, p3 : f1});
                                    //add_line3_color(debug_renderer, Line3{start : f0, end : f0 - dir * 0.1}, Vector3::new(1.0, 1.0, 1.0));
                                   /*  if (-dir.dot(&debug_real_normal) <= 0.0) {
                                        add_square3_bounds_color(debug_renderer, vg.square3(x, y, z), Vector3::new(1.0, 0.0, 0.0));
                                        add_line3_color(debug_renderer, Line3{start : f0, end : f0 - dir * 0.1}, Vector3::new(1.0, 1.0, 1.0));
                                        add_line3_color(debug_renderer, Line3{start : f0, end : f0 - normal.normalize()}, Vector3::new(0.0, 0.0, 0.0));
                                        println!("bad normal at {} {}", 2, -dir.dot(&debug_real_normal));
                                    } */
                                    contour_data.triangle_normals.push(-dir);
                                }
                            }
                            if !const_sign(p12, p13){
                                let f1 = cached_make(x, y, z + 1, &mut contour_data).unwrap();
                                let f2 = cached_make(x, y + 1, z + 1, &mut contour_data).unwrap();
                                let f3 = cached_make(x, y + 1, z, &mut contour_data).unwrap();
                                //f1 && f2 && f3 all should be non-empty, as they all exhibit a sign change at least on their common edge

                                //this is needed to calculate the direction of the resulting quad correctly
                                let dir = (f2 - f0).cross(&(f3 - f0)).normalize();
                                if dir.dot(&normal) > 0.0{ //should not be zero at any time
                                    contour_data.triangles.push(Triangle3{p1 : f0, p2 : f2, p3 : f3});
                                    contour_data.triangles.push(Triangle3{p1 : f0, p2 : f1, p3 : f2});
                                    //add_line3_color(debug_renderer, Line3{start : f0, end : f0 + dir * 0.1}, Vector3::new(1.0, 1.0, 1.0));
                                   /*  if (dir.dot(&debug_real_normal) <= 0.0) {
                                        println!("bad normal at {} {} {} {}", 3, x, y, z);
                                    } */
                                    contour_data.triangle_normals.push(dir);
                                }else{
                                    contour_data.triangles.push(Triangle3{p1 : f0, p2 : f3, p3 : f2});
                                    contour_data.triangles.push(Triangle3{p1 : f0, p2 : f2, p3 : f1});
                                    //add_line3_color(debug_renderer, Line3{start : f0, end : f0 - dir * 0.1}, Vector3::new(1.0, 1.0, 1.0));
                                   /*  if (-dir.dot(&debug_real_normal) <= 0.0) {
                                        add_square3_bounds_color(debug_renderer, vg.square3(x, y, z), Vector3::new(1.0, 0.0, 0.0));
                                        add_line3_color(debug_renderer, Line3{start : f0, end : f0 - dir * 0.1}, Vector3::new(1.0, 1.0, 1.0));
                                        add_line3_color(debug_renderer, Line3{start : f0, end : f0 - normal.normalize()}, Vector3::new(0.0, 0.0, 0.0));
                                        
                                        println!("bad normal at {} {}", 4, -dir.dot(&debug_real_normal));
                                    } */
                                    contour_data.triangle_normals.push(-dir);
                                }
                            }
                            if !const_sign(p11, p13){
                                let f1 = cached_make(x + 1, y, z, &mut contour_data).unwrap();
                                let f2 = cached_make(x + 1, y, z + 1, &mut contour_data).unwrap();
                                let f3 = cached_make(x, y, z + 1, &mut contour_data).unwrap();
                                //f1 && f2 && f3 all should be non-empty, as they all exhibit a sign change at least on their common edge

                                //this is needed to calculate the direction of the resulting quad correctly
                                let dir = (f2 - f0).cross(&(f3 - f0)).normalize();
                                if dir.dot(&normal) > 0.0{ //should not be zero at any time
                                    contour_data.triangles.push(Triangle3{p1 : f0, p2 : f2, p3 : f3});
                                    contour_data.triangles.push(Triangle3{p1 : f0, p2 : f1, p3 : f2});
                                    //add_line3_color(debug_renderer, Line3{start : f0, end : f0 + dir * 0.1}, Vector3::new(1.0, 1.0, 1.0));
                                   /*  if (dir.dot(&debug_real_normal) <= 0.0) {
                                        println!("bad normal at {} {} {} {}", 5, x, y, z);
                                    } */
                                    contour_data.triangle_normals.push(dir);
                                } else{
                                    contour_data.triangles.push(Triangle3{p1 : f0, p2 : f3, p3 : f2});
                                    contour_data.triangles.push(Triangle3{p1 : f0, p2 : f2, p3 : f1});
                                    //add_line3_color(debug_renderer, Line3{start : f0, end : f0 - dir * 0.1}, Vector3::new(1.0, 1.0, 1.0));
                                   /*  if (-dir.dot(&debug_real_normal) <= 0.0) {
                                        add_square3_bounds_color(debug_renderer, vg.square3(x, y, z), Vector3::new(1.0, 0.0, 0.0));
                                        add_line3_color(debug_renderer, Line3{start : f0, end : f0 - dir * 0.1}, Vector3::new(1.0, 1.0, 1.0));
                                        println!("bad normal at {} {} {} {}", 6, x, y, z);
                                    } */
                                    contour_data.triangle_normals.push(-dir);
                                } 
                            }
                        },
                    }



                }
            }
        }

    }

    contour_data

}

pub fn fill_in_grid(vg : &mut VoxelGrid3<f32>, f : &DenFn3<f32>, offset : Vector3<f32>){
    for z in 0..vg.size_z + 1 {
        for y in 0..vg.size_y + 1{
            for x in 0..vg.size_x + 1 {
                let vx = vg.vertices_x();
                let vy = vg.vertices_y();
                vg.grid[z * vy * vx + y * vx + x] = f(offset + Vector3::new(vg.a * (x as f32), vg.a * (y as f32), vg.a * (z as f32)));
            }
        }
    }
}


pub struct ContourData{ // + hermite data ? (exact points of intersection of the surface with each edge that exhibits a sign change + normals for each of those points)
    pub lines : Vec<Line3<f32>>,
    pub triangles : Vec<Triangle3<f32>>,
    pub triangle_normals : Vec<Vector3<f32>>,
    pub features : Vec<Option<Vector3<f32>>>,
    pub normals : Vec<Option<Vector3<f32>>>, //normal to the surface calculated at feature vertex
}