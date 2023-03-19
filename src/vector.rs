pub struct Vector3 {
    pub x: f32,
    pub y: f32,
}

impl Vector3 {
    pub fn print(&self) {
        println!("[Vector3] x: {} y: {}", self.x, self.y)
    }

    pub fn magnitude(&self) -> f32 {
        (self.x.powf(2.0) + self.y.powf(2.0)).sqrt()
    }

    pub fn add(&self, v2: &Vector3) -> Vector3 {
        let x = self.x + v2.x;
        let y = self.y + v2.y;
        // Creamos un nuevo vector `resultado` vacío para almacenar la suma de los vectores
        let result: Vector3 = Vector3 { x, y };
        return result;
    }

    pub fn sub(&self, v2: &Vector3) -> Vector3 {
        let x = self.x - v2.x;
        let y = self.y - v2.y;
        // Creamos un nuevo vector `resultado` vacío para almacenar la suma de los vectores
        let result: Vector3 = Vector3 { x, y };
        return result;
    }

    pub fn angle_between_vectors(&self, v2: &Vector3) -> Option<f32> {
        let mag_product = self.magnitude() * v2.magnitude();
        if mag_product == 0.0 {
            return None;
        }
        let dot_product = self.x * v2.x + self.y * v2.y;
        let cos_angle = dot_product / mag_product;
        Some(cos_angle.acos())
    }
}
