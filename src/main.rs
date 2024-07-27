use std::{ops, thread::sleep, time::Duration};

#[derive(Clone, Copy)]
pub struct Point3d {
    pub x : f32,
    pub y : f32,
    pub z : f32
}

impl Point3d {
    pub fn new(x : f32, y : f32, z : f32) -> Self {
        Self {
            x,y,z
        }
    }

    pub fn dot(self, point : Point3d) -> f32{
        (self.x * point.x) + (self.y * point.y) + (self.z * point.z) 
    }
}

#[derive(Copy, Clone)]
pub enum RotationAxis {
    X,Y,Z
}

#[derive(Clone, Copy)]
pub struct Rotation {
    pub angle : f32,
    pub axis : RotationAxis
}

impl Rotation {
    pub fn new(angle : f32, axis : RotationAxis) -> Self {
        Self {
            angle, axis
        }
    }
}

impl ops::Mul<Point3d> for Rotation {
    type Output = Point3d;

    fn mul(self, point : Point3d) -> Point3d {
        let cos = self.angle.cos();   
        let sin = self.angle.sin();

        match self.axis {
            RotationAxis::X => Point3d::new(    
                point.x,
                (point.y * cos) + (point.z * -sin),
                (point.y * sin) + (point.z * cos)
            ),
            RotationAxis::Y => Point3d::new(
                (point.x * cos) + (point.z * sin),
                point.y,
                -(point.x * sin) + (point.z * cos)
            ),
            RotationAxis::Z => Point3d::new(
                (point.x * cos) + (point.y * -sin),
                (point.x * sin) + (point.y * cos),
                point.z,
            ),
        }   
    }
}


pub fn project(point : Point3d, distance : f32)  -> Point3d {
    return Point3d::new(point.x / (distance - point.z), point.y / (distance - point.z), point.z);
}

pub fn toroid(outer_r : f32, inner_r : f32) -> Vec<(Point3d, Point3d)>{
    
    let mut points : Vec<(Point3d,Point3d)> = Vec::new();

    for theta in 0..628 { 
        let theta_cos = (theta as f32 / 10.0).cos();
        let theta_sin = (theta as f32 / 10.0).sin();
        
        for phi in 0..628 {
            let current_rotation = Rotation::new(phi as f32 / 10.0, RotationAxis::Y);
            
            let point = current_rotation * Point3d::new(outer_r + (inner_r * theta_cos), inner_r * theta_sin, 0.0);

            let normal = current_rotation * Point3d::new(theta_cos, theta_sin, 0.0);
            
            points.push((point, normal));
        }   
    }

    points
}

fn main(){
    const WIDTH : usize = 150;
    const HEIGHT : usize = 80;
    let outer_r = 3.0;
    let inner_r = 1.8;
 
    let mut screen_output = String::new();
    let mut draw_buff = [[" "; WIDTH]; HEIGHT];
    let mut z_buffer = [[0.0; WIDTH]; HEIGHT];
    let k = 10.0;

    let size = (WIDTH as f32 * k) / (8.0 * (inner_r + outer_r));

    let mut angle_x = 0.0;
    let mut angle_y = 0.0;
    let mut angle_z = 0.0;

    let luminance = [".",",","-","~",":",";","=","!","*","#","$","@"];

    let light_origin = Point3d::new(0.0, 1.0, -1.0);
    let mut toroid = toroid(outer_r,inner_r);

    loop {
        let rotation_x = Rotation::new(angle_x, RotationAxis::X);
        let rotation_y = Rotation::new(angle_y, RotationAxis::Y);
        let rotation_z = Rotation::new(angle_z, RotationAxis::Z);
        
        for (point, normal) in toroid.iter_mut() {

            let rotated_point = rotation_z * (rotation_x * (rotation_y * *point));
            let rotated_normal = rotation_z * (rotation_x * (rotation_y * *normal));

            let depth = 1.0 / (rotated_point.z + k);

            let screen_x = WIDTH as f32 / 2.0 + rotated_point.x * 2.0 * size * depth;
            let screen_y = (HEIGHT as f32 / 2.0 + 1.0) - rotated_point.y * size * depth;
            
            //Out of screen bounds
            if screen_x > WIDTH as f32 || screen_x < 0.0 || screen_y > HEIGHT as f32|| screen_y < 0.0 {
                continue;
            }
            
            //Ignored by depth buffer
            if z_buffer[screen_y as usize][screen_x as usize] > depth {
                continue;
            }
            
            z_buffer[screen_y as usize][screen_x as usize] = depth;

            let l_level = rotated_normal.dot(light_origin) * 8.0;

            draw_buff[screen_y as usize][screen_x as usize] = luminance[if l_level > 0.0 { l_level as usize } else { 0 }];
        }
    
        
        for h in 0..HEIGHT - 1 {
            for w in 0..WIDTH - 1 {
                screen_output.push_str(draw_buff[h][w]);
            }
            screen_output.push('\n');
        }
    
        println!("{}", screen_output);
        println!("{}",screen_output);
        sleep(Duration::from_millis(100));
        print!("\x1b[H");
        angle_x += 0.1;
        angle_y += 0.1;
        angle_z += 0.1;
        draw_buff = [[" "; WIDTH]; HEIGHT];
        z_buffer = [[0.0_f32; WIDTH]; HEIGHT];
    }

}