use vector::Vector3;

mod vector;

fn main() {
    let vec = Vector3 {
        x: 10.0,
        y: 20.0
    };

    vec.print();
    
    print!("Magnitude {}", &vec.magnitude());
}
