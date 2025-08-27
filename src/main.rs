const IMG_WIDTH: usize = 256;
const IMG_HEIGHT: usize = 256;

fn main() {
    println!("Hello, world!");

    let mut image = vec![vec![Color3::default(); IMG_WIDTH]; IMG_HEIGHT];
    (0..IMG_HEIGHT).for_each(|y| {
        (0..IMG_WIDTH).for_each(|x| {
            let r = x as f32 / (IMG_WIDTH - 1) as f32;
            let g = y as f32 / (IMG_HEIGHT - 1) as f32;
            let b = 0.0;
            image[y][x] = Color3::new(r, g, b);
        });
    });

    let ppm = get_ppm(&image);
    std::fs::write("output.ppm", ppm).unwrap();
}

fn get_ppm(image: &Vec<Vec<Color3>>) -> String {
    let mut ppm = String::new();
    ppm.push_str("P3\n");
    ppm.push_str(&format!("{} {}\n", image[0].len(), image.len()));
    ppm.push_str("255\n");
    for row in image {
        for color in row {
            let r = (color.0 * 255.999) as u8;
            let g = (color.1 * 255.999) as u8;
            let b = (color.2 * 255.999) as u8;
            ppm.push_str(&format!("{} {} {} ", r, g, b));
        }
        ppm.push('\n');
    }
    ppm
}

#[derive(Clone, Debug)]
struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3(x, y, z)
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3(0.0, 0.0, 0.0)
    }
}

type Color3 = Vec3;
type Point3 = Vec3;

struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    fn new(orig: Point3, dir: Vec3) -> Self {
        Ray { orig, dir }
    }

    fn at(&self, t: f32) -> Point3 {
        Point3::new(
            self.orig.0 + t * self.dir.0,
            self.orig.1 + t * self.dir.1,
            self.orig.2 + t * self.dir.2,
        )
    }
}
