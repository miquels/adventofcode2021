use std::cmp;
use std::time::Instant;
use std::collections::HashMap;
use scan_fmt::*;

// position 0: room 1 front
// position 1: room 1 back
// ....
// position 8-18: hallway.
type Burrow = [u8; 27];

struct Game<const ROOM_LENGTH: usize, const HALL_START: usize, const HALL_END: usize> {
    burrow:   Burrow,
    solved:   Burrow,
    visited:  HashMap<Burrow, u32>,
    min_cost: u32,
}

const EMPTY: u8 = 4;

impl<const ROOM_LENGTH: usize, const HALL_START: usize, const HALL_END: usize> Game<ROOM_LENGTH, HALL_START, HALL_END> {
    fn from_stdin() -> Self {
        let _skip = scanln_fmt!("{}", String);
        let _skip = scanln_fmt!("{}", String);
        let (a0, b0, c0, d0) = scanln_fmt!("###{}#{}#{}#{}###", char, char, char, char).unwrap();
        let (a1, b1, c1, d1) = scanln_fmt!("  #{}#{}#{}#{}#", char, char, char, char).unwrap();
        let mut burrow = Burrow::default();
        burrow.fill(EMPTY);
        burrow[0] = a0 as u8 - 65;
        burrow[1] = a1 as u8 - 65;
        burrow[2] = b0 as u8 - 65;
        burrow[3] = b1 as u8 - 65;
        burrow[4] = c0 as u8 - 65;
        burrow[5] = c1 as u8 - 65;
        burrow[6] = d0 as u8 - 65;
        burrow[7] = d1 as u8 - 65;
        Self::from_burrow(burrow)
    }

    fn from_burrow(mut burrow: Burrow) -> Self {
        if ROOM_LENGTH == 4 {
            burrow.copy_within(7..=7, 15);
            burrow.copy_within(5..=6, 11);
            burrow.copy_within(3..=4, 7);
            burrow.copy_within(1..=2, 3);
            burrow[1] = 'D' as u8 - 65;
            burrow[2] = 'D' as u8 - 65;
            burrow[5] = 'C' as u8 - 65;
            burrow[6] = 'B' as u8 - 65;
            burrow[9] = 'B' as u8 - 65;
            burrow[10] = 'A' as u8 - 65;
            burrow[13] = 'A' as u8 - 65;
            burrow[14] = 'C' as u8 - 65;
        }
        let mut solved = Burrow::default();
        for x in 0 .. 4 {
            solved[x * ROOM_LENGTH .. (x + 1) * ROOM_LENGTH].fill(x as u8);
        }
        Game {
            burrow:     burrow,
            solved,
            visited:    HashMap::new(),
            min_cost:   0,
        }
    }

    // Calculate energy required to move from a to b separately,
    // keeps the main algorithms better readable.
    fn energy(&self, burrow: &Burrow, from: usize, to: usize) -> u32 {
        let e = 10u32.pow(burrow[from] as u32);
        let mut r = 0;

        // move into hallway.
        let src = if from < HALL_START {
            r += e * (1 + (from % ROOM_LENGTH) as u32);
            // in front of the current room.
            (from / ROOM_LENGTH) * 2 + HALL_START + 2
        } else {
            from
        };

        // move through hallway.
        let dest = if to < HALL_START {
            // in front of destination.
            (to / ROOM_LENGTH) * 2 + HALL_START + 2
        } else {
            to
        };
        r += e * (((src as i32) - (dest as i32)).abs() as u32);

        // move into room.
        if to < HALL_START {
            r += e * (1 + (to % ROOM_LENGTH) as u32);
        }

        r
    }

    // We can leave the room if:
    //
    // - this is our home room, but there's a different color amphipod behind us
    // - this is not our home room and there's noone in front of us.
    //
    #[inline(always)]
    fn can_leave_room(&self, burrow: &Burrow, pos: usize) -> bool {
        let color = burrow[pos];
        let already_home = color as usize == pos / ROOM_LENGTH;
        let room_base = (pos / ROOM_LENGTH) * ROOM_LENGTH;
        for x in room_base .. room_base + ROOM_LENGTH {
            if x < pos && burrow[x] != EMPTY {
                return false;
            }
            if x == pos && !already_home {
                return true;
            }
            if x > pos && burrow[x] != color {
                return true;
            }
        }
        false
    }

