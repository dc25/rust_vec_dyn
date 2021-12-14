/*
Demonstrates that you can work with a Vec of dyn traits even without a Box to hold them.
Motivated by this response: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=c11f8f2b3e5481faafb6367226de8c1e
to this blog post: https://dev.to/magnusstrale/rust-trait-objects-in-a-vector-non-trivial-4co5

Added another function call that takes Vec<Box<dyn Shape>> for contrast.
*/


#[derive(Debug, Copy, Clone)]
struct Matrix {
	foo: i32,
}

#[derive(Debug)]
struct Material {
	bar: i32,
}

trait Shape {
    fn material(&self) -> &Material;
    fn transformation(&self) -> Matrix;
}

// Original code
#[derive(Debug)]
struct Sphere {
    transformation: Matrix, // FYI - implements Copy trait
    material: Material
}

impl Shape for Sphere {
    fn material(&self) -> &Material {
        &self.material
    }
    fn transformation(&self) -> Matrix {
        self.transformation
    }
}

fn do_stuff(objects: Vec<&dyn Shape>) {
    for i in objects {
        println!("{:#?}", i.material());
    }
}

fn do_stuff_box(objects: Vec<Box<dyn Shape>>) {
    for i in objects {
        println!("{:#?}", i.transformation());
    }
}

fn main() {
    let material = Material{bar: 1};
    let transformation = Matrix{foo: 2};
    let sphere = Sphere{transformation, material};


    let shape = &sphere as &dyn Shape;
    let v = vec![shape];
    do_stuff(v);

    let box_dyn_shape : Box<dyn Shape> = Box::new(sphere);
    let v_box = vec![box_dyn_shape];
    do_stuff_box(v_box);
}
