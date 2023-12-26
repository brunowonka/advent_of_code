use std::str::FromStr;

use itertools::Itertools;
const INPUT: &str = std::include_str!("input/day24.txt");

type Vector = euclid::Vector3D<f64, ()>;

struct Hail {
    pos: Vector,
    speed: Vector,
}

impl Hail {
    fn intersect2d(&self, other: &Hail) -> Option<euclid::Vector2D<f64, ()>> {
        // Ps + Vs*Ts = Po + Vo*To
        // P 18098= Ps - Po
        // P = Vo*To - Vs*Ts
        // To = (Px + Vsx*Ts)/Vox
        // Ts*Vsy = (Voy*To - Py)
        // Ts*Vsy = Voy * (Px + Vsx * Ts) / Vox - Py
        // Ts* (Vsy - Voy * Vsx / Vox) =  Voy * Px / Vox - Py
        let p = self.pos - other.pos;
        let ts = (other.speed.y * p.x / other.speed.x - p.y)
            / (self.speed.y - other.speed.y * self.speed.x / other.speed.x);
        if ts < 0.0 {
            return None;
        }
        let to = (p.x + self.speed.x * ts) / other.speed.x;
        if to < 0.0 {
            return None;
        }
        let end = self.pos + self.speed * ts;
        Some(end.xy())
    }
}

fn input(input: &str) -> impl Iterator<Item = Hail> + '_ {
    fn parse_vec(input: &str) -> Vector {
        let (x, r) = input.split_once(",").unwrap();
        let (y, z) = r.split_once(",").unwrap();
        let x = x.trim();
        let y = y.trim();
        let z = z.trim();
        Vector::new(
            f64::from_str(x).unwrap(),
            f64::from_str(y).unwrap(),
            f64::from_str(z).unwrap(),
        )
    }
    input.lines().map(|l| {
        let (p, s) = l.split_once(" @ ").unwrap();
        Hail {
            pos: parse_vec(p),
            speed: parse_vec(s),
        }
    })
}

#[test]
fn part_1() {
    let lo = 200000000000000.0;
    let hi = 400000000000000.0;
    let hail = input(INPUT)
        .map(|mut hail| {
            hail.pos.z = 0.0;
            hail.speed.z = 0.0;
            hail
        })
        .collect::<Vec<_>>();
    let ans = hail
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| {
            a.intersect2d(b)
                .map(|p| p.x >= lo && p.y >= lo && p.x <= hi && p.y <= hi)
                .unwrap_or(false)
        })
        .count();
    println!("day 24 part 1 = {ans}");
}

#[test]
fn part_2() {
    // Solve with equations.
    // 3 hailstones are selected, each one is Vi.
    // Vs * Ti + Ps - Vi * Ti - Pi = 0
    // We have 9 variables, Ps..., Vs... T1, T2, T3.
    type SVector = nalgebra::base::SVector<f64, 9>;
    type Matrix = nalgebra::base::SMatrix<f64, 9, 9>;

    let hail = input(INPUT).collect::<Vec<_>>();

    // Pick some different hail that makes the calculated error lower.
    let hail_delta = 3;
    let f = |v: SVector| {
        let mut o = SVector::default();
        for i in 0..3 {
            let ps = v.index(i);
            let vs = v.index(3 + i);
            for j in 0..3 {
                let ti = v.index(6 + j);
                let h = &hail[j + hail_delta];
                let pi = h.pos.to_array()[i];
                let vi = h.speed.to_array()[i];
                *o.index_mut(3 * j + i) = vs * ti + ps - vi * ti - pi
            }
        }
        o
    };

    // Jacobian of f.
    //
    // [1] [Ti] [vs - vi]
    let j = |v: SVector| {
        let mut o = Matrix::default();
        for i in 0..3 {
            let vs = v.index(3 + i);
            for j in 0..3 {
                let ti = v.index(6 + j);
                let h = &hail[j + hail_delta];
                let vi = h.speed.to_array()[i];

                let row = 3 * j + i;
                *o.index_mut((row, i)) = 1.;
                *o.index_mut((row, i + 3)) = *ti;
                *o.index_mut((row, 6 + j)) = vs - vi;
                // *o.index_mut((i, row)) = 1.;
                // *o.index_mut((i + 3, row)) = *ti;
                // *o.index_mut((6 + j, row)) = vs - vi;
            }
        }
        o
    };
    let solution = eqsolver::multivariable::MultiVarNewton::new(f, j)
        .with_tol(1e-9)
        .solve(SVector::from_vec(vec![
            20., -20., 20., 5., -2., 20., 10., 20., 30.,
        ]));

    println!("solution = {solution:?}");
    let solution = solution.expect("bad solution");
    let p = Vector::new(*solution.index(0), *solution.index(1), *solution.index(2));
    let v = Vector::new(*solution.index(3), *solution.index(4), *solution.index(5));
    let t = [*solution.index(6), *solution.index(7), *solution.index(8)];
    for i in 0..3 {
        let hail = &hail[hail_delta + i];
        let p = p + v * t[i] - (hail.pos + hail.speed * t[i]);
        // Print to choose hail delta we want error to be 0 here.
        // An alternative would be to use combinations of hail instead and pick
        // the one with the smallest error. But, hey, this works.
        println!("{i} => {}", p.length());
    }
    let ans = p.x + p.y + p.z;
    println!("day 24 part 2 = {ans}");
}