    // Calculate possible moves from any position to the home room.
    fn move_to_room(&self, burrow: &Burrow, curpos: usize) -> Option<(usize, usize)> {
        if burrow[curpos] == EMPTY {
            return None;
        }
        let color = burrow[curpos];
        let room_base = color as usize * ROOM_LENGTH;

        if curpos < HALL_START {
            // If we're already in the home room, don't move.
            if curpos / ROOM_LENGTH == color as usize {
                return None;
            }
            // Can we leave the room?
            if !self.can_leave_room(burrow, curpos) {
                return None;
            }
        }

        // First see if our home room is reachable.
        let src = if curpos < HALL_START {
            // hallway in front of current room.
            HALL_START + 2 + (curpos / ROOM_LENGTH) * 2
        } else {
            // hallway
            curpos
        };

        // dest is hallway in front of destination room.
        let dest = HALL_START + 2 + color as usize * 2;
        let s1 = cmp::min(src, dest);
        let s2 = cmp::max(src, dest);
        for x in s1 ..= s2 {
            if x != src && burrow[x] != EMPTY {
                return None;
            }
        }

        // Room must be empty, or contain only the same color amphipod.
        let mut x = room_base + ROOM_LENGTH;
        while x > room_base {
            x -= 1;
            if burrow[x] == EMPTY {
                return Some((curpos, x));
            }
            if burrow[x] != color {
                break;
            }
        }

        None
    }

    // Calculate possible moves from a room to positions in the hallway.
    fn move_to_hall(&self, burrow: &Burrow, pos: usize) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();

        if burrow[pos] == EMPTY {
            return moves;
        }

        if !self.can_leave_room(burrow, pos) {
            return moves;
        }

        // all positions in the hall that are reachable.
        let hpos = HALL_START + 2 + (pos / ROOM_LENGTH) * 2;
        let hs = HALL_START + 2;
        for n in (HALL_START .. hpos).rev() {
            if burrow[n] != EMPTY {
                break;
            }
            if n != hs && n != hs + 2 && n != hs + 4 && n != hs + 6 {
                moves.push((pos, n));
            }
        }
        for n in hpos + 1 .. HALL_END {
            if burrow[n] != EMPTY {
                break;
            }
            if n != hs && n != hs + 2 && n != hs + 4 && n != hs + 6 {
                moves.push((pos, n));
            }
        }
        moves
    }

    fn do_move(&mut self, mut burrow: Burrow, the_move: (usize, usize), mut cost: u32) {

        // Check if this move would make the solution too expensive.
        cost += self.energy(&burrow, the_move.0, the_move.1);
        if self.min_cost != 0 && cost >= self.min_cost {
            return;
        }

        burrow[the_move.1] = burrow[the_move.0];
        burrow[the_move.0] = EMPTY;

        // Check if we've already been here via a less expensive route.
        if let Some (&c) = self.visited.get(&burrow) {
            if c <= cost {
                return;
            }
        }

        self.try_all_next(burrow, cost);
    }

    fn try_all_next(&mut self, burrow: Burrow, cost: u32) {

        // Solved?
        if burrow[..HALL_START] == self.solved[..HALL_START] {
            if cost < self.min_cost || self.min_cost == 0 {
                self.min_cost = cost;
            }
            return;
        }
        self.visited.insert(burrow, cost);

        // anyone anywhere that can move into a room?
        for pos in 0 .. HALL_END {
            if let Some(next_move) = self.move_to_room(&burrow, pos) {
                self.do_move(burrow, next_move, cost);
            }
        }

        // try to move an amphipod into the hallway.
        for pos in 0 .. HALL_START {
            for &next_move in &self.move_to_hall(&burrow, pos) {
                self.do_move(burrow, next_move, cost);
            }
        }
    }
}

fn main() {
    let mut b = Game::<2, 8, 19>::from_stdin();
    let start = Instant::now();
    b.try_all_next(b.burrow, 0);
    println!("part1: min energy cost: {} ({:?})", b.min_cost, start.elapsed());

    let mut b = Game::<4, 16, 27>::from_burrow(b.burrow);
    let start = Instant::now();
    b.try_all_next(b.burrow, 0);
    println!("part2: min energy cost: {} ({:?})", b.min_cost, start.elapsed());
}

