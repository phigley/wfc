
use std::f32;

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

    fn choose<R: Rng>(&mut self, weights: &[PointWeight], mut rng: &mut R) {
        assert!(self.num_allowed > 0);

        let normalized_range = Range::new(0.0, 1.0f32);
        let mut total_weight = 0.0;
        let mut current_choice = None;

        for (index, allow) in self.allowed.iter().enumerate() {
            if *allow {
                let current_weight = weights[index].weight;

                total_weight += current_weight;

                if normalized_range.ind_sample(&mut rng) * total_weight < current_weight {
                    current_choice = Some(index);
                }
            }
        }

        if let Some(index) = current_choice {
            self.force(index);
        } else {
            panic!("did not find an index!");
        }
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
    num_encountered: f32,
    entropy: f32,
}


impl FoundFieldPoint {
    fn new(point: &FieldPoint, point_index: usize, weights: &[PointWeight]) -> FoundFieldPoint {
        FoundFieldPoint {
            point_index,
            entropy: measure_entropy(&point, &weights),
            num_encountered: 1.0,
        }
    }

    fn possibly_better<R: Rng>(
        self,
        new_point: &FieldPoint,
        new_point_index: usize,
        weights: &[PointWeight],
        rng: &mut R,
    ) -> FoundFieldPoint {
        let new_entropy = measure_entropy(&new_point, &weights);

        let epsilon = 1e-6f32;

        if new_entropy < self.entropy - epsilon {
            // Always go for the lower entropy.
            FoundFieldPoint::new(&new_point, new_point_index, &weights)
        } else if new_entropy > self.entropy + epsilon {
            // Always reject higher entropy.
            self
        } else {
            // They are nearly equal, use single pass fair selector.fair
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
        }
    }
}

#[derive(Clone, Debug)]
struct PointWeight {
    weight: f32,
    entropic_element: f32,
}

impl PointWeight {
    pub fn new(weight: f32) -> PointWeight {
        PointWeight {
            weight,
            entropic_element: weight * weight.ln(),
        }
    }
}

fn measure_entropy(point: &FieldPoint, weights: &[PointWeight]) -> f32 {
    let mut total_weight = 0.0f32;
    let mut total_component = 0.0f32;

    for (index, allowed) in point.allowed.iter().enumerate() {
        if *allowed {
            total_weight += weights[index].weight;
            total_component += weights[index].entropic_element;
        }
    }

    total_weight.ln() - (total_component / total_weight)
}

#[derive(Clone, Debug)]
pub struct Field {
    num_potentials: usize,

    boundaries: Vec<Boundary>,
    weights: Vec<PointWeight>,

    width: usize,
    height: usize,

    points: Vec<FieldPoint>,
}

impl Field {
    pub fn new<E: Entry>(potentials: &[E], width: usize, height: usize) -> Field {
        let num_potentials = potentials.len();
        let mut boundaries = Vec::with_capacity(num_potentials);
        let mut weights = Vec::with_capacity(num_potentials);

        for entry in potentials {
            boundaries.push(entry.boundary().clone());
            weights.push(PointWeight::new(entry.weight()));
        }

        let mut prototype_fieldpoint = FieldPoint::new(num_potentials);

        for (entry_index, entry) in potentials.iter().enumerate() {
            if entry.weight() <= 0.0 {
                prototype_fieldpoint.invalidate(entry_index);
            }
        }

        let num_points = width * height;

        let mut points = Vec::new();
        points.resize(num_points, prototype_fieldpoint);

        Field {
            num_potentials,
            boundaries,
            weights,
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
                        ) {
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
                        ) {
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
                        ) {
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
                        ) {
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
            let point = &mut self.points[point_index];
            point.force(potential_index);
        }

        let mut changes = ChangeQueue::new();
        changes.add((x, y));

        self.propagate(changes)
    }

    pub fn step<R: Rng>(&mut self, mut rng: &mut R) -> bool {
        let possible_best_point = self.observe(&mut rng);

        match possible_best_point {
            None => false,
            Some(FoundFieldPoint { point_index, .. }) => {
                {
                    let point = &mut self.points[point_index];
                    point.choose(&self.weights, &mut rng);
                }
                let mut changes = ChangeQueue::new();
                changes.add(generate_coord(point_index, self.width));

                self.propagate(changes)
            }
        }
    }

