extern crate rand;
use rand::distributions::{IndependentSample, Range};
use nalgebra::Vec3;
use material;
use std::sync::Arc;
use objects;
use objects::{Hitable, BVHFindHit, HitableList, sphere};
use objects::bvh::Node;
use objects::sphere::{MovingSphere, Sphere};

pub fn three_world() -> (Arc<BVHFindHit>, Arc<BVHFindHit>) {
    let mut world = Vec::<Arc<Hitable>>::new();
    let base_mat = Arc::new(material::Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let sphere = Arc::new(Sphere::new(Vec3::new(0.0, (0.0 - 100.5), 0.0), 100.0, base_mat.clone()));
    world.push(sphere.clone());


    let lam1 = Arc::new(material::Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let sphere = Arc::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0 - 1.0), 0.5, lam1.clone()));
    world.push(sphere.clone());

    let metal1 = Arc::new(material::Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0));
    let sphere = Arc::new(Sphere::new(Vec3::new(1.0, 0.0, 0.0 - 1.0), 0.5, metal1.clone()));
    world.push(sphere.clone());

    let die1 = Arc::new(material::Dielectric::new(1.5));
    let sphere = Arc::new(Sphere::new(Vec3::new(0.0 - 1.0, 0.0, 0.0 - 1.0), 0.5, die1.clone()));
    world.push(sphere.clone());
    let mut hitlist = HitableList::new();
    for record in &world {
        hitlist.push(record.clone());
    }
    let n = Node::new(world, None, None, None);
    n.print("  ".to_string(), None);
    (Arc::new(n), Arc::new(hitlist))
}

pub fn corner_world() -> (Arc<BVHFindHit>, Arc<BVHFindHit>) {
    let mut world = Vec::<Arc<Hitable>>::new();

    let base_mat = Arc::new(material::Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    let sphere = Arc::new(Sphere::new(Vec3::new(0.0, (0.0 - 1000.0), 0.0),
                                      1000.0,
                                      base_mat.clone()));
    world.push(sphere.clone());
    let metal1 = Arc::new(material::Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0));
    let base_mat = Arc::new(material::Lambertian::new(Vec3::new(0.9, 0.0, 0.0)));
    let lam = Arc::new(material::Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    let lam2 = Arc::new(material::Lambertian::new(Vec3::new(0.9, 0.9, 0.9)));
    let lam3 = Arc::new(material::Lambertian::new(Vec3::new(0.0, 0.0, 0.0)));

    let sphere = Arc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, metal1.clone()));
    world.push(sphere.clone());
    let sphere = Arc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, lam2.clone()));
    world.push(sphere.clone());
    let sphere = Arc::new(Sphere::new(Vec3::new(-4.0,1.0, -4.0), 1.0, base_mat.clone()));
    world.push(sphere.clone());
    let sphere = Arc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, lam3.clone()));
    world.push(sphere.clone());
    let sphere = Arc::new(Sphere::new(Vec3::new(4.0, 1.0, 4.0), 1.0, lam.clone()));
    world.push(sphere.clone());
    let mut hitlist = HitableList::new();
    for record in &world {
        hitlist.push(record.clone());
    }
    let n = Node::new(world, None, None, None);
    n.print("  ".to_string(), None);
    (Arc::new(n), Arc::new(hitlist))
}

pub fn random_world() -> (Arc<BVHFindHit>, Arc<BVHFindHit>) {
    let mut rng = rand::thread_rng();
    let random_index = Range::new(0.0, 1.0);
    let random_size_index = Range::new(0.03, 0.55);
    let mut world = Vec::<Arc<Hitable>>::new();
    let base_mat = Arc::new(material::Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));

    let metal_base = Arc::new(material::Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.5));
    let sphere = Arc::new(Sphere::new(Vec3::new(0.0, (0.0 - 1000.0), 0.0),
                                      1000.0,
                                      base_mat.clone()));
    world.push(sphere.clone());
    let minus_vec = Vec3::new(4.0, 0.2, 0.0);
    for a in (0 - 10)..10 {
        for b in (0 - 10)..10 {
            let rand_size = 0.2;//random_size_index.ind_sample(&mut rng);
            let rand_mat = random_index.ind_sample(&mut rng);
            let center = Vec3::new(a as f32 + 0.9 * random_index.ind_sample(&mut rng),
                                   0.2,
                                   b as f32 * 0.9 * random_index.ind_sample(&mut rng));
            if (center - minus_vec).len() as f32 > 0.92 {
                let sphere: Arc<objects::Hitable> = if rand_mat < 0.0 {

                    let one = random_index.ind_sample(&mut rng) * random_index.ind_sample(&mut rng);
                    let two = random_index.ind_sample(&mut rng) * random_index.ind_sample(&mut rng);
                    let three = random_index.ind_sample(&mut rng) *
                                random_index.ind_sample(&mut rng);
                    let base_mat = Arc::new(material::Lambertian::new(Vec3::new(one, two, three)));
                    let center1 = center + Vec3::new(0.0, 0.5, 0.0);

                    Arc::new(MovingSphere::new(center,
                                               center1,
                                               rand_size,
                                               base_mat.clone(),
                                               0.0,
                                               1.0))
                } else if rand_mat < 0.5 {
                    let one = random_index.ind_sample(&mut rng) * random_index.ind_sample(&mut rng);
                    let two = random_index.ind_sample(&mut rng) * random_index.ind_sample(&mut rng);
                    let three = random_index.ind_sample(&mut rng) *
                                random_index.ind_sample(&mut rng);
                    let base_mat = Arc::new(material::Lambertian::new(Vec3::new(one, two, three)));
                    Arc::new(Sphere::new(center, rand_size, base_mat.clone()))
                } else if rand_mat < 0.75 {
                    let one = random_index.ind_sample(&mut rng) * random_index.ind_sample(&mut rng);
                    let two = random_index.ind_sample(&mut rng) * random_index.ind_sample(&mut rng);
                    let three = random_index.ind_sample(&mut rng) *
                                random_index.ind_sample(&mut rng);
                    let four = random_index.ind_sample(&mut rng) *
                               random_index.ind_sample(&mut rng);
                    let base_mat = Arc::new(material::Metal::new(Vec3::new(0.5 * (1.0 + one),
                                                                           0.5 * (1.0 + two),
                                                                           0.5 * (1.0 + three)),
                                                                 0.5 * four));
                    Arc::new(Sphere::new(center, rand_size, base_mat.clone()))
                } else {
                    let base_mat = Arc::new(material::Dielectric::new(1.5));
                    Arc::new(Sphere::new(center, rand_size, base_mat.clone()))
                };
                world.push(sphere.clone());
            }
        }
        let die1 = Arc::new(material::Dielectric::new(1.5));
        let metal1 = Arc::new(material::Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
        let lam = Arc::new(material::Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
        let sphere = Arc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, die1.clone()));
        world.push(sphere.clone());

        let sphere = Arc::new(Sphere::new(Vec3::new(0.0 - 4.0, 1.0, 0.0), 1.0, lam.clone()));
        world.push(sphere.clone());

        let sphere = Arc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, metal1.clone()));
        world.push(sphere.clone());
    }
    let mut hitlist = HitableList::new();
    for record in &world {
        hitlist.push(record.clone());
    }
    let n = Node::new(world, None, None, None);
    n.print("  ".to_string(), None);
    (Arc::new(n), Arc::new(hitlist))
}
