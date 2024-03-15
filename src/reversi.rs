#[rustfmt::skip]
pub mod templates {
    const fn array_to_u8(arr: [u8; 8]) -> u8 {
        (arr[0] << 7) | (arr[1] << 6) | (arr[2] << 5) | (arr[3] << 4) |
        (arr[4] << 3) | (arr[5] << 2) | (arr[6] << 1) | arr[7]
    }

    pub const INIT_WHITE: u64 =
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 0  |
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 8  |
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 16 |
        (array_to_u8([0, 0, 0, 0, 1, 0, 0, 0]) as u64) << 24 |
        (array_to_u8([0, 0, 0, 1, 0, 0, 0, 0]) as u64) << 32 |
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 40 |
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 48 |
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 56;

    pub const INIT_BLACK: u64 =
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 0  |
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 8  |
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 16 |
        (array_to_u8([0, 0, 0, 1, 0, 0, 0, 0]) as u64) << 24 |
        (array_to_u8([0, 0, 0, 0, 1, 0, 0, 0]) as u64) << 32 |
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 40 |
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 48 |
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 56;

    pub const BEST_POSITIONS: u64 =
        (array_to_u8([1, 0, 0, 0, 0, 0, 0, 1]) as u64) << 0  |
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 8  |
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 16 |
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 24 |
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 32 |
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 40 |
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 48 |
        (array_to_u8([1, 0, 0, 0, 0, 0, 0, 1]) as u64) << 56;

    pub const GOOD_POSITIONS: u64 =
        (array_to_u8([0, 1, 1, 1, 1, 1, 1, 0]) as u64) << 0  |
        (array_to_u8([1, 0, 0, 0, 0, 0, 0, 1]) as u64) << 8  |
        (array_to_u8([1, 0, 0, 0, 0, 0, 0, 1]) as u64) << 16 |
        (array_to_u8([1, 0, 0, 0, 0, 0, 0, 1]) as u64) << 24 |
        (array_to_u8([1, 0, 0, 0, 0, 0, 0, 1]) as u64) << 32 |
        (array_to_u8([1, 0, 0, 0, 0, 0, 0, 1]) as u64) << 40 |
        (array_to_u8([1, 0, 0, 0, 0, 0, 0, 1]) as u64) << 48 |
        (array_to_u8([0, 1, 1, 1, 1, 1, 1, 0]) as u64) << 56;

    pub const FINE_POSITIONS: u64 =
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 0  |
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 8  |
        (array_to_u8([0, 0, 1, 1, 1, 1, 0, 0]) as u64) << 16 |
        (array_to_u8([0, 0, 1, 1, 1, 1, 0, 0]) as u64) << 24 |
        (array_to_u8([0, 0, 1, 1, 1, 1, 0, 0]) as u64) << 32 |
        (array_to_u8([0, 0, 1, 1, 1, 1, 0, 0]) as u64) << 40 |
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 48 |
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 56;

    pub const BAD_POSITIONS: u64 =
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 0  |
        (array_to_u8([0, 0, 1, 1, 1, 1, 0, 0]) as u64) << 8  |
        (array_to_u8([0, 1, 0, 0, 0, 0, 1, 0]) as u64) << 16 |
        (array_to_u8([0, 1, 0, 0, 0, 0, 1, 0]) as u64) << 24 |
        (array_to_u8([0, 1, 0, 0, 0, 0, 1, 0]) as u64) << 32 |
        (array_to_u8([0, 1, 0, 0, 0, 0, 1, 0]) as u64) << 40 |
        (array_to_u8([0, 0, 1, 1, 1, 1, 0, 0]) as u64) << 48 |
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 56;

    pub const WORST_POSITIONS: u64 =
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 0  |
        (array_to_u8([0, 1, 0, 0, 0, 0, 1, 0]) as u64) << 8  |
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 16 |
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 24 |
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 32 |
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 40 |
        (array_to_u8([0, 1, 0, 0, 0, 0, 1, 0]) as u64) << 48 |
        (array_to_u8([0, 0, 0, 0, 0, 0, 0, 0]) as u64) << 56;
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Direction {
    dx: i32,
    dy: i32,
}

#[derive(Debug, Clone)]
pub struct EvaluatedPosition {
    pub pos: Position,
    pub taken: Vec<Position>,
}

impl Position {
    fn mask(&self) -> u64 {
        1 << (self.x + self.y * 8)
    }

