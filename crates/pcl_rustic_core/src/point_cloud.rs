use crate::point::Point;

/// PointCloud trait 抽象了点云容器的基本操作。
///
/// 约定：实现该 trait 的容器中存储的点被视为位于参考坐标系 "pa"。
/// 方法 `transform` 和 `transform_inplace` 接受一个 4x4 齐次变换矩阵 `a2b`，
/// 该矩阵将点从坐标系 pa 映射到坐标系 "pb"。也就是说，容器内的点 p（在 pa 中）
/// 通过 a2b 变换后得到对应在 pb 中的点。
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
    fn has_classification(&self) -> bool;
    fn has_intensity(&self) -> bool;
    fn has_attribute(&self, attribute: &str) -> bool;
    fn attribute_names(&self) -> Vec<String>;

    fn transform(&self, a2b: &[[f32; 4]; 4]) -> Self;
    fn transform_inplace(&mut self, a2b: &[[f32; 4]; 4]);
}
