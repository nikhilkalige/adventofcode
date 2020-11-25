use std::io::{self, Read};
use std::str::FromStr;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let result_point = part1(&input);
    let result_steps = part2(&input);

    println!(
        "Closest point, x: {}, y: {}, dist: {}",
        result_point.x,
        result_point.y,
        result_point.manhattan_distance()
    );
    println!("Minimum steps: {}", result_steps);

    Ok(())
}

fn create_wires(input: &str) -> Vec<Wire> {
    let mut wires = Vec::<Wire>::new();
    for line in input.lines() {
        let wire: Wire = line.parse::<Wire>().unwrap();
        wires.push(wire);
    }

    if wires.len() != 2 {
        panic!("Expected 2 wires");
    }
    wires
}

fn part1(input: &str) -> Point {
    let wires = create_wires(input);

    let intersection_points = wires[0].intersections(&wires[1]);
    let result_point = closest_point(&intersection_points);
    result_point
}

fn part2(input: &str) -> u32 {
    let wires = create_wires(input);
    let intersection_points = wires[0].intersections(&wires[1]);
    let steps = intersection_points
        .iter()
        .map(|inter| steps(&wires[0], &wires[1], inter));

    return steps.filter(|e| *e != 0).min().unwrap();
}

#[derive(Debug, Clone)]
struct Wire {
    lines: Vec<Line>,
}

#[derive(Debug, Clone)]
struct Intersection {
    a: usize,
    b: usize,
    p: Point,
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance(self: &Point) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug, Clone)]
struct Line {
    a: Point,
    b: Point,
}

impl Line {
    fn coefficients(self: &Line) -> (i64, i64, i64) {
        let a = (self.a.y - self.b.y) as i64;
        let b = (self.b.x - self.a.x) as i64;
        let c = (self.b.x * self.a.y - self.a.x * self.b.y) as i64;
        return (a, b, c);
    }

    fn intersection(self: &Line, p2: &Line) -> Option<Point> {
        let (a1, b1, c1) = self.coefficients();
        let (a2, b2, c2) = p2.coefficients();

        let det_a = b2 * a1 - a2 * b1;

        if det_a == 0 {
            return None;
        }

        let det_a1 = b2 * c1 - b1 * c2;
        let det_a2 = c2 * a1 - c1 * a2;

        let p = Point {
            x: (det_a1 / det_a) as i32,
            y: (det_a2 / det_a) as i32,
        };

        if self.on_segment(&p) && p2.on_segment(&p) {
            return Some(p);
        }
        None
    }

    fn on_segment(self: &Line, point: &Point) -> bool {
        return (point.x <= std::cmp::max(self.a.x, self.b.x))
            && (point.x >= std::cmp::min(self.a.x, self.b.x))
            && (point.y <= std::cmp::max(self.a.y, self.b.y))
            && (point.y >= std::cmp::min(self.a.y, self.b.y));
    }

    fn length(self: &Line) -> u32 {
        let d = (self.a.x - self.b.x).abs() + (self.a.y - self.b.y).abs();
        d as u32
    }
}

impl Wire {
    fn intersections(self: &Wire, other: &Wire) -> Vec<Intersection> {
        let mut intersecion_points = Vec::<Intersection>::new();
        for (i, line_a) in self.lines.iter().enumerate() {
            for (j, line_b) in other.lines.iter().enumerate() {
                if let Some(point) = line_a.intersection(line_b) {
                    intersecion_points.push(Intersection {
                        a: i,
                        b: j,
                        p: point,
                    });
                }
            }
        }
        intersecion_points
    }
}

impl FromStr for Wire {
    type Err = i32;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines: Vec<Line> = Vec::new();
        let mut current_pos = Point { x: 0, y: 0 };

        let moves = s.split(',');
        for mv in moves {
            let bytes = mv.as_bytes();
            let code = bytes[0] as char;
            let change = mv[1..].parse::<i32>().unwrap();

            let mut new_pos = Point { x: 0, y: 0 };
            if code == 'R' {
                new_pos.x = current_pos.x + change;
                new_pos.y = current_pos.y;
            } else if code == 'L' {
                new_pos.x = current_pos.x - change;
                new_pos.y = current_pos.y;
            } else if code == 'U' {
                new_pos.x = current_pos.x;
                new_pos.y = current_pos.y + change;
            } else {
                new_pos.x = current_pos.x;
                new_pos.y = current_pos.y - change;
            }

            lines.push(Line {
                a: current_pos,
                b: new_pos,
            });

            current_pos = new_pos;
        }

        Ok(Wire { lines: lines })
    }
}

fn closest_point(points: &[Intersection]) -> Point {
    let distances = points.iter().map(|p| p.p.manhattan_distance());
    let result = distances
        .enumerate()
        .min_by_key(|(_, e)| if *e == 0 { i32::MAX } else { *e })
        .unwrap();
    points[result.0].p
}

