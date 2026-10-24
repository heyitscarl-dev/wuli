use glam::{DVec2, dvec2};

fn main() {
    let g = dvec2(0.0, -9.81);
    let anchor = DVec2::ZERO;
    let length = 1.0;
    let mut x = dvec2(1.0, -1.0).normalize() * length;
    let mut v = DVec2::ZERO;

    let dt = 1.0 / 60.0; // 60 Hz
    let substeps = 20;
    let h = dt / substeps as f64;

    let w_anchor = 0.0; // => static
    let w_bob = 1.0;

    let mut lf = 0;
    let mut lx = 0.1;

    for frame in 0..600 {
        for _ in 0..substeps {
            let prev = x;
            v += h * g;
            x += h * v;

            // distance constraint between anchor and x

            let d = x - anchor;
            let dist = d.length();
            let n = d / dist;
            let c = dist - length;
            let dlam = -c / (w_anchor + w_bob);

            x += w_bob * dlam * n;

            v = (x - prev) / h;
        }

        // TODO: detect zero crossings of x.x to measure period

        if (lx * x.x) < 0.0 {
            // sign change => zero crossing
            println!("T = {:.2}f", frame - lf);
            lf = frame;
        }

        lx = x.x;

        if frame % 30 == 0 {
            println!("t = {:2}  x = {:.4}", frame as f64 * dt, x.x);
        }
    }
}
