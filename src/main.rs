fn main() {
    image_generation();

    let aspect_ratio = 16.0 / 9.0_f32;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as usize;

    // Camera
    let focal_length = 1.0_f32;
    let viewport_height = 2.0_f32;
    let viewport_width = viewport_height * image_width as f32 / image_height as f32;
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Vectors accross the viewport
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // delta vectors
    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    // Upper left corner of the viewport
    let focal_vec = Vec3::new(0.0, 0.0, focal_length);
    let viewport_upper_left = camera_center - (viewport_u / 2.0) - (viewport_v / 2.0) - focal_vec;
    let pixel_origin = viewport_upper_left + (pixel_delta_u / 2.0) + (pixel_delta_v / 2.0);

    // Image generation
    let mut image = vec![vec![Color3::default(); image_width]; image_height];
    (0..image_height).for_each(|j| {
        (0..image_width).for_each(|i| {
            let pixel_center =
                pixel_origin + (pixel_delta_u * i as f32) + (pixel_delta_v * j as f32);
            let ray_dir = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_dir);
            image[j][i] = ray.color();
        });
    });

    let ppm = get_ppm(&image);
    std::fs::write("output.ppm", ppm).unwrap();
}

fn image_generation() {
    const IMG_WIDTH: usize = 256;
    const IMG_HEIGHT: usize = 256;
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

#[derive(Clone, Copy, Debug)]
struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3(x, y, z)
    }

    fn normalize(&self) -> Self {
        let mag = (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt();
        Vec3(self.0 / mag, self.1 / mag, self.2 / mag)
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3(0.0, 0.0, 0.0)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Vec3(self.0 * scalar, self.1 * scalar, self.2 * scalar)
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3(vec.0 * self, vec.1 * self, vec.2 * self)
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Vec3(self.0 / scalar, self.1 / scalar, self.2 / scalar)
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

    #[allow(dead_code)]
    fn at(&self, t: f32) -> Point3 {
        Point3::new(
            self.orig.0 + t * self.dir.0,
            self.orig.1 + t * self.dir.1,
            self.orig.2 + t * self.dir.2,
        )
    }

    fn color(&self) -> Color3 {
        let unit_dir = self.dir.normalize();
        let t = 0.5 * (unit_dir.1 + 1.0);
        (1.0 - t) * Color3::new(1.0, 1.0, 1.0) + t * Color3::new(0.5, 0.7, 1.0)
    }
}
