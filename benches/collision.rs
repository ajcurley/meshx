use criterion::{criterion_group, criterion_main, Criterion};
use meshx::geometry::collision;
use meshx::geometry::{Aabb, Triangle, Vector3};
use rand::prelude::*;

/// Benchmark for the AABB/Triangle intersection test
pub fn benchmark_intersects_aabb_triangle(c: &mut Criterion) {
    c.bench_function("AABB/Triangle Intersection", |b| {
        b.iter(|| {
            let aabb = Aabb::unit();
            let p = generate_vector3();
            let q = generate_vector3();
            let r = generate_vector3();
            let triangle = Triangle::new(p, q, r);
            collision::intersects_aabb_triangle(&aabb, &triangle);
        })
    });
}

/// Generate a random Vector3 in the range (-4, 4) for all
/// coordinate component directions.
fn generate_vector3() -> Vector3 {
    let mut rng = rand::thread_rng();
    let x = (rng.gen::<f64>() - 0.5) * 4.;
    let y = (rng.gen::<f64>() - 0.5) * 4.;
    let z = (rng.gen::<f64>() - 0.5) * 4.;
    Vector3::new(x, y, z)
}

criterion_group!(benches, benchmark_intersects_aabb_triangle);
criterion_main!(benches);
