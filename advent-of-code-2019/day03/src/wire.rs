use common::grid::{Dir, Pos};

/// A wire segment is a straight part of a wire, described relatively to another segment.
#[derive(Debug, Eq, PartialEq)]
pub struct WireSegment {
    direction: Dir,
    length: u32,
}

impl WireSegment {
    pub fn parse(s: &str) -> Result<Self, &'static str> {
        let direction = match &s[0..1] {
            "U" => Dir::Up,
            "D" => Dir::Down,
            "L" => Dir::Left,
            "R" => Dir::Right,
            _ => return Err("invalid direction"),
        };
    
        let length = s[1..]
            .parse::<u32>()
            .map_err(|_| "could not parse input as integers")?;
    
        Ok(WireSegment { direction, length })
    }
}

/// A wire is represented as a sequence of the positions that it covers.
pub struct Wire {
    pub positions: Vec<Pos>,
}

impl Wire {
    /// Construct a wire from a series of wire segments, starting at position (0, 0).
    pub fn from_segments(segments: Vec<WireSegment>) -> Self {
        let mut positions = Vec::<Pos>::new();
        let mut curr = Pos::at(0, 0);

        positions.push(curr);

        for s in segments {
            for _ in 0..s.length {
                curr = curr.step(s.direction);

                positions.push(curr);
            }
        }

        Self { positions }
    }

    /// Returns the shortest amount of steps from the origin to the given
    /// position following the wire.
    pub fn shortest_length(&self, target: Pos) -> Option<u32> {
        // TODO this can likely be replaced by a simple find...
        self.positions
            .iter()
            .enumerate()
            .filter(|(_, &p)| p == target)
            .map(|(i, _)| i as u32)
            .min()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::grid::Dir::*;

    #[test]
    fn wire_segment_parse() {
        assert_eq!(WireSegment::parse("U2"), Ok(WireSegment { direction: Up, length: 2 }));
        assert_eq!(WireSegment::parse("D3"), Ok(WireSegment { direction: Down, length: 3 }));
        assert_eq!(WireSegment::parse("L12"), Ok(WireSegment { direction: Left, length: 12 }));
        assert_eq!(WireSegment::parse("R231"), Ok(WireSegment { direction: Right, length: 231 }));
    }

    #[test]
    fn wire_from_wire_segments() {
        let input = vec![
            WireSegment { direction: Up, length: 2 },
            WireSegment { direction: Left, length: 1 },
            WireSegment { direction: Down, length: 3 },
            WireSegment { direction: Right, length: 1 },
        ];

        let wire = Wire::from_segments(input);

        assert_eq!(wire.positions, vec![
            Pos::at(0, 0),
            Pos::at(0, 1), Pos::at(0, 2),
            Pos::at(-1, 2),
            Pos::at(-1, 1), Pos::at(-1, 0), Pos::at(-1, -1),
            Pos::at(0, -1),
        ]);
    }
}
