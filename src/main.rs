mod positionin;

fn main() {
    let mut vector = vec![1.0,2.0,3.0];
    let orgi = vector.clone();
    let scale = 2.0;
    let mag = positionin::magnitude(vector[0],vector[1],vector[2]);
    println!("{:?}",vector);
    println!("The magnitude is {}",mag);
    vector[0]=vector[0]/mag;
    vector[1]=vector[1]/mag;
    vector[2]=vector[2]/mag;
    println!("normalized vector : {:?}",vector);
    vector = orgi.clone();
    vector[0]=vector[0]*scale;
    vector[1]=vector[1]*scale;
    vector[2]=vector[2]*scale;
    println!("scale vector : {:?}",vector)
}