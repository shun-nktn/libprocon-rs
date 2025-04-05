#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pos: (usize, usize),
}

impl Position {
    pub fn new(pos: (usize, usize)) -> Self {
        Self { pos }
    }

    pub fn to_zero_origin(self) -> Self {
        let pos = (self.pos.0 - 1, self.pos.1 - 1);
        Self { pos }
    }

    pub fn go(self, direction: (isize, isize), size: (usize, usize)) -> Option<Self> {
        let (a, b) = self.pos;
        let (da, db) = direction;
        let next = (a.checked_add_signed(da), b.checked_add_signed(db));
        if let (Some(na), Some(nb)) = next {
            if na < size.0 && nb < size.1 {
                let pos = (na, nb);
                return Some(Self { pos });
            }
        }
        return None;
    }

    pub fn char_at(self, grid: &Vec<Vec<char>>) -> char {
        grid[self.pos.0][self.pos.1]
    }

    pub fn is_space_in(self, grid: &Vec<Vec<char>>) -> bool {
        self.char_at(grid) == '.'
    }

    pub fn is_wall_in(self, grid: &Vec<Vec<char>>) -> bool {
        self.char_at(grid) == '#'
    }

    pub fn is_already(self, visited: &Vec<Vec<bool>>) -> bool {
        visited[self.pos.0][self.pos.1]
    }

    pub fn mark_as(self, visited: &mut Vec<Vec<bool>>) {
        visited[self.pos.0][self.pos.1] = true;
    }
}