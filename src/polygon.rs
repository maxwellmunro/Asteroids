pub fn point_intersects_polygon((x, y): (f32, f32), polygon: Vec<(f32, f32)>) -> bool {
    let mut pairs = polygon.windows(2).collect::<Vec<&[(f32, f32)]>>();
    let binding = vec![polygon[0], *polygon.last().unwrap()];
    pairs.push(binding.as_slice());

    pairs
        .iter()
        .map(|l| {
            let (x1, y1) = l[0];
            let (x2, y2) = l[1];

            if x < x1.min(x2) && y > y1.min(y2) && y < y1.max(y2) {
                1
            } else {
                0
            }
        })
        .sum::<u32>()
        % 2
        == 1
}
