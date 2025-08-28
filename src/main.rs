fn main() {
    let aspect_ratio = 16.0 / 9.0_f32;
    let image_width = 400;
    let camera = Camera::new(aspect_ratio, image_width);

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let image = camera.render(&world);

    let ppm = to_ppm(&image);
    std::fs::write("output.ppm", ppm).unwrap();
}

fn to_ppm(image: &Vec<Vec<Color3>>) -> String {
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

    fn dot(u: &Self, v: &Self) -> f32 {
        u.0 * v.0 + u.1 * v.1 + u.2 * v.2
    }

    fn self_dot(&self) -> f32 {
        Self::dot(self, self)
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3(0.0, 0.0, 0.0)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl std::ops::Add<f32> for Vec3 {
    type Output = Self;

    fn add(self, scalar: f32) -> Self {
        Vec3(self.0 + scalar, self.1 + scalar, self.2 + scalar)
    }
}

impl std::ops::Add<Vec3> for f32 {
    type Output = Vec3;

    fn add(self, vec: Vec3) -> Vec3 {
        Vec3(vec.0 + self, vec.1 + self, vec.2 + self)
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

    fn at(&self, t: f32) -> Point3 {
        self.orig + t * self.dir
    }
}

struct Interval {
    min: f32,
    max: f32,
}

impl Interval {
    fn new(min: f32, max: f32) -> Self {
        Interval { min, max }
    }

    fn surrounds(&self, value: f32) -> bool {
        self.min < value && value < self.max
    }
}

impl Default for Interval {
    fn default() -> Self {
        EMPTY_INTERVAL
    }
}

const EMPTY_INTERVAL: Interval = Interval {
    min: f32::MAX,
    max: f32::MIN,
};

struct HitRecord {
    #[allow(dead_code)]
    p: Point3,
    normal: Vec3,
    t: f32,
}

impl HitRecord {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        let front_face = Vec3::dot(&r.dir, outward_normal) < 0.0;
        self.normal = if front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

trait Hittable {
    fn hit(&self, r: &Ray, interval: Interval) -> Option<HitRecord>;
}

struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    fn new(center: Point3, radius: f32) -> Self {
        let radius = f32::max(0.0, radius);
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, interval: Interval) -> Option<HitRecord> {
        let oc = self.center - r.orig;
        let a = r.dir.self_dot();
        let h = Vec3::dot(&r.dir, &oc);
        let c = oc.self_dot() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !interval.surrounds(root) {
            root = (h + sqrtd) / a;
            if !interval.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let p = r.at(t);
        let normal = (p - self.center) / self.radius;
        let mut rec = HitRecord { p, normal, t };
        rec.set_face_normal(r, &normal);
        Some(rec)
    }
}

struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    fn new() -> Self {
        HittableList { objects: vec![] }
    }

    fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, interval: Interval) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = interval.max;

        for object in &self.objects {
            if let Some(rec) = object.hit(r, Interval::new(interval.min, closest_so_far)) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }

        temp_rec
    }
}

struct Camera {
    #[allow(dead_code)]
    aspect_ratio: f32,
    image_width: usize,
    image_height: usize,
    center: Point3,
    lower_left_corner: Point3,
    pixel_du: Vec3,
    pixel_dv: Vec3,
}

impl Camera {
    fn new(aspect_ratio: f32, image_width: usize) -> Self {
        let image_height = (image_width as f32 / aspect_ratio) as usize;

        let center = Point3::new(0.0, 0.0, 0.0);

        // Viewport dimensions
        let focal_length = 1.0_f32;
        let viewport_height = 2.0_f32;
        let viewport_width = viewport_height * image_width as f32 / image_height as f32;

        // Vectors accross the viewport
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // delta vectors
        let pixel_du = viewport_u / image_width as f32;
        let pixel_dv = viewport_v / image_height as f32;

        // Lower left corner of the viewport
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);
        let lower_left_corner = viewport_upper_left + (pixel_du / 2.0) + (pixel_dv / 2.0);

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            center,
            lower_left_corner,
            pixel_du,
            pixel_dv,
        }
    }

    fn render<T: Hittable>(&self, world: &T) -> Vec<Vec<Color3>> {
        let mut image = vec![vec![Color3::default(); self.image_width]; self.image_height];
        (0..self.image_height).for_each(|j| {
            (0..self.image_width).for_each(|i| {
                let (u, v) = (i as f32, j as f32);
                let pixel_center = self.lower_left_corner + u * self.pixel_du + v * self.pixel_dv;
                let ray = Ray::new(self.center, pixel_center - self.center);
                image[j][i] = self.ray_color(&ray, world)
            });
        });
        image
    }

    fn ray_color<T: Hittable>(&self, r: &Ray, world: &T) -> Color3 {
        if let Some(rec) = world.hit(r, Interval::new(0.0, f32::MAX)) {
            return 0.5 * (Color3::new(1.0, 1.0, 1.0) + rec.normal);
        }

        let unit_dir = r.dir.normalize();
        let t = 0.5 * (unit_dir.1 + 1.0);
        (1.0 - t) * Color3::new(1.0, 1.0, 1.0) + t * Color3::new(0.5, 0.7, 1.0)
    }
}
