pub fn point_intersects_polygon((x, y): (f32, f32), polygon: &[(f32, f32)]) -> bool {
    let mut pairs = polygon.windows(2).collect::<Vec<&[(f32, f32)]>>();
    let binding = vec![polygon[0], *polygon.last().unwrap()];
    pairs.push(binding.as_slice());

    pairs
        .iter()
        .map(|l| {
            let (x1, y1) = l[0];
            let (x2, y2) = l[1];

            if ((y1 > y) != (y2 > y)) && (x < (x2 - x1) * (y - y1) / (y2 - y1 + f32::EPSILON) + x1) {
                1
            } else {
                0
            }
        })
        .sum::<u32>()
        % 2
        == 1
}
fn orientation(p: (f32, f32), q: (f32, f32), r: (f32, f32)) -> i32 {
    let val = (q.1 - p.1) * (r.0 - q.0) - (q.0 - p.0) * (r.1 - q.1);
    if val.abs() < f32::EPSILON {
        0
    } else if val > 0.0 {
        1
    } else {
        2
    }
}

fn on_segment(p: (f32, f32), q: (f32, f32), r: (f32, f32)) -> bool {
    q.0 <= p.0.max(r.0) && q.0 >= p.0.min(r.0) &&
        q.1 <= p.1.max(r.1) && q.1 >= p.1.min(r.1)
}

pub fn lines_intersect(a: &[(f32, f32)], b: &[(f32, f32)]) -> bool {
    let (p1, q1) = (a[0], a[1]);
    let (p2, q2) = (b[0], b[1]);

    let o1 = orientation(p1, q1, p2);
    let o2 = orientation(p1, q1, q2);
    let o3 = orientation(p2, q2, p1);
    let o4 = orientation(p2, q2, q1);

    if o1 != o2 && o3 != o4 {
        return true;
    }

    (o1 == 0 && on_segment(p1, p2, q1))
        || (o2 == 0 && on_segment(p1, q2, q1))
        || (o3 == 0 && on_segment(p2, p1, q2))
        || (o4 == 0 && on_segment(p2, q1, q2))
}

pub fn polygons_intersect(a: &[(f32, f32)], b: &[(f32, f32)]) -> bool {
    if a.iter().any(|&p| point_intersects_polygon(p, b)) {
        return true;
    }

    if b.iter().any(|&p| point_intersects_polygon(p, a)) {
        return true;
    }

    for i in 0..a.len() {
        let l_a = &[a[i], a[(i + 1) % a.len()]];
        for j in 0..b.len() {
            let l_b = &[b[j], b[(j+1) % b.len()]];
            if lines_intersect(l_a, l_b) {
                return true;
            }
        }
    }

    false
}