    fn add(&self, dir: Direction) -> Option<Position> {
        let x = self.x + dir.dx;
        let y = self.y + dir.dy;
        if x >= 0 && x < 8 && y >= 0 && y < 8 {
            Some(Position { x, y })
        } else {
            None
        }
    }

    fn iter_all() -> impl Iterator<Item = Position> {
        (0..8).flat_map(|x| (0..8).map(move |y| Position { x, y }))
    }
}

impl Direction {
    const fn new(dx: i32, dy: i32) -> Direction {
        Direction { dx, dy }
    }

    fn mul(&self, scalar: i32) -> Direction {
        Direction {
            dx: self.dx * scalar,
            dy: self.dy * scalar,
        }
    }

    #[rustfmt::skip]
    fn iter_all() -> impl Iterator<Item = Direction> {
        [
            Self::new(-1, -1), Self::new(0, -1), Self::new(1, -1),
            Self::new(-1,  0),                   Self::new(1,  0),
            Self::new(-1,  1), Self::new(0,  1), Self::new(1,  1),
        ]
        .into_iter()
    }
}

pub fn user_pick(player: &mut u64, enemy: &mut u64, choice: &EvaluatedPosition) {
    *player |= choice.pos.mask();
    for pos in &choice.taken {
        *player |= pos.mask();
        *enemy &= !pos.mask();
    }
}

pub fn smart_pick(player: &mut u64, enemy: &mut u64) -> bool {
    let available_positions = !(*player | *enemy);

    let position_templates = [
        templates::BEST_POSITIONS,
        templates::GOOD_POSITIONS,
        templates::FINE_POSITIONS,
        templates::BAD_POSITIONS,
        templates::WORST_POSITIONS,
    ];

    if let Some(choice) = position_templates
        .iter()
        .map(|&template| available_positions & template)
        .filter_map(|possible_positions| {
            if possible_positions != 0 {
                most_rewarding(possible_positions, *player, *enemy)
            } else {
                None
            }
        })
        .next()
    {
        *player |= choice.pos.mask();
        for pos in &choice.taken {
            *player |= pos.mask();
            *enemy &= !pos.mask();
        }

        return true;
    }

    false
}

fn most_rewarding(possible_positions: u64, player: u64, enemy: u64) -> Option<EvaluatedPosition> {
    find_choices(player, enemy, Some(possible_positions))
        .into_iter()
        .max_by_key(|evaluated| evaluated.taken.len())
}

pub fn find_choices(player: u64, enemy: u64, filter: Option<u64>) -> Vec<EvaluatedPosition> {
    Position::iter_all()
        .filter(move |pos| filter.unwrap_or(!(player | enemy)) & pos.mask() != 0)
        .map(|pos| evaluate_position(pos, player, enemy))
        .filter(|evaluated| evaluated.taken.len() > 0)
        .collect()
}

fn evaluate_position(pos: Position, player: u64, enemy: u64) -> EvaluatedPosition {
    let taken = Direction::iter_all().flat_map(|dir| walk_line(pos, dir, player, enemy));
    let taken = taken.collect::<Vec<_>>();
    EvaluatedPosition { pos, taken }
}

fn walk_line(start: Position, direction: Direction, player: u64, enemy: u64) -> Vec<Position> {
    let taken = std::iter::successors(start.add(direction), move |pos| pos.add(direction))
        .take_while(|pos| enemy & pos.mask() != 0)
        .collect::<Vec<_>>();

    if taken.len() > 0 {
        let end = start.add(direction.mul(taken.len() as i32 + 1));
        if end.map_or(0, |p| player & p.mask()) != 0 {
            return taken;
        }
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::templates::*;

    #[test]
    fn test_positions_overlap() {
        let overlap =
            BEST_POSITIONS & GOOD_POSITIONS & FINE_POSITIONS & BAD_POSITIONS & WORST_POSITIONS;

        assert_eq!(
            overlap, 0,
            "Positions from different templates should not overlap"
        );
    }

    #[test]
    fn test_position_coverage() {
        let all_positions =
            BEST_POSITIONS | GOOD_POSITIONS | FINE_POSITIONS | BAD_POSITIONS | WORST_POSITIONS;

        assert_eq!(
            all_positions, 0xFFFF_FFFF_FFFF_FFFF,
            "All positions should be covered by the position templates"
        );
    }
}
