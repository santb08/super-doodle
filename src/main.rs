use vector::Vector3;

mod vector;

fn main() {
    let vec = Vector3 {
        x: 10.0,
        y: 20.0
    };

    vec.print();

    let vec2 = Vector3 {
        x: 5,
        y: 6
    };

    // let result = vec.add(vec2);

    let mag = vec.magnitude();
}
