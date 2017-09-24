
use rand::Rng;
use rand::distributions::{IndependentSample, Range};

use changequeue::ChangeQueue;
use containerutils::extract_two_elements;
use boundary::{Boundary, Direction};
use entry::Entry;

#[derive(Clone, Debug)]
struct FieldPoint {
    num_allowed: usize,
    allowed: Vec<bool>,
}

impl FieldPoint {
    fn new(num_allowed: usize) -> FieldPoint {
        let allowed = vec![true; num_allowed];

        FieldPoint {
            num_allowed,
            allowed,
        }
    }

    fn invalidate(&mut self, index: usize) {
        let element = &mut self.allowed[index];

        if *element {
            *element = false;

            assert!(self.num_allowed > 0);
            self.num_allowed -= 1;
        }
    }

    fn force(&mut self, index: usize) {


        for allow in &mut self.allowed {
            *allow = false;
        }

        self.allowed[index] = true;
        self.num_allowed = 1;
    }

    fn choose<R: Rng>(&mut self, mut rng: &mut R) {
        assert!(self.num_allowed > 0);

        let mut selection: i32 = Range::new(0, self.num_allowed as i32).ind_sample(&mut rng);

        for allow in &mut self.allowed {

            if *allow {

                if selection > 0 {
                    *allow = false;
                    self.num_allowed -= 1;
                } else if selection < 0 {
                    *allow = false;
                    self.num_allowed -= 1;
                } else {
                    assert!(*allow);
                }

                selection -= 1;
            }
        }

        assert!(self.num_allowed == 1);
    }

    fn extract_selection(&self) -> Option<usize> {

        if self.num_allowed == 1 {
            for (i, allow) in self.allowed.iter().enumerate() {
                if *allow {
                    return Some(i);
                }
            }
        }

        None
    }
}

struct FoundFieldPoint {
    point_index: usize,
    num_allowed: usize,
    num_encountered: f32,
}


impl FoundFieldPoint {
    fn new(point_index: usize, num_allowed: usize) -> FoundFieldPoint {
        FoundFieldPoint {
            point_index,
            num_allowed,
            num_encountered: 1.0,
        }
    }

    fn possibly_better<R: Rng>(
        self,
        new_point: &FieldPoint,
        new_point_index: usize,
        rng: &mut R,
    ) -> FoundFieldPoint {
        if new_point.num_allowed < self.num_allowed {
            // Always go for the less allowed.
            FoundFieldPoint::new(new_point_index, new_point.num_allowed)
        } else if new_point.num_allowed == self.num_allowed {
            let num_encountered = self.num_encountered + 1.0;

            if rng.gen_range(0.0, self.num_encountered) < 1.0 {

                FoundFieldPoint {
                    point_index: new_point_index,
                    num_encountered,
                    ..self
                }
            } else {
                FoundFieldPoint {
                    num_encountered,
                    ..self
                }
            }
        } else {
            self
        }
    }
}

#[derive(Clone)]
pub struct Field {
    num_potentials: usize,

    boundaries: Vec<Boundary>,

    width: usize,
    height: usize,

    points: Vec<FieldPoint>,
}

impl Field {
    pub fn new(potentials: &[Entry], width: usize, height: usize) -> Field {

        let num_potentials = potentials.len();
        let mut boundaries = Vec::with_capacity(num_potentials);

        for entry in potentials {
            boundaries.push(entry.boundary.clone());
        }


        let num_points = width * height;

        let mut points = Vec::with_capacity(num_points);

        for _ in 0..num_points {
            points.push(FieldPoint::new(num_potentials));
        }

        Field {
            num_potentials,
            boundaries,
            width,
            height,
            points,
        }
    }

