use vector::Vector3;

mod vector;

fn main() {
    let vec = Vector3 { x: 10.0, y: 20.0 };

    vec.print();
    // Creamos dos vectores, `v1` y `v2`, que contienen los valores `[1, 2, 3]` y `[4, 5, 6]`, respectivamente
    let v1: Vector3 = Vector3 { x: (3.0), y: (3.0) };
    let v2: Vector3 = Vector3 { x: (4.0), y: (4.0) };

    let result = v1.add(&v2);
    result.print();

    let result = v1.sub(&v2);
    result.print();

    let angle = v1.angle_between_vectors(&v2);
    match angle {
            Some(angle) => println!("El ángulo entre los vectores es: {:.2} grados", angle.to_degrees()),
            None => println!("El ángulo entre los vectores no se puede calcular porque uno o ambos vectores tienen magnitud cero."),
        }
}
