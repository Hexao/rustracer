use rulinalg::matrix;

fn main() {
    let vec = matrix![1.0; 1.0; 1.0; 1.0];
    let mat = matrix![1.0, 0.0, 0.0, 0.5;
                      0.0, 1.0, 0.0, 1.0;
                      0.0, 0.0, 1.0, -0.5;
                      0.0, 0.0, 0.0, 1.0];

    let po = &mat * &vec;
    println!("{}\n*\n{}\n=\n{}", mat, vec, po);
}
