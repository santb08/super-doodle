pub fn magnitude(x: f64, y: f64 , z: f64 ) -> f64 {
    let res = x*x+y*y+z*z;
    res.sqrt()
}
