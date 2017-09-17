
use changequeue::ChangeQueue;
use containerutils::extract_two_elements;
use boundary::Direction;
use entry::Entry;

#[derive(Debug)]
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

    fn select(&mut self, index: usize) {

        for i in 0..self.allowed.len() {
            self.allowed[i] = false;
        }

        self.allowed[index] = true;
        self.num_allowed = 1;
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

pub struct Field<'a> {
    potentials: &'a [Entry],

    width: usize,
    height: usize,

    points: Vec<FieldPoint>,
}

impl<'a> Field<'a> {
    pub fn new(potentials: &'a [Entry], width: usize, height: usize) -> Field {

        let num_potentials = potentials.len();
        let num_points = width * height;

        let mut points = Vec::with_capacity(num_points);

        for _ in 0..num_points {
            points.push(FieldPoint::new(num_potentials));
        }

        Field {
            potentials,
            width,
            height,
            points,
        }
    }

    pub fn simple_test() {
        let potentials = [
            Entry::new('-', false, false, true, true),
            Entry::new('|', true, true, false, false),
            Entry::new(' ', false, false, false, false),
        ];

        let mut field = Field::new(&potentials, 2, 2);

        let test_point_index = field.generate_index(0, 0);

        assert_eq!(field.points[test_point_index].allowed[0], true);
        assert_eq!(field.points[test_point_index].allowed[1], true);
        assert_eq!(field.points[test_point_index].allowed[2], true);
        field.force_potential(0, 1, 1);
        println!("{:?}", field.points);
        assert_eq!(field.points[test_point_index].allowed[0], false);
        assert_eq!(field.points[test_point_index].allowed[1], true);
        assert_eq!(field.points[test_point_index].allowed[2], false);

    }

    fn apply_failed_edge(
        &mut self,
        x: usize,
        y: usize,
        potential_index: usize,
        changes: &mut ChangeQueue<(usize, usize)>,
    ) -> bool {

        let point_index = self.generate_index(x, y);
        let point = &mut self.points[point_index];
        point.invalidate(potential_index);

        if point.num_allowed > 0 {
            changes.add((x, y));
            true
        } else {
            false
        }


    }

    pub fn close_edges(&mut self) -> bool {

        let mut changes = ChangeQueue::new();

        for (potential_index, potential) in self.potentials.iter().enumerate() {

            if potential.requires(Direction::North) {

                for x in 0..self.width {
                    if !self.apply_failed_edge(x, 0, potential_index, &mut changes) {
                        return false;
                    }
                }
            }

            if potential.requires(Direction::South) {

                for x in 0..self.width {
                    let y = self.height - 1;
                    if !self.apply_failed_edge(x, y, potential_index, &mut changes) {
                        return false;
                    }
                }
            }

            if potential.requires(Direction::East) {

                for y in 0..self.height {
                    let x = self.width - 1;

                    if !self.apply_failed_edge(x, y, potential_index, &mut changes) {
                        return false;
                    }
                }
            }

            if potential.requires(Direction::West) {

                for y in 0..self.height {

                    if !self.apply_failed_edge(0, y, potential_index, &mut changes) {
                        return false;
                    }
                }
            }
        }

        self.propagate(changes)
    }


    pub fn force_potential(&mut self, x: usize, y: usize, potential_index: usize) -> bool {

        let point_index = self.generate_index(x, y);

        {
            let mut point = &mut self.points[point_index];
            point.select(potential_index);
        }

        let mut changes = ChangeQueue::new();
        changes.add((x, y));

        self.propagate(changes)
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

    pub fn render(&self) -> Option<String> {

        let mut result = String::new();

        for y in 0..self.height {
            for x in 0..self.width {

                let point_index = self.generate_index(x, y);
                let point = &self.points[point_index];

                if let Some(i) = point.extract_selection() {
                    result.push(self.potentials[i].character);
                } else {
                    return None;
                }
            }

            result.push('\n');
        }

        Some(result)

    }

    fn generate_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn propagate_direction(
        &mut self,
        x: usize,
        y: usize,
        direction: Direction,
        changes: &mut ChangeQueue<(usize, usize)>,
    ) -> bool {

        if let Some((test_x, test_y)) = self.build_delta(x, y, direction) {

            let source_point_index = self.generate_index(x, y);
            let test_point_index = self.generate_index(test_x, test_y);

            if let Some((source_point, mut test_point)) =
                extract_two_elements(&mut self.points, source_point_index, test_point_index)
            {
                if Self::test_direction(
                    &self.potentials,
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
        potentials: &[Entry],
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



#[cfg(test)]
mod tests {

    use super::*;

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

        let test_point_index = field.generate_index(0, 0);

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

        let test_point_index = field.generate_index(0, 1);

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

        let test_point_index = field.generate_index(1, 0);

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

        let test_point_index = field.generate_index(0, 0);

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
            assert_eq!(result, expected);

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
            assert_eq!(result, expected);

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
            assert_eq!(result, expected);

        } else {
            panic!("Field did not fully close.");
        }
    }
}