fn steps(wa: &Wire, wb: &Wire, int: &Intersection) -> u32 {
    let mut steps: u32 = 0;
    for line in &wa.lines[..int.a] {
        steps += line.length();
    }
    steps += Line {
        a: wa.lines[int.a].a,
        b: int.p,
    }
    .length();
    for line in &wb.lines[..int.b] {
        steps += line.length();
    }
    steps += Line {
        a: wb.lines[int.b].a,
        b: int.p,
    }
    .length();

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coefficients() {
        let a = Line {
            a: Point { x: -5, y: 5 },
            b: Point { x: -2, y: -4 },
        };
        let b = Line {
            a: Point { x: 3, y: 2 },
            b: Point { x: 7, y: -4 },
        };

        assert_eq!(a.coefficients(), (9, 3, -30));
        assert_eq!(b.coefficients(), (6, 4, 26));
    }

    #[test]
    fn test_intersection() {
        let a = Line {
            a: Point { x: 4, y: 0 },
            b: Point { x: 6, y: 10 },
        };
        let b = Line {
            a: Point { x: 0, y: 3 },
            b: Point { x: 10, y: 7 },
        };
        let c = Line {
            a: Point { x: 4, y: 0 },
            b: Point { x: 6, y: 10 },
        };

        assert_eq!(a.intersection(&b).unwrap(), Point { x: 5, y: 5 });
        assert!(a.intersection(&c).is_none());
    }

    const INPUT1: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";

    const INPUT2: &str = "R8,U5,L5,D3
U7,R6,D4,L4";

    const INPUT3: &str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT1).manhattan_distance(), 159);
        assert_eq!(part1(&INPUT2).manhattan_distance(), 6);

        let input3 = "R1003,D138,L341,U798,L922,U153,R721,D177,L297,D559,L414,U470,L589,D179,L267,D954,R739,D414,L865,U688,R541,U242,R32,D607,L480,D401,L521,U727,L295,D154,R905,D54,L353,U840,L187,U942,R313,D143,R927,D962,R739,U152,R6,D9,L807,D67,R425,D235,L598,D107,L838,D522,L882,U780,L942,D29,R933,U129,L556,D11,L859,D455,L156,U673,L54,D141,R862,U88,R362,U742,L511,D408,R825,U622,R650,D393,L882,D969,R866,D232,L423,U371,L744,U35,L196,D189,R803,U663,R41,U741,R742,U929,L311,U30,R357,D776,L929,U85,R415,U540,R921,U599,R651,U79,R608,D620,L978,D92,L491,D310,L830,U656,R244,U72,L35,U768,R666,U356,R82,U596,L798,D455,L280,D626,R586,U668,R331,D245,L140,U3,R283,U813,R620,U975,L795,U477,L100,D94,R353,D732,R694,U702,L305,U497,R900,U810,L412,D954,R584,D444,L531,D875,R49,D328,L955,U227,L370,D548,L351,U571,R373,U743,R105,D226,L755,U325,R496,D960,L415,U262,R197,D508,R725,U930,L722,D162,L996,D610,R346,U680,L75,U211,R953,U147,R114,D48,L305,D284,L630,U575,R142,D518,R704,D820,L617,D118,R67,D674,L90,D916,L483,D598,L424,U92,R188,U413,L702,D262,R720,D995,L759,D732,L259,D814,L342,U642,L875,U726,R265,D143,R754,D235,L535,U1,R211,D720,R943,D726,L398,U636,R994,U653,L401,U877,R577,D460,L730,U889,R166,D641,L693,U490,L78,D80,R535,U551,L866,U283,L336,U586,L913,U474,R158,D220,R278,U11,R421,D661,R719,D696,R188,D735,L799,U391,R331,U581,R689,D82,R375,D125,R613,D705,L927,U18,R399,D352,L411,D777,L733,D884,R791,U973,R772,D878,R327,U215,L298,D360,R426,D872,L99,U78,L745,U59,L641,U73,L294,D247,R944,U512,L396
L1004,D252,L909,D935,R918,D981,L251,U486,R266,U613,L546,D815,L789,D692,L550,U633,R485,U955,R693,D784,R974,U529,R926,U550,L742,U88,R647,D572,R832,D345,R98,D122,R634,U943,L956,U551,R295,U122,L575,U378,R652,U97,R129,D872,R275,D492,L530,D328,R761,U738,R836,U884,R636,U776,L951,D977,R980,U526,L824,U125,R778,D818,R281,U929,R907,U234,L359,D521,R294,U137,L607,U421,L7,U582,R194,U979,L941,D999,R442,D330,L656,U410,R753,U704,R834,D61,R775,U750,R891,D989,R856,D944,R526,D44,R227,U938,R130,D280,L721,D171,R763,D677,L643,U931,L489,U250,L779,U552,R796,U220,R465,U700,L459,U766,R501,D16,R555,U257,R122,U452,L197,U905,L486,D726,L551,U487,R785,U470,L879,U149,L978,D708,R18,U211,L652,D141,L99,D190,L982,U556,R861,U745,L786,U674,R706,U986,R554,D39,R881,D626,R885,U907,R196,U532,L297,U232,L508,U283,L236,U613,L551,U647,R679,U760,L435,D475,R916,U669,R788,U922,R107,D503,R687,D282,L940,U835,L226,U421,L64,U245,R977,D958,L866,D328,R215,D532,R350,D199,R872,U373,R415,U463,L132,U225,L144,U786,R658,D535,R263,U263,R48,D420,L407,D177,L496,U521,R47,D356,L503,D557,R220,D879,L12,U853,R265,U983,L221,U235,R46,D906,L271,U448,L464,U258,R952,D976,L949,D526,L458,D753,L408,U222,R256,U885,R986,U622,R503,D5,L659,D553,R311,U783,L541,U17,R267,U767,L423,D501,R357,D160,L316,D912,R303,U648,L182,U342,L185,U743,L559,U816,R24,D203,R608,D370,R25,U883,L72,D816,L877,U990,R49,U331,L482,U37,L585,D327,R268,D106,L224,U401,L203,D734,L695,U910,L417,U105,R135,U876,L194,U723,L282,D966,R246,U447,R966,U346,L636,D9,L480,D35,R96";
        assert_eq!(part1(&input3).manhattan_distance(), 446);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT1), 610);
        assert_eq!(part2(&INPUT2), 30);
        assert_eq!(part2(&INPUT3), 410);
    }
}
