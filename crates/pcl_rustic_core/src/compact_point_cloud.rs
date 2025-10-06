extern crate ndarray;
use crate::Point;
use crate::PointCloud;
use ndarray::prelude::*;

pub struct CompactPointCloud<Txy = f32, Ti = f32, Trgb = u8, Tc = f32, Textra = f32>
where
    Txy: ndarray::LinalgScalar,
    Ti: ndarray::LinalgScalar,
    Trgb: ndarray::LinalgScalar,
    Tc: ndarray::LinalgScalar,
    Textra: ndarray::LinalgScalar,
{
    positions: Array2<Txy>,              // shape (N,3) or (N,2)
    colors: Option<Array2<Trgb>>,        // shape (N,3) or shape (N,4) or None
    intensities: Option<Array2<Ti>>,     // shape (N,1) or None
    classifications: Option<Array2<Tc>>, // shape (N,1) or None
    extra_attributes: Option<std::collections::HashMap<String, Array2<Textra>>>, // shape (N,1) for each attribute
    _capacity: usize,
    _is_auto_expand_capacity: bool,
}

impl Default for CompactPointCloud {
    fn default() -> Self {
        Self {
            positions: Array2::zeros((0, 3)),
            colors: None,
            intensities: None,
            classifications: None,
            extra_attributes: None,
            _capacity: 0,
            _is_auto_expand_capacity: true,
        }
    }
}

impl CompactPointCloud {
    pub fn len(&self) -> usize {
        self.positions.len_of(Axis(0))
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn num_points(&self) -> usize {
        self.len()
    }

    pub fn capacity(&self) -> usize {
        self._capacity
    }

    pub fn _auto_expand_capacity(&mut self) {
        if !self._is_auto_expand_capacity {
            return;
        }
        let current_len = self.len();
        if self._capacity == 0 {
            self.reserve(16);
        } else if current_len >= self._capacity {
            self.reserve(self._capacity); // double the capacity
        }
    }

    pub fn is_valid(&self) -> bool {
        let n = self.len();
        if self.positions.len_of(Axis(1)) != 2 && self.positions.len_of(Axis(1)) != 3 {
            return false;
        }
        if let Some(ref colors) = self.colors {
            let c_dim = colors.len_of(Axis(1));
            if c_dim != 3 && c_dim != 4 {
                return false;
            }
            if colors.len_of(Axis(0)) != n {
                return false;
            }
        }
        if let Some(ref intensities) = self.intensities {
            if intensities.len_of(Axis(0)) != n || intensities.len_of(Axis(1)) != 1 {
                return false;
            }
        }
        if let Some(ref classifications) = self.classifications {
            if classifications.len_of(Axis(0)) != n || classifications.len_of(Axis(1)) != 1 {
                return false;
            }
        }
        if let Some(ref extra_attrs) = self.extra_attributes {
            for (_key, attr_array) in extra_attrs.iter() {
                if attr_array.len_of(Axis(0)) != n || attr_array.len_of(Axis(1)) != 1 {
                    return false;
                }
            }
        }
        true
    }
}

impl PointCloud for CompactPointCloud {
    fn new() -> Self {
        Self::default()
    }

    // `fn has_classification`, `fn has_intensity`, `fn has_attribute`, `fn attribute_names`, `fn transform`, `fn transform_inplace`
    fn has_attribute(&self, attribute: &str) -> bool {
        match attribute {
            "x" | "y" | "z" => true,
            "r" | "g" | "b" | "a" => self.colors.is_some(),
            "intensity" => self.intensities.is_some(),
            "classification" => self.classifications.is_some(),
            _ => {
                if let Some(ref extra_attrs) = self.extra_attributes {
                    extra_attrs.contains_key(attribute)
                } else {
                    false
                }
            }
        }
    }

    fn transform(&self, _a2b: &[[f32; 4]; 4]) -> Self {
        unimplemented!();
    }

    fn transform_inplace(&mut self, _a2b: &[[f32; 4]; 4]) {
        unimplemented!();
    }

    fn has_classification(&self) -> bool {
        self.classifications.is_some()
    }
    fn has_intensity(&self) -> bool {
        self.intensities.is_some()
    }

    fn attribute_names(&self) -> Vec<String> {
        let mut attrs: Vec<String> = Vec::with_capacity(8);
        attrs.push("x".to_string());
        attrs.push("y".to_string());
        if self.is_3d() {
            attrs.push("z".to_string());
        }
        if let Some(ref colors) = self.colors {
            if colors.len_of(Axis(1)) == 3 || colors.len_of(Axis(1)) == 4 {
                attrs.push("r".to_string());
                attrs.push("g".to_string());
                attrs.push("b".to_string());
                if colors.len_of(Axis(1)) == 4 {
                    attrs.push("a".to_string());
                }
            }
        }
        if self.intensities.is_some() {
            attrs.push("intensity".to_string());
        }
        if self.classifications.is_some() {
            attrs.push("classification".to_string());
        }
        if let Some(ref extra_attrs) = self.extra_attributes {
            for key in extra_attrs.keys() {
                attrs.push(key.clone());
            }
        }
        attrs
    }

    fn with_capacity(capacity: usize) -> Self {
        let mut pc = Self::default();
        pc.reserve(capacity);
        pc
    }

    fn mutable_points(&mut self) -> &mut [Point] {
        unimplemented!()
    }

    fn points(&self) -> &[Point] {
        unimplemented!()
    }

    fn add_point(&mut self, _point: Point) {
        unimplemented!()
    }

    fn clear(&mut self) {
        self.positions = Array2::zeros((0, self.positions.len_of(Axis(1))));
        if let Some(ref mut colors) = self.colors {
            *colors = Array2::zeros((0, colors.len_of(Axis(1))));
        }
        if let Some(ref mut intensities) = self.intensities {
            *intensities = Array2::zeros((0, 1));
        }
        if let Some(ref mut classifications) = self.classifications {
            *classifications = Array2::zeros((0, 1));
        }
        if let Some(ref mut extra_attrs) = self.extra_attributes {
            for (_key, attr_array) in extra_attrs.iter_mut() {
                *attr_array = Array2::zeros((0, 1));
            }
        }
    }

    fn reserve(&mut self, additional: usize) {
        let current_len = self.len();
        let new_capacity = current_len + additional;
        if new_capacity > self._capacity {
            self.positions.reserve_rows(additional).unwrap();
            self._capacity = new_capacity;
            if let Some(ref mut colors) = self.colors {
                colors.reserve_rows(additional).unwrap();
            }
            if let Some(ref mut intensities) = self.intensities {
                intensities.reserve_rows(additional).unwrap();
            }
            if let Some(ref mut classifications) = self.classifications {
                classifications.reserve_rows(additional).unwrap();
            }
            if let Some(ref mut extra_attrs) = self.extra_attributes {
                for (_key, attr_array) in extra_attrs.iter_mut() {
                    attr_array.reserve_rows(additional).unwrap();
                }
            }
        }
    }

    fn is_3d(&self) -> bool {
        self.positions.len_of(Axis(1)) == 3
    }

    fn has_color(&self) -> bool {
        self.colors.is_some()
    }
}