    fn observe<R: Rng>(&self, mut rng: &mut R) -> Option<FoundFieldPoint> {
        let mut result = None;

        for (current_point_index, current_point) in self.points.iter().enumerate() {
            if current_point.num_allowed > 1 {
                match result {
                    None => {
                        result = Some(FoundFieldPoint::new(
                            &current_point,
                            current_point_index,
                            &self.weights,
                        ))
                    }
                    Some(best_point) => {
                        let new_best_point = best_point.possibly_better(
                            &current_point,
                            current_point_index,
                            &self.weights,
                            &mut rng,
                        );

                        result = Some(new_best_point);
                    }
                }
            }
        }

        result
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

    pub fn render_partial(&self) -> Vec<Vec<usize>> {
        let mut result = Vec::with_capacity(self.height);

        for y in 0..self.height {
            let mut row = Vec::with_capacity(self.width);

            for x in 0..self.width {
                let point_index = generate_index(x, y, self.width);
                let point = &self.points[point_index];

                if let Some(i) = point.extract_selection() {
                    row.push(i);
                } else if point.num_allowed > 0 {
                    row.push(self.num_potentials);
                } else {
                    row.push(usize::max_value());
                }
            }

            result.push(row);
        }

        result
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
                if test_direction(&self.boundaries, source_point, &mut test_point, direction) {
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
            Direction::North => if y == 0 {
                None
            } else {
                Some((x, y - 1))
            },

            Direction::South => if y == self.height - 1 {
                None
            } else {
                Some((x, y + 1))
            },

            Direction::East => if x == self.width - 1 {
                None
            } else {
                Some((x + 1, y))
            },

            Direction::West => if x == 0 {
                None
            } else {
                Some((x - 1, y))
            },
        }
    }
}


fn generate_index(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

fn generate_coord(point_index: usize, width: usize) -> (usize, usize) {
    (point_index % width, point_index / width)
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

    use entry::CharacterEntry;
    use rand;

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
    fn field_point_choose() {
        let weights = [
            PointWeight::new(0.1),
            PointWeight::new(0.2),
            PointWeight::new(0.3),
            PointWeight::new(10.1),
        ];

        let mut rng = rand::thread_rng();

        for _ in 0..20 {
            let mut fieldpoint = FieldPoint::new(4);

            fieldpoint.choose(&weights, &mut rng);

            assert_eq!(fieldpoint.num_allowed, 1);

            if let Some(chosen_index) = fieldpoint.extract_selection() {
                assert!(chosen_index < 4);
            } else {
                panic!("failed to extract selection");
            }
        }
    }

    #[test]
    fn found_fieldpoint() {
        let weights = [
            PointWeight::new(0.1),
            PointWeight::new(0.2),
            PointWeight::new(0.3),
            PointWeight::new(10.1),
        ];

        let mut rng = rand::thread_rng();

        let mut fieldpoint_0 = FieldPoint::new(4);
        let mut fieldpoint_1 = FieldPoint::new(4);

        fieldpoint_1.invalidate(0);

        let inital_best_fieldpoint_a = FoundFieldPoint::new(&fieldpoint_0, 0, &weights);
        let best_fieldpoint_a =
            inital_best_fieldpoint_a.possibly_better(&fieldpoint_1, 1, &weights, &mut rng);

        assert_eq!(best_fieldpoint_a.point_index, 1);

        fieldpoint_0.invalidate(1);
        let inital_best_fieldpoint_b = FoundFieldPoint::new(&fieldpoint_0, 0, &weights);
        let best_fieldpoint_b =
            inital_best_fieldpoint_b.possibly_better(&fieldpoint_1, 1, &weights, &mut rng);

        assert_eq!(best_fieldpoint_b.point_index, 0);
    }

    #[test]
    fn simple_field_propagate_north() {
        let potentials = [
            CharacterEntry::build('-', 1.0, "000|101|000").unwrap(),
            CharacterEntry::build('|', 1.0, "010|000|010").unwrap(),
            CharacterEntry::build(' ', 1.0, "000|000|000").unwrap(),
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
            CharacterEntry::build('-', 1.0, "000|101|000").unwrap(),
            CharacterEntry::build('|', 1.0, "010|000|010").unwrap(),
            CharacterEntry::build(' ', 1.0, "000|000|000").unwrap(),
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
            CharacterEntry::build('-', 1.0, "000|101|000").unwrap(),
            CharacterEntry::build('|', 1.0, "010|000|010").unwrap(),
            CharacterEntry::build(' ', 1.0, "000|000|000").unwrap(),
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
            CharacterEntry::build('-', 1.0, "000|101|000").unwrap(),
            CharacterEntry::build('|', 1.0, "010|000|010").unwrap(),
            CharacterEntry::build(' ', 1.0, "000|000|000").unwrap(),
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
    fn zero_weight_invalidated() {
        let potentials = [
            CharacterEntry::build('┌', 0.1, "000|001|010").unwrap(),
            CharacterEntry::build('┐', 1.0, "000|100|010").unwrap(),
            CharacterEntry::build('└', 0.0, "010|001|000").unwrap(),
            CharacterEntry::build('┘', -1.0, "010|100|000").unwrap(),
        ];

        let field = Field::new(&potentials, 2, 2);

        for p in field.points {
            assert_eq!(p.allowed[0], true);
            assert_eq!(p.allowed[1], true);
            assert_eq!(p.allowed[2], false);
            assert_eq!(p.allowed[3], false);
        }
    }
}
