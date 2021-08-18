use std::f32;
use std::fmt;
use std::ops;
use std::ops::*;
use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub e: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y:f32, z:f32) -> Vec3 {
        Vec3 { e: [x, y, z]}
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32{
        self.e[2]
    }

    pub fn r(&self) -> f32 {
        self.e[0]
    }

    pub fn g(&self) -> f32 {
        self.e[1]
    }

    pub fn b(&self) -> f32 {
        self.e[2]
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn dot(&self, v2: Vec3) -> f32 {
        self.e[0] * v2.e[0] + self.e[1] * v2.e[1] + self.e[2] * v2.e[2]
    }

    pub fn cross(&self, v2: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[1] * v2.e[2] - self.e[2] * v2.e[1],
                self.e[2] * v2.e[0] - self.e[0] * v2.e[2],
                self.e[0] * v2.e[1] - self.e[1] * v2.e[0],
            ]
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        return self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }

}

impl Neg for Vec3 {
    type Output = Vec3;
    
    fn neg(self) -> Vec3 {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

// Add
impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3{ 
        e: [
            a.e[0] + b.e[0],
            a.e[1] + b.e[1],
            a.e[2] + b.e[2],
        ]
    }
});

// Sub
impl_op_ex!(- |a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3{ 
        e: [
            a.e[0] - b.e[0],
            a.e[1] - b.e[1],
            a.e[2] - b.e[2],
        ]
    }
});

// Mul
impl_op_ex!(* |a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3{ 
        e: [
            a.e[0] * b.e[0],
            a.e[1] * b.e[1],
            a.e[2] * b.e[2],
        ]
    }
});

// Div
impl_op_ex!(/ |a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3{ 
        e: [
            a.e[0] / b.e[0],
            a.e[1] / b.e[1],
            a.e[2] / b.e[2],
        ]
    }
});




// Vec3 add f32
impl_op_ex!(+ |a: &Vec3, b: &f32| -> Vec3 {
    Vec3{ 
        e: [
            a.e[0] + b,
            a.e[1] + b,
            a.e[2] + b,
        ]
    }
});

// Vec3 sub f32
impl_op_ex!(- |a: &Vec3, b: &f32| -> Vec3 {
    Vec3{ 
        e: [
            a.e[0] - b,
            a.e[1] - b,
            a.e[2] - b,
        ]
    }
});

// Vec3 mul f32
impl_op_ex!(* |a: &Vec3, b: &f32| -> Vec3 {
    Vec3{ 
        e: [
            a.e[0] * b,
            a.e[1] * b,
            a.e[2] * b,
        ]
    }
});

// Vec3 div f32
impl_op_ex!(/ |a: &Vec3, b: &f32| -> Vec3 {
    Vec3{ 
        e: [
            a.e[0] / b,
            a.e[1] / b,
            a.e[2] / b,
        ]
    }
});


// f32 add Vec3
impl_op_ex!(+ |a: &f32, b: &Vec3| -> Vec3 {
    Vec3{ 
        e: [
            a + b.e[0],
            a + b.e[1],
            a + b.e[2],
        ]
    }
});

// f32 sub Vec3
impl_op_ex!(- |a: &f32, b: &Vec3| -> Vec3 {
    Vec3{ 
        e: [
            a - b.e[0],
            a - b.e[1],
            a - b.e[2],
        ]
    }
});

// f32 mul Vec3
impl_op_ex!(* |a: &f32, b: &Vec3| -> Vec3 {
    Vec3{ 
        e: [
            a * b.e[0],
            a * b.e[1],
            a * b.e[2],
        ]
    }
});

// f32 div Vec3
impl_op_ex!(/ |a: &f32, b: &Vec3| -> Vec3 {
    Vec3{ 
        e: [
            a / b.e[0],
            a / b.e[1],
            a / b.e[2],
        ]
    }
});

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, i:usize) -> &f32 {
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i:usize) -> &mut f32 {
        &mut self.e[i]
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn dot(v1: Vec3, v2: Vec3) -> f32 {
    v1.e[0] * v2.e[0] + 
    v1.e[1] * v2.e[1] +
    v1.e[2] * v2.e[2]
}

pub fn random_vec3() -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
}

pub fn random_range(min: f32, max: f32) -> Vec3 {
    let mut rng = rand::thread_rng();
    Vec3::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max)
    )
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p = Vec3::new(1.0, 1.0, 1.0);
    while p.length_squared() >= 1.0 {
        p = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
             * 2.0 - Vec3::new(1.0, 1.0, 1.0);
    }
    p
}

pub fn random_unit_vector() -> Vec3 {
    return unit_vector(random_in_unit_sphere());
}

pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        return in_unit_sphere;
    }
    else {
        return -in_unit_sphere;
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.length_squared() >= 1.0 {continue;}
        return p;
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str = format!("x: {} y:{} z{}", self.e[0], self.e[1], self.e[2]);
        f.write_str(&str.to_string())?;
        Ok(())
    }
}