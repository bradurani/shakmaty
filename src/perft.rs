// This file is part of the shakmaty library.
// Copyright (C) 2017-2018 Niklas Fiekas <niklas.fiekas@backscattering.de>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use position::Position;
use movelist::MoveList;
use uci::uci;

/// Counts legal move paths of a given length.
///
/// Paths with mate or stalemate are not counted unless it occurs in the final
/// position. Useful for comparing, testing and debugging move generation
/// correctness and performance.
///
/// # Examples
///
/// ```
/// use shakmaty::Chess;
/// use shakmaty::perft;
///
/// let pos = Chess::default();
/// assert_eq!(perft(&pos, 1), 20);
/// assert_eq!(perft(&pos, 2), 400);
/// assert_eq!(perft(&pos, 3), 8902);
/// ```
pub fn perft<P: Position + Clone>(pos: &P, depth: u8) -> u64 {
    if depth < 1 {
        1
    } else {
        let mut moves = MoveList::new();
        pos.legal_moves(&mut moves);

        if depth == 1 {
            moves.len() as u64
        } else {
            moves.drain().map(|m| {
                let mut child = pos.clone();
                child.play_unchecked(&m);
                perft(&child, depth - 1)
            }).sum()
        }
    }
}

/// Like `perft()`, but also prints the perft of each child for debugging.
#[allow(unused)]
pub fn debug_perft<P: Position + Clone>(pos: &P, depth: u8) -> u64 {
    if depth < 1 {
        1
    } else {
        let mut moves = MoveList::new();
        pos.legal_moves(&mut moves);

        moves.iter().map(|m| {
            let child = pos.clone().play(m).expect("legal move");
            let nodes = perft(&child, depth - 1);
            let uci = uci(pos, m);
            println!("{} {} {}: {}", uci, m, depth - 1, nodes);
            nodes
        }).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use position::Chess;

    #[test]
    fn test_perft() {
        let pos = Chess::default();
        assert_eq!(perft(&pos, 0), 1);
        assert_eq!(perft(&pos, 1), 20);
    }
}
