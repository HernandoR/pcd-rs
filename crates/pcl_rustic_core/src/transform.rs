use ndarray::Array2;
use ndarray::LinalgScalar;
use std::ops::Mul;

/// 一个 4x4 齐次变换矩阵类型，表示从坐标系 pa 到坐标系 pb 的变换矩阵。
///
/// 内部保证为 4x4 尺寸；提供构造、单位矩阵、相乘（组合变换）以及将点以齐次坐标变换的方法。
#[derive(Clone, Debug)]
pub struct Transform<T>
where
    T: LinalgScalar,
{
    mat: Array2<T>, // shape must be (4,4)
}

impl Transform {
    /// 使用一个 [[f32;4];4] 数组构造 Transform
    pub fn from(a: [[T; 4]; 4]) -> Self
    where
        T: LinalgScalar,
    {
        // flatten row-major
        let mut v = Vec::with_capacity(16);
        for r in 0..4 {
            for c in 0..4 {
                v.push(a[r][c]);
            }
        }
        let mat = Array2::from_shape_vec((4, 4), v).expect("shape is 4x4");
        Transform { mat }
    }

    /// 使用已有的 Array2<T> 构造，若尺寸不为 4x4 则返回 Err
    pub fn from(m: Array2<T>) -> Result<Self, String>
    where
        T: LinalgScalar,
    {
        let shape = m.dim();
        if shape == (4, 4) {
            Ok(Transform { mat: m })
        } else {
            Err(format!("ndarray must be 4x4, got {:?}", shape))
        }
    }

    /// 返回单位变换
    pub fn identity() -> Self {
        let mut mat = Array2::eye(4);
        Transform { mat }
    }

    /// 以齐次坐标将 (x,y,z) 变换为 (x',y',z')；若输入 z 为 None 则按 0.0 处理（平面点）。
    /// 输出已做 w 分量归一化（若 w 非零）。
    pub fn apply_to_point(&self, x: T, y: T, z: Option<T>) -> (T, T, T)
    where
        T: LinalgScalar,
    {
        let zv = z.unwrap_or(T::zero());
        // compute result = mat @ [x,y,z,1]

        let p_arr = Array2::from_shape_vec((4, 1), vec![x, y, zv, T::one()]).unwrap();
        let res = self.mat.dot(&p_arr);
        let w = res[[3, 0]];
        if w.abs() > T::zero() + T::from(1e-6) {
            (res[[0, 0]] / w, res[[1, 0]] / w, res[[2, 0]] / w)
        } else {
            (res[[0, 0]], res[[1, 0]], res[[2, 0]])
        }
    }

    pub fn apply_to_point(&self, xyz: Array<T>) -> Array<T>
    where
        T: LinalgScalar,
    {
        assert_eq!(xyz.len(), 3);
        let mut vec4 =
            Array2::from_shape_vec((4, 1), vec![xyz[0], xyz[1], xyz[2], T::one()]).unwrap();
        let res = self.mat.dot(&vec4);
        let w = res[[3, 0]];
        if w.abs() > T::zero() + T::from(1e-6) {
            Array2::from_shape_vec(
                (3,),
                vec![res[[0, 0]] / w, res[[1, 0]] / w, res[[2, 0]] / w],
            )
            .unwrap()
        } else {
            Array2::from_shape_vec((3,), vec![res[[0, 0]], res[[1, 0]], res[[2, 0]]]).unwrap()
        }
    }

    /// 返回对内部矩阵的只读引用
    pub fn as_ndarray(&self) -> &Array2<f32> {
        &self.mat
    }
}

impl Default for Transform {
    fn default() -> Self {
        Transform::identity()
    }
}

impl Mul for Transform {
    type Output = Transform;
    /// concate self(a2b) and b2c to get a2c
    fn mul(self, b2c: Transform) -> Transform {
        let out = &b2c.mat.dot(&self.mat);
        Transform { mat: out.clone() }
    }
}