    pub fn close_edges(&mut self) -> bool {

        let mut changes = ChangeQueue::new();

        {
            let mut points: &mut [FieldPoint] = &mut self.points;

            for (potential_index, boundary) in self.boundaries.iter().enumerate() {

                if boundary.requires(Direction::North) {

                    for x in 0..self.width {
                        if !apply_failed_edge(
                            x,
                            0,
                            self.width,
                            potential_index,
                            &mut points,
                            &mut changes,
                        )
                        {
                            return false;
                        }
                    }
                }

                if boundary.requires(Direction::South) {

                    for x in 0..self.width {
                        let y = self.height - 1;
                        if !apply_failed_edge(
                            x,
                            y,
                            self.width,
                            potential_index,
                            &mut points,
                            &mut changes,
                        )
                        {
                            return false;
                        }
                    }
                }

                if boundary.requires(Direction::East) {

                    for y in 0..self.height {
                        let x = self.width - 1;

                        if !apply_failed_edge(
                            x,
                            y,
                            self.width,
                            potential_index,
                            &mut points,
                            &mut changes,
                        )
                        {
                            return false;
                        }
                    }
                }

                if boundary.requires(Direction::West) {

                    for y in 0..self.height {

                        if !apply_failed_edge(
                            0,
                            y,
                            self.width,
                            potential_index,
                            &mut points,
                            &mut changes,
                        )
                        {
                            return false;
                        }
                    }
                }
            }
        }

        self.propagate(changes)
    }

    pub fn force_potential(&mut self, x: usize, y: usize, potential_index: usize) -> bool {

        let point_index = generate_index(x, y, self.width);

        {
            let mut point = &mut self.points[point_index];
            point.force(potential_index);
        }

        let mut changes = ChangeQueue::new();
        changes.add((x, y));

        self.propagate(changes)
    }

    pub fn step<R: Rng>(&mut self, mut rng: &mut R) -> bool {

        let mut possible_best_point = None;

        for (current_point_index, current_point) in self.points.iter().enumerate() {

            if current_point.num_allowed > 1 {
                match possible_best_point {
                    None => {
                        possible_best_point = Some(FoundFieldPoint::new(
                            current_point_index,
                            current_point.num_allowed,
                        ))
                    }
                    Some(best_point) => {
                        possible_best_point = Some(best_point.possibly_better(
                            &current_point,
                            current_point_index,
                            &mut rng,
                        ));
                    }
                }

            }
        }

        match possible_best_point {
            None => false,
            Some(FoundFieldPoint { point_index, .. }) => {
                {
                    let mut point = &mut self.points[point_index];
                    point.choose(&mut rng);
                }
                let mut changes = ChangeQueue::new();
                changes.add(self.generate_coord(point_index));

                self.propagate(changes)
            }
        }
    }

    fn propagate(&mut self, mut changes: ChangeQueue<(usize, usize)>) -> bool {

        while !changes.is_empty() {
            if let Some((x, y)) = changes.next() {

                if !self.propagate_direction(x, y, Direction::North, &mut changes) {
                    return false;
                }

                if !self.propagate_direction(x, y, Direction::South, &mut changes) {
                    return false;
                }

                if !self.propagate_direction(x, y, Direction::East, &mut changes) {
                    return false;
                }

                if !self.propagate_direction(x, y, Direction::West, &mut changes) {
                    return false;
                }
            }
        }

        true
    }

    pub fn render(&self) -> Option<Vec<Vec<usize>>> {

        let mut result = Vec::with_capacity(self.height);

        for y in 0..self.height {

            let mut row = Vec::with_capacity(self.width);

            for x in 0..self.width {

                let point_index = generate_index(x, y, self.width);
                let point = &self.points[point_index];

                if let Some(i) = point.extract_selection() {
                    row.push(i);
                } else {
                    return None;
                }
            }

            result.push(row);
        }

        Some(result)

    }

    fn generate_coord(&self, point_index: usize) -> (usize, usize) {

        (point_index % self.width, point_index / self.width)
    }

    fn propagate_direction(
        &mut self,
        x: usize,
        y: usize,
        direction: Direction,
        changes: &mut ChangeQueue<(usize, usize)>,
    ) -> bool {

        if let Some((test_x, test_y)) = self.build_delta(x, y, direction) {

            let source_point_index = generate_index(x, y, self.width);
            let test_point_index = generate_index(test_x, test_y, self.width);

            if let Some((source_point, mut test_point)) =
                extract_two_elements(&mut self.points, source_point_index, test_point_index)
            {
                if Self::test_direction(
                    &self.boundaries,
                    source_point,
                    &mut test_point,
                    direction,
                )
                {
                    if test_point.num_allowed == 0 {
                        return false;
                    }

                    changes.add((test_x, test_y));
                }
            }
        }

        true
    }

