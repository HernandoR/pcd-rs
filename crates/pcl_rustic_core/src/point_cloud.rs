use crate::point::Point;

pub trait PointCloud {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;

    fn mutable_points(&mut self) -> &mut [Point];

    fn at(&self, index: usize) -> Option<&Point> {
        self.points().get(index)
    }

    fn at_mut(&mut self, index: usize) -> Option<&mut Point> {
        self.mutable_points().get_mut(index)
    }

    fn set(&mut self, index: usize, point: Point) -> Result<(), String> {
        if let Some(p) = self.at_mut(index) {
            *p = point;
            Ok(())
        } else {
            Err(format!("Index {} out of bounds", index))
        }
    }

    fn points(&self) -> &[Point];
    fn add_point(&mut self, point: Point);
    fn num_points(&self) -> usize {
        self.points().len()
    }
    fn is_empty(&self) -> bool {
        self.num_points() == 0
    }
    fn clear(&mut self);
    fn reserve(&mut self, additional: usize);

    fn is_3d(&self) -> bool;
    fn has_color(&self) -> bool;
    fn has_intensity(&self) -> bool;
    fn has_attribute(&self, attribute: &str) -> bool;

    fn transform(&pa, &a2b: &[[f32; 4]; 4]) -> Self;
    fn transform_inplace(&mut pa, &a2b: &[[f32; 4]; 4]);

}
