pub struct Vector3 {
    pub x: f32,
    pub y: f32
}

impl Vector3 {
    pub fn print(&self) {
        println!("[Vector3] x: {} y: {}", self.x, self.y)
    }

    pub fn add(self, vec: Vector3) -> Vector3 {
        return self;
    }

    pub fn magnitude(self) -> f32 {
        let result: f32 = 1234.0; // TODO: Implement
        result
    }
}