    fn build_delta(&self, x: usize, y: usize, direction: Direction) -> Option<(usize, usize)> {

        match direction {
            Direction::North => if y == 0 { None } else { Some((x, y - 1)) },

            Direction::South => {
                if y == self.height {
                    None
                } else {
                    Some((x, y + 1))
                }
            }

            Direction::East => {
                if x == self.width {
                    None
                } else {
                    Some((x + 1, y))
                }
            }

            Direction::West => if x == 0 { None } else { Some((x - 1, y)) },
        }
    }

    fn test_direction(
        potentials: &[Boundary],
        source_point: &FieldPoint,
        test_point: &mut FieldPoint,
        direction: Direction,
    ) -> bool {

        let mut changed = false;

        for test_index in 0..potentials.len() {

            if test_point.allowed[test_index] {

                let mut fits = false;

                for source_index in 0..potentials.len() {

                    if source_point.allowed[source_index] {
                        if potentials[source_index].fits(&potentials[test_index], direction) {
                            fits = true;
                            break;
                        }

                    }

                }

                if !fits {
                    test_point.invalidate(test_index);
                    changed = true;
                }
            }
        }

        changed


    }
}


fn generate_index(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

fn apply_failed_edge(
    x: usize,
    y: usize,
    width: usize,
    potential_index: usize,
    points: &mut [FieldPoint],
    changes: &mut ChangeQueue<(usize, usize)>,
) -> bool {

    let point_index = generate_index(x, y, width);
    let point = &mut points[point_index];
    point.invalidate(potential_index);

    if point.num_allowed > 0 {
        changes.add((x, y));
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use entry;

    #[test]
    fn initialize_fieldpoint() {

        let f0 = FieldPoint::new(3);
        let f1 = FieldPoint::new(6);

        assert_eq!(f0.num_allowed, 3);
        assert_eq!(f0.allowed.len(), 3);
        assert_eq!(f1.num_allowed, 6);
        assert_eq!(f1.allowed.len(), 6);

        for v in f0.allowed {
            assert!(v);
        }

        for v in f1.allowed {
            assert!(v);
        }
    }

    #[test]
    fn invalidate_fieldpoint() {
        let mut f0 = FieldPoint::new(3);

        assert_eq!(f0.num_allowed, 3);

        f0.invalidate(0);
        assert_eq!(f0.allowed[0], false);
        assert_eq!(f0.num_allowed, 2);

        f0.invalidate(2);
        assert_eq!(f0.allowed[2], false);
        assert_eq!(f0.num_allowed, 1);

        // test repeated invalidation
        f0.invalidate(2);
        assert_eq!(f0.allowed[2], false);
        assert_eq!(f0.num_allowed, 1);

        f0.invalidate(1);
        assert_eq!(f0.allowed[1], false);
        assert_eq!(f0.num_allowed, 0);
    }

    #[test]
    fn simple_field_propagate_north() {

        let potentials = [
            Entry::new('-', false, false, true, true),
            Entry::new('|', true, true, false, false),
            Entry::new(' ', false, false, false, false),
        ];

        let mut field = Field::new(&potentials, 2, 2);

        let test_point_index = generate_index(0, 0, field.width);

        assert_eq!(field.points[test_point_index].allowed[0], true);
        assert_eq!(field.points[test_point_index].allowed[1], true);
        assert_eq!(field.points[test_point_index].allowed[2], true);
        assert!(field.force_potential(0, 1, 1));
        assert_eq!(field.points[test_point_index].allowed[0], false);
        assert_eq!(field.points[test_point_index].allowed[1], true);
        assert_eq!(field.points[test_point_index].allowed[2], false);

    }

    #[test]
    fn simple_field_propagate_south() {

        let potentials = [
            Entry::new('-', false, false, true, true),
            Entry::new('|', true, true, false, false),
            Entry::new(' ', false, false, false, false),
        ];

        let mut field = Field::new(&potentials, 2, 2);

        let test_point_index = generate_index(0, 1, field.width);

        assert_eq!(field.points[test_point_index].allowed[0], true);
        assert_eq!(field.points[test_point_index].allowed[1], true);
        assert_eq!(field.points[test_point_index].allowed[2], true);
        assert!(field.force_potential(0, 0, 1));
        assert_eq!(field.points[test_point_index].allowed[0], false);
        assert_eq!(field.points[test_point_index].allowed[1], true);
        assert_eq!(field.points[test_point_index].allowed[2], false);
    }

    #[test]
    fn simple_field_propagate_east() {

        let potentials = [
            Entry::new('-', false, false, true, true),
            Entry::new('|', true, true, false, false),
            Entry::new(' ', false, false, false, false),
        ];

        let mut field = Field::new(&potentials, 2, 2);

        let test_point_index = generate_index(1, 0, field.width);

        assert_eq!(field.points[test_point_index].allowed[0], true);
        assert_eq!(field.points[test_point_index].allowed[1], true);
        assert_eq!(field.points[test_point_index].allowed[2], true);
        assert!(field.force_potential(0, 0, 0));
        assert_eq!(field.points[test_point_index].allowed[0], true);
        assert_eq!(field.points[test_point_index].allowed[1], false);
        assert_eq!(field.points[test_point_index].allowed[2], false);
    }

    #[test]
    fn simple_field_propagate_west() {

        let potentials = [
            Entry::new('-', false, false, true, true),
            Entry::new('|', true, true, false, false),
            Entry::new(' ', false, false, false, false),
        ];

        let mut field = Field::new(&potentials, 2, 2);

        let test_point_index = generate_index(0, 0, field.width);

        assert_eq!(field.points[test_point_index].allowed[0], true);
        assert_eq!(field.points[test_point_index].allowed[1], true);
        assert_eq!(field.points[test_point_index].allowed[2], true);
        assert!(field.force_potential(1, 0, 0));
        assert_eq!(field.points[test_point_index].allowed[0], true);
        assert_eq!(field.points[test_point_index].allowed[1], false);
        assert_eq!(field.points[test_point_index].allowed[2], false);
    }

    #[test]
    fn simple_field_propagate_fail() {

        let potentials = [
            Entry::new('-', false, false, true, true),
            Entry::new('┌', false, true, true, false),
        ];

        let mut field = Field::new(&potentials, 2, 2);
        assert_eq!(field.force_potential(0, 0, 1), false);
    }

    #[test]
    fn simple_field_full_propagate() {

        let potentials = [
            Entry::new('┌', false, true, true, false),
            Entry::new('┐', false, true, false, true),
            Entry::new('└', true, false, true, false),
            Entry::new('┘', true, false, false, true),
        ];

        let mut field = Field::new(&potentials, 2, 2);

        assert!(field.force_potential(0, 0, 0));
        assert!(field.force_potential(1, 0, 1));

        if let Some(result) = field.render() {
            let expected = "┌┐\n└┘\n";
            let result_str = entry::make_string(&potentials, &result);
            assert_eq!(result_str, expected);

        } else {
            panic!("Field did not fully propagate.");
        }
    }

    #[test]
    fn simple_field_closed_edges_fail() {

        let potentials = [
            Entry::new('-', false, false, true, true),
            Entry::new('|', true, true, false, false),
        ];

        let mut field = Field::new(&potentials, 2, 2);
        assert_eq!(field.close_edges(), false);
    }

    #[test]
    fn simple_field_closed_edges() {
        let potentials = [
            Entry::new('-', false, false, true, true),
            Entry::new('|', true, true, false, false),
            Entry::new(' ', false, false, false, false),
        ];

        let mut field = Field::new(&potentials, 2, 2);

        assert!(field.close_edges());

        if let Some(result) = field.render() {
            let expected = "  \n  \n";
            let result_str = entry::make_string(&potentials, &result);
            assert_eq!(result_str, expected);

        } else {
            panic!("Field did not fully close.");
        }
    }

    #[test]
    fn simple_field_closed_edges2() {
        let potentials = [
            Entry::new('┌', false, true, true, false),
            Entry::new('┐', false, true, false, true),
            Entry::new('└', true, false, true, false),
            Entry::new('┘', true, false, false, true),
        ];

        let mut field = Field::new(&potentials, 2, 2);

        assert!(field.close_edges());

        if let Some(result) = field.render() {
            let expected = "┌┐\n└┘\n";
            let result_str = entry::make_string(&potentials, &result);
            assert_eq!(result_str, expected);

        } else {
            panic!("Field did not fully close.");
        }
    }
}
