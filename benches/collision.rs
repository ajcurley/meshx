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

/// Benchmar for the Triangle/Triangle intersection test
pub fn benchmark_intersects_triangle_triangle(c: &mut Criterion) {
    c.bench_function("Triangle/Triangle Intersection", |b| {
        b.iter(|| {
            let a = generate_vector3();
            let b = generate_vector3();
            let c = generate_vector3();
            let t1 = Triangle::new(a, b, c);

            let d = generate_vector3();
            let e = generate_vector3();
            let f = generate_vector3();
            let t2 = Triangle::new(d, e, f);

            collision::intersects_triangle_triangle(&t1, &t2);
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

criterion_group!(
    benches,
    benchmark_intersects_aabb_triangle,
    benchmark_intersects_triangle_triangle
);
criterion_main!(benches);
