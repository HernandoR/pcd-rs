use crate::point::Point;
use crate::point_cloud::PointCloud;


pub struct CompactPointCloud{
    x: Vec<f32>,
    y: Vec<f32>,
    z: Option<Vec<f32>>,
    intensity: Option<Vec<f32>>,
    r: Option<Vec<u8>>,
    g: Option<Vec<u8>>,
    b: Option<Vec<u8>>,
    a: Option<Vec<u8>>,
    extra_fields: std::collections::HashMap<String, Vec<u8>>,
}

impl PointCloud for CompactPointCloud {
    fn new() -> Self {
        CompactPointCloud {
            x: Vec::new(),
            y: Vec::new(),
            z: None,
            intensity: None,
            r: None,
            g: None,
            b: None,
            a: None,
            extra_fields: std::collections::HashMap::new(),
        }
    }



    fn with_capacity(capacity: usize) -> Self {
        CompactPointCloud {
            x: Vec::with_capacity(capacity),
            y: Vec::with_capacity(capacity),
            z: None,
            intensity: None,
            r: None,
            g: None,
            b: None,
            a: None,
            extra_fields: std::collections::HashMap::new(),
        }
    }

    fn mutable_points(&mut self) -> &mut [Point] {
        unimplemented!("Direct mutable access to points is not supported in CompactPointCloud");
    }


    fn add_point(&mut self, point: Point) {
        self.x.push(point.x);
        self.y.push(point.y);

        if let Some(zv) = point.z {
            if self.z.is_none() {
                self.z = Some(vec![0.0; self.x.len() - 1]);
            }
            if let Some(zcol) = &mut self.z {
                zcol.push(zv);
            }
        } else if let Some(zcol) = &mut self.z {
            zcol.push(0.0);
        }

        if let Some(inten) = point.intensity {
            if self.intensity.is_none() {
                self.intensity = Some(vec![0.0; self.x.len() - 1]);
            }
            if let Some(icol) = &mut self.intensity {
                icol.push(inten);
            }
        } else if let Some(icol) = &mut self.intensity {
            icol.push(0.0);
        }

        if let Some(rv) = point.r {
            if self.r.is_none() {
                self.r = Some(vec![0; self.x.len() - 1]);
            }
            if let Some(rcol) = &mut self.r {
                rcol.push(rv);
            }
        } else if let Some(rcol) = &mut self.r {
            rcol.push(0);
        }

        if let Some(gv) = point.g {
            if self.g.is_none() {
                self.g = Some(vec![0; self.x.len() - 1]);
            }
            if let Some(gcol) = &mut self.g {
                gcol.push(gv);
            }
        } else if let Some(gcol) = &mut self.g {
            gcol.push(0);
        }

        if let Some(bv) = point.b {
            if self.b.is_none() {
                self.b = Some(vec![0; self.x.len() - 1]);
            }
            if let Some(bcol) = &mut self.b {
                bcol.push(bv);
            }
        } else if let Some(bcol) = &mut self.b {
            bcol.push(0);
        }

        if let Some(av) = point.a {
            if self.a.is_none() {
                self.a = Some(vec![0; self.x.len() - 1]);
            }
            if let Some(acol) = &mut self.a {
                acol.push(av);
            }
        } else if let Some(acol) = &mut self.a {
            acol.push(0);
        }
    }

    fn clear(&mut self) {
        self.x.clear();
        self.y.clear();
        if let Some(zcol) = &mut self.z { zcol.clear(); }
        if let Some(icol) = &mut self.intensity { icol.clear(); }
        if let Some(rcol) = &mut self.r { rcol.clear(); }
        if let Some(gcol) = &mut self.g { gcol.clear(); }
        if let Some(bcol) = &mut self.b { bcol.clear(); }
        if let Some(acol) = &mut self.a { acol.clear(); }
        self.extra_fields.clear();
    }

    fn reserve(&mut self, additional: usize) {
        self.x.reserve(additional);
        self.y.reserve(additional);
        if let Some(zcol) = &mut self.z { zcol.reserve(additional); }
        if let Some(icol) = &mut self.intensity { icol.reserve(additional); }
        if let Some(rcol) = &mut self.r { rcol.reserve(additional); }
        if let Some(gcol) = &mut self.g { gcol.reserve(additional); }
        if let Some(bcol) = &mut self.b { bcol.reserve(additional); }
        if let Some(acol) = &mut self.a { acol.reserve(additional); }
    }

