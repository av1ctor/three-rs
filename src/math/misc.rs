use super::{Line3, Vector3};

const EPS: f32 = 1e-10;

pub(crate) fn line_to_line_closest_points( 
    line1: &Line3, 
    line2: &Line3
) -> (Vector3, Vector3) {

	let r = line1.end.sub(&line1.start);
	let s = line2.end.sub(&line2.start);
	let w = line2.start.sub(&line1.start);

	let a = r.dot(&s);
	let b = r.dot(&r);
	let c = s.dot(&s);
	let d = s.dot(&w);
	let e = r.dot(&w);

	let divisor = b * c - a * a;

	let (mut t1, mut t2) = if f32::abs(divisor) < EPS {
		let d1 = -d / c;
		let d2 = (a - d) / c;

		if f32::abs(d1 - 0.5) < f32::abs(d2 - 0.5) {
			(0.0, d1)
		} 
        else {
			(1.0, d2)
		}
	}
    else {
		let t1 = ( d * a + e * c ) / divisor;
        (
            t1,
		    (t1 * a - d) / c
        )
	};

	t2 = f32::max(0.0, f32::min(1.0, t2));
	t1 = f32::max(0.0, f32::min(1.0, t1));

	(
		r.mul_scalar(t1).add(&line1.start),
        s.mul_scalar(t2).add(&line2.start)
    )
}