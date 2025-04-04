use kasuari::WeightedRelation::*;
use kasuari::{Solver, Strength, Variable};

mod common;
use common::new_values;

#[test]
fn test_quadrilateral() {
    struct Point {
        x: Variable,
        y: Variable,
    }
    impl Point {
        fn new() -> Point {
            Point {
                x: Variable::new(),
                y: Variable::new(),
            }
        }
    }
    let (value_of, update_values) = new_values();

    let points = [Point::new(), Point::new(), Point::new(), Point::new()];
    let point_starts = [(10.0, 10.0), (10.0, 200.0), (200.0, 200.0), (200.0, 10.0)];
    let midpoints = [Point::new(), Point::new(), Point::new(), Point::new()];
    let mut solver = Solver::new();
    let mut weight = 1.0;
    let multiplier = 2.0;
    for i in 0..4 {
        solver
            .add_constraints([
                points[i].x | EQ(Strength::WEAK * weight) | point_starts[i].0,
                points[i].y | EQ(Strength::WEAK * weight) | point_starts[i].1,
            ])
            .unwrap();
        weight *= multiplier;
    }

    for (start, end) in [(0, 1), (1, 2), (2, 3), (3, 0)] {
        solver
            .add_constraints([
                midpoints[start].x
                    | EQ(Strength::REQUIRED)
                    | ((points[start].x + points[end].x) / 2.0),
                midpoints[start].y
                    | EQ(Strength::REQUIRED)
                    | ((points[start].y + points[end].y) / 2.0),
            ])
            .unwrap();
    }

    solver
        .add_constraints([
            (points[0].x + 20.0) | LE(Strength::STRONG) | points[2].x,
            (points[0].x + 20.0) | LE(Strength::STRONG) | points[3].x,
            (points[1].x + 20.0) | LE(Strength::STRONG) | points[2].x,
            (points[1].x + 20.0) | LE(Strength::STRONG) | points[3].x,
            (points[0].y + 20.0) | LE(Strength::STRONG) | points[1].y,
            (points[0].y + 20.0) | LE(Strength::STRONG) | points[2].y,
            (points[3].y + 20.0) | LE(Strength::STRONG) | points[1].y,
            (points[3].y + 20.0) | LE(Strength::STRONG) | points[2].y,
        ])
        .unwrap();

    for point in &points {
        solver
            .add_constraints([
                point.x | GE(Strength::REQUIRED) | 0.0,
                point.y | GE(Strength::REQUIRED) | 0.0,
                point.x | LE(Strength::REQUIRED) | 500.0,
                point.y | LE(Strength::REQUIRED) | 500.0,
            ])
            .unwrap()
    }

    update_values(solver.fetch_changes());

    assert_eq!(
        [
            (value_of(midpoints[0].x), value_of(midpoints[0].y)),
            (value_of(midpoints[1].x), value_of(midpoints[1].y)),
            (value_of(midpoints[2].x), value_of(midpoints[2].y)),
            (value_of(midpoints[3].x), value_of(midpoints[3].y))
        ],
        [(10.0, 105.0), (105.0, 200.0), (200.0, 105.0), (105.0, 10.0)]
    );

    solver
        .add_edit_variable(points[2].x, Strength::STRONG)
        .unwrap();
    solver
        .add_edit_variable(points[2].y, Strength::STRONG)
        .unwrap();
    solver.suggest_value(points[2].x, 300.0).unwrap();
    solver.suggest_value(points[2].y, 400.0).unwrap();

    update_values(solver.fetch_changes());

    assert_eq!(
        [
            (value_of(points[0].x), value_of(points[0].y)),
            (value_of(points[1].x), value_of(points[1].y)),
            (value_of(points[2].x), value_of(points[2].y)),
            (value_of(points[3].x), value_of(points[3].y))
        ],
        [(10.0, 10.0), (10.0, 200.0), (300.0, 400.0), (200.0, 10.0)]
    );

    assert_eq!(
        [
            (value_of(midpoints[0].x), value_of(midpoints[0].y)),
            (value_of(midpoints[1].x), value_of(midpoints[1].y)),
            (value_of(midpoints[2].x), value_of(midpoints[2].y)),
            (value_of(midpoints[3].x), value_of(midpoints[3].y))
        ],
        [(10.0, 105.0), (155.0, 300.0), (250.0, 205.0), (105.0, 10.0)]
    );
}