    fn points(&self) -> &[Point] {
        unimplemented!("Direct access to points is not supported in CompactPointCloud");
    }

    fn num_points(&self) -> usize {
        self.x.len()
    }

    fn is_3d(&self) -> bool {
        self.z.is_some()
    }

    fn has_color(&self) -> bool {
        self.r.is_some() && self.g.is_some() && self.b.is_some()
    }

    fn has_intensity(&self) -> bool {
        self.intensity.is_some()
    }

    fn has_attribute(&self, attribute: &str) -> bool {
        match attribute {
            "x" | "y" => true,
            "z" => self.z.is_some(),
            "r" | "g" | "b" | "a" => match attribute {
                "r" => self.r.is_some(),
                "g" => self.g.is_some(),
                "b" => self.b.is_some(),
                "a" => self.a.is_some(),
                _ => false,
            },
            "intensity" => self.intensity.is_some(),
            _ => self.extra_fields.contains_key(attribute),
        }
    }

    fn transform<F>(self, mut func: F) -> Self
    where
        F: FnMut(Point) -> Point,
    {
        // Build a SimplePointCloud-like stream by reconstructing Points,
        // applying func, and pushing into a new CompactPointCloud
        let mut out = Self::with_capacity(self.num_points());

        for i in 0..self.num_points() {
            let p = Point {
                x: self.x[i],
                y: self.y[i],
                z: self.z.as_ref().and_then(|zcol| zcol.get(i).copied()),
                r: self.r.as_ref().and_then(|rcol| rcol.get(i).copied()),
                g: self.g.as_ref().and_then(|gcol| gcol.get(i).copied()),
                b: self.b.as_ref().and_then(|bcol| bcol.get(i).copied()),
                a: self.a.as_ref().and_then(|acol| acol.get(i).copied()),
                intensity: self.intensity.as_ref().and_then(|icol| icol.get(i).copied()),
                ring_id: None,
                time_offset: None,
            };
            let np = func(p);
            out.add_point(np);
        }

        out
    }

    fn transform_inplace<F>(&mut self, mut func: F)
    where
        F: FnMut(Point) -> Point,
    {
        // Apply func to each reconstructed Point and write back into columns
        let n = self.num_points();
        for i in 0..n {
            let p = Point {
                x: self.x[i],
                y: self.y[i],
                z: self.z.as_ref().and_then(|zcol| zcol.get(i).copied()),
                r: self.r.as_ref().and_then(|rcol| rcol.get(i).copied()),
                g: self.g.as_ref().and_then(|gcol| gcol.get(i).copied()),
                b: self.b.as_ref().and_then(|bcol| bcol.get(i).copied()),
                a: self.a.as_ref().and_then(|acol| acol.get(i).copied()),
                intensity: self.intensity.as_ref().and_then(|icol| icol.get(i).copied()),
                ring_id: None,
                time_offset: None,
            };
            let np = func(p);

            self.x[i] = np.x;
            self.y[i] = np.y;

            if let Some(zv) = np.z {
                if self.z.is_none() {
                    self.z = Some(vec![0.0; n]);
                }
                if let Some(zcol) = &mut self.z { zcol[i] = zv; }
            }

            if let Some(iv) = np.intensity {
                if self.intensity.is_none() {
                    self.intensity = Some(vec![0.0; n]);
                }
                if let Some(icol) = &mut self.intensity { icol[i] = iv; }
            }

            if let Some(rv) = np.r {
                if self.r.is_none() {
                    self.r = Some(vec![0; n]);
                }
                if let Some(rcol) = &mut self.r { rcol[i] = rv; }
            }

            if let Some(gv) = np.g {
                if self.g.is_none() {
                    self.g = Some(vec![0; n]);
                }
                if let Some(gcol) = &mut self.g { gcol[i] = gv; }
            }

            if let Some(bv) = np.b {
                if self.b.is_none() {
                    self.b = Some(vec![0; n]);
                }
                if let Some(bcol) = &mut self.b { bcol[i] = bv; }
            }

            if let Some(av) = np.a {
                if self.a.is_none() {
                    self.a = Some(vec![0; n]);
                }
                if let Some(acol) = &mut self.a { acol[i] = av; }
            }
        }
    }
}