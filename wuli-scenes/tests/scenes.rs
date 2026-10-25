use wuli::World;
use wuli_scenes::{billiards_break, counting_pi, galton_board, newtons_row};

const DT: f64 = 1.0 / 240.0;

fn run(world: &mut World, seconds: f64, mut each_step: impl FnMut(&World)) {
    for _ in 0..(seconds / DT).round() as usize {
        world.step(DT);
        each_step(world);
    }
}

#[test]
fn every_scene_stays_finite_and_calm() {
    for (name, make) in wuli_scenes::available() {
        let mut world = make();
        run(&mut world, 5.0, |w| {
            for body in w.bodies.values() {
                assert!(
                    body.position.is_finite()
                        && body.velocity.is_finite()
                        && body.angle.is_finite(),
                    "{name}: a body's state became non-finite"
                );
                // Bodies that start out overlapping get pushed apart at hundreds of m/s
                assert!(
                    body.velocity.length() < 50.0,
                    "{name}: a body reached {} m/s",
                    body.velocity.length()
                );
            }
        });
    }
}

#[test]
fn walled_scenes_keep_their_bodies_inside() {
    for (name, make, bound) in [
        ("Newton's Row", newtons_row as fn() -> World, 4.0),
        ("Billiards Break", billiards_break, 4.1),
        ("Galton Board", galton_board, 3.5),
    ] {
        let mut world = make();
        run(&mut world, 10.0, |w| {
            for body in w.bodies.values() {
                assert!(
                    body.position.abs().max_element() < bound + 0.5,
                    "{name}: a body escaped to {}",
                    body.position
                );
            }
        });
    }
}

#[test]
fn counting_pi_collides_31_times() {
    let mut world = counting_pi();
    let small = world.handles()[0];

    // Every collision changes the small ball's velocity; count the steps where that starts
    let mut collisions = 0;
    let mut previous = world.body(small).unwrap().velocity.x;
    let mut was_changing = false;
    run(&mut world, 10.0, |w| {
        let v = w.body(small).unwrap().velocity.x;
        let changing = (v - previous).abs() > 1e-6;
        if changing && !was_changing {
            collisions += 1;
        }
        was_changing = changing;
        previous = v;
    });

    assert_eq!(collisions, 31);
}
