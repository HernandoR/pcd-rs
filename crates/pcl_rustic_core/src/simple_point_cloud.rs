use crate::point::Point;
use crate::point_cloud::PointCloud;

pub struct SimplePointCloud {
    points: Vec<Point>,
}

impl SimplePointCloud {
    pub fn points_vec(&self) -> &Vec<Point> {
        &self.points
    }
}

impl PointCloud for SimplePointCloud {
    fn new() -> Self {
        SimplePointCloud { points: Vec::new() }
    }

    fn with_capacity(capacity: usize) -> Self {
        SimplePointCloud {
            points: Vec::with_capacity(capacity),
        }
    }

    fn mutable_points(&mut self) -> &mut [Point] {
        &mut self.points
    }

    fn points(&self) -> &[Point] {
        &self.points
    }

    fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }

    fn clear(&mut self) {
        self.points.clear();
    }

    fn reserve(&mut self, additional: usize) {
        self.points.reserve(additional);
    }

    fn is_3d(&self) -> bool {
        self.points.iter().any(|p| p.is_3d())
    }

    fn has_color(&self) -> bool {
        self.points.iter().any(|p| p.has_color())
    }

    fn has_intensity(&self) -> bool {
        self.points.iter().any(|p| p.intensity.is_some())
    }

    fn has_attribute(&self, attribute: &str) -> bool {
        match attribute {
            "x" | "y" => true,
            "z" => self.is_3d(),
            "r" | "g" | "b" | "a" => self.has_color(),
            "intensity" => self.has_intensity(),
            "ring_id" => self.points.iter().any(|p| p.ring_id.is_some()),
            "time_offset" => self.points.iter().any(|p| p.time_offset.is_some()),
            _ => false,
        }
    }

    fn transform(self, a2b: &arr2<f32; 4, 4>) -> Self {
        let mut out = Self::with_capacity(self.num_points());
        for p in self.points {
            let np = p.transform(a2b);
            out.add_point(np);
        }
        out
    }

    fn transform_inplace(&mut self, a2b: &arr2<f32; 4, 4>) {
        for p in &mut self.points {
            let np = p.transform(a2b);
            *p = np;
        }
    }
}