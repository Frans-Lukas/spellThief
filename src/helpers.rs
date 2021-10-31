use rltk::Point;

pub fn points_in_circle(start_point: Point, radius: i32) -> Vec<Point> {
    let mut targets: Vec<Point> = Vec::new();
    for x in start_point.x - radius..=start_point.x + radius {
        for y in start_point.y - radius..=start_point.y + radius {
            if rltk::DistanceAlg::Pythagoras.distance2d(Point { x, y }, start_point)
                < (radius) as f32
            {
                targets.push(Point { x, y });
            }
        }
    }
    targets
}
