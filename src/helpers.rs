use rltk::Point;

pub fn points_in_circle(start_point: Point, radius: i32) -> Vec<Point> {
    let mut targets: Vec<Point> = Vec::new();
    for x in start_point.x - radius..=start_point.x + radius {
        for y in start_point.y - radius..=start_point.y + radius {
            if distance(start_point, Point { x: x, y: y }) < radius {
                targets.push(Point { x, y });
            }
        }
    }
    targets
}

fn distance(start_point: Point, end_point: Point) -> i32 {
    (start_point.x - end_point.x).abs() + (start_point.y - end_point.y).abs()
}
