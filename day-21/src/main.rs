use scan_fmt::*;

#[derive(Default, Debug, Clone, Copy)]
struct Player {
    pos:    u32,
    score:  u32,
}

fn part1(mut players: [Player; 2]) {
    let mut roll = 0;
'ROLL:
    loop {
        for p in 0 ..= 1 {
            roll += 3;
            let die = 3 * (roll - 1);
            players[p].pos = (players[p].pos + die - 1) % 10 + 1;
            players[p].score += players[p].pos;
            if players[p].score >= 1000 {
                println!("part1: {}", roll * players[1-p].score);
                break 'ROLL;
            }
        }
    }
}

fn play(players: [Player; 2], p: usize, univ: u64, wins: &mut [u64]) {
    const UNIV_WEIGHT: [u64; 10] = [ 0, 0, 0, 1, 3, 6, 7, 6, 3, 1 ];

    for d in 3 ..= 9 {
        let nuniv = univ * UNIV_WEIGHT[d as usize];
        let npos = ((players[p].pos + d - 1) % 10) + 1;
        let mut nplayers = players.clone();
        nplayers[p].pos = npos;
        nplayers[p].score = players[p].score + npos;
        if nplayers[p].score >= 21 {
            wins[p] += nuniv;
        } else {
            play(nplayers, 1 - p, nuniv, wins);
        }
    }
}

fn part2(players: [Player; 2]) {
    let mut wins = [0u64; 2];
    play(players, 0, 1, &mut wins);
    let winning = std::cmp::max(wins[0], wins[1]);
    println!("part2: player1: {}, player2: {}, highest: {}", wins[0], wins[1], winning);
}

fn main() {
    let p1 = scanln_fmt!("Player 1 starting position: {}", u32).unwrap();
    let p2 = scanln_fmt!("Player 2 starting position: {}", u32).unwrap();
    let players = [ Player{ pos: p1, score: 0}, Player{ pos: p2, score: 0 } ];

    part1(players);
    part2(players);
}
