use std::f64::INFINITY;
use std::io::{self, Read};

const EPS: f64 = 1e-9;

#[derive(Clone, Copy, Debug)]
struct Pt {
    x: f64,
    y: f64,
}
impl Pt {
    fn sub(self, o: Pt) -> Pt { Pt { x: self.x - o.x, y: self.y - o.y } }
    fn add(self, o: Pt) -> Pt { Pt { x: self.x + o.x, y: self.y + o.y } }
    fn mul(self, k: f64) -> Pt { Pt { x: self.x * k, y: self.y * k } }
    fn dot(self, o: Pt) -> f64 { self.x * o.x + self.y * o.y }
    fn cross(self, o: Pt) -> f64 { self.x * o.y - self.y * o.x }
    fn norm(self) -> f64 { self.dot(self).sqrt() }
    fn dist(self, o: Pt) -> f64 { self.sub(o).norm() }
}

fn cross(a: Pt, b: Pt) -> f64 { a.cross(b) }

enum SegInter {
    None,
    Point(f64),       // t along AB in [0,1]
    Overlap(f64, f64) // t interval along AB
}

fn seg_seg_intersection(a: Pt, b: Pt, c: Pt, d: Pt) -> SegInter {
    let r = b.sub(a);
    let s = d.sub(c);
    let rxs = cross(r, s);
    let cma = c.sub(a);
    let qpxr = cross(cma, r);
    if rxs.abs() < EPS && qpxr.abs() < EPS {
        let rdotr = r.dot(r);
        if rdotr < EPS {
            // a==b
            return SegInter::None;
        }
        let t0 = cma.dot(r) / rdotr;
        let t1 = c.sub(a).add(s).dot(r) / rdotr; // (d - a) · r / |r|^2
        let (mut t0, mut t1) = if t0 <= t1 { (t0, t1) } else { (t1, t0) };
        let lo = t0.max(0.0);
        let hi = t1.min(1.0);
        if hi < lo + EPS {
            if (hi - lo).abs() < EPS && lo >= -EPS && lo <= 1.0 + EPS {
                return SegInter::Point(lo.clamp(0.0, 1.0));
            }
            return SegInter::None;
        }
        return SegInter::Overlap(lo, hi);
    }
    if rxs.abs() < EPS && qpxr.abs() >= EPS {
        return SegInter::None;
    }
    if rxs.abs() >= EPS {
        let t = cross(cma, s) / rxs;
        let u = cross(cma, r) / rxs;
        if t >= -EPS && t <= 1.0 + EPS && u >= -EPS && u <= 1.0 + EPS {
            return SegInter::Point(t.clamp(0.0, 1.0));
        }
    }
    SegInter::None
}

fn strictly_inside_convex(p: Pt, poly: &Vec<Pt>) -> bool {
    let n = poly.len();
    for i in 0..n {
        let a = poly[i];
        let b = poly[(i + 1) % n];
        let e = b.sub(a);
        let val = cross(e, p.sub(a));
        if val <= EPS { // boundary is not strictly inside
            return false;
        }
    }
    true
}

fn length_inside_segment_polygon(a: Pt, b: Pt, poly: &Vec<Pt>) -> f64 {
    let n = poly.len();
    let mut ts: Vec<f64> = Vec::new();
    ts.push(0.0);
    ts.push(1.0);
    for i in 0..n {
        let c = poly[i];
        let d = poly[(i + 1) % n];
        match seg_seg_intersection(a, b, c, d) {
            SegInter::None => {}
            SegInter::Point(t) => {
                if t > EPS && t < 1.0 - EPS {
                    ts.push(t);
                } else {
                    // Near endpoints 0 or 1; include if within [0,1]
                    ts.push(t.clamp(0.0, 1.0));
                }
            }
            SegInter::Overlap(t0, t1) => {
                // Add overlap endpoints clamped
                ts.push(t0.clamp(0.0, 1.0));
                ts.push(t1.clamp(0.0, 1.0));
            }
        }
    }
    ts.sort_by(|a, b| a.partial_cmp(b).unwrap());
    // dedupe
    let mut uniq: Vec<f64> = Vec::new();
    for t in ts {
        if uniq.is_empty() || (t - *uniq.last().unwrap()).abs() > 1e-10 {
            uniq.push(t);
        }
    }
    let u = b.sub(a);
    let full = u.norm();
    let mut inside_len = 0.0;
    for w in 0..uniq.len().saturating_sub(1) {
        let t0 = uniq[w];
        let t1 = uniq[w + 1];
        if t1 <= t0 + 1e-12 { continue; }
        let tmid = (t0 + t1) * 0.5;
        let pmid = a.add(u.mul(tmid));
        if strictly_inside_convex(pmid, poly) {
            inside_len += (t1 - t0) * full;
        }
    }
    inside_len
}

fn visible_point_from(P: Pt, Q: Pt, poly: &Vec<Pt>) -> bool {
    let n = poly.len();
    for i in 0..n {
        let a = poly[i];
        let b = poly[(i + 1) % n];
        match seg_seg_intersection(P, Q, a, b) {
            SegInter::None => {}
            SegInter::Point(t) => {
                if t > EPS && t < 1.0 - EPS {
                    return false;
                }
            }
            SegInter::Overlap(t0, _t1) => {
                if t0 > EPS && t0 < 1.0 - EPS {
                    // Overlap starts before Q, but this implies collinear along boundary.
                    // Treat as visible (movement along boundary is allowed).
                    // So do nothing.
                }
            }
        }
    }
    true
}

fn fully_visible_edge_from(P: Pt, a: Pt, b: Pt) -> bool {
    // For CCW polygon, interior is to the left of edge (a->b).
    // Fully visible if P is strictly to the right of the supporting line: cross(e, P-a) < 0
    cross(b.sub(a), P.sub(a)) < -EPS
}

fn main() {
    // Read input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let xs: f64 = it.next().unwrap().parse::<f64>().unwrap();
    let ys: f64 = it.next().unwrap().parse::<f64>().unwrap();
    let xe: f64 = it.next().unwrap().parse::<f64>().unwrap();
    let ye: f64 = it.next().unwrap().parse::<f64>().unwrap();
    let s = Pt { x: xs, y: ys };
    let e = Pt { x: xe, y: ye };
    let n: usize = it.next().unwrap().parse::<usize>().unwrap();
    let mut poly: Vec<Pt> = Vec::with_capacity(n);
    for _ in 0..n {
        let x: f64 = it.next().unwrap().parse::<f64>().unwrap();
        let y: f64 = it.next().unwrap().parse::<f64>().unwrap();
        poly.push(Pt { x, y });
    }

    // Precompute perimeter and prefix lengths
    let mut perim = 0.0;
    let mut pref: Vec<f64> = vec![0.0; n + 1];
    for i in 0..n {
        let a = poly[i];
        let b = poly[(i + 1) % n];
        let d = a.dist(b);
        perim += d;
        pref[i + 1] = perim;
    }

    let arc_ccw = |i: usize, j: usize| -> f64 {
        if j >= i {
            pref[j] - pref[i]
        } else {
            perim - (pref[i] - pref[j])
        }
    };

    let mut best = INFINITY;

    // Candidate 1: direct path
    let direct_inside = length_inside_segment_polygon(s, e, &poly);
    let direct = s.dist(e) + direct_inside; // sea cost + extra 1 per unit for inside
    if direct < best { best = direct; }

    // Candidate 2: around polygon along boundary via vertices
    // Precompute vertex visibility from S and E
    let mut vis_s: Vec<bool> = vec![false; n];
    let mut vis_e_vec: Vec<bool> = vec![false; n];
    for i in 0..n {
        let v = poly[i];
        vis_s[i] = visible_point_from(s, v, &poly);
        vis_e_vec[i] = visible_point_from(e, v, &poly);
    }
    for i in 0..n {
        if !vis_s[i] { continue; }
        for j in 0..n {
            if !vis_e_vec[j] { continue; }
            let path1 = s.dist(poly[i]) + arc_ccw(i, j) + e.dist(poly[j]);
            let path2 = s.dist(poly[i]) + (perim - arc_ccw(i, j)) + e.dist(poly[j]);
            let around = if path1 < path2 { path1 } else { path2 };
            if around < best { best = around; }
        }
    }

    // Candidate 3: through interior via boundary points on edges (continuous optimization)
    // Collect fully visible edges from S and from E
    let mut edges_s: Vec<(usize, Pt, Pt)> = Vec::new();
    let mut edges_e: Vec<(usize, Pt, Pt)> = Vec::new();
    for i in 0..n {
        let a = poly[i];
        let b = poly[(i + 1) % n];
        if fully_visible_edge_from(s, a, b) {
            edges_s.push((i, a, b));
        }
        if fully_visible_edge_from(e, a, b) {
            edges_e.push((i, a, b));
        }
    }

    // Edge-edge nested ternary
    let iters_outer = 50;
    let iters_inner = 50;
    for &(_is, as0, as1) in &edges_s {
        let es = as1.sub(as0);
        for &(_ie, bs0, bs1) in &edges_e {
            let ee = bs1.sub(bs0);
            // Nested ternary: minimize over s in [0,1], t in [0,1] of |S-A| + 2|A-B| + |B-E|
            let mut lo_s = 0.0;
            let mut hi_s = 1.0;
            for _ in 0..iters_outer {
                let m1 = (2.0 * lo_s + hi_s) / 3.0;
                let m2 = (lo_s + 2.0 * hi_s) / 3.0;
                let a1 = as0.add(es.mul(m1));
                let a2 = as0.add(es.mul(m2));
                // inner minimization over t
                let f1 = {
                    let mut lo_t = 0.0;
                    let mut hi_t = 1.0;
                    for _ in 0..iters_inner {
                        let n1 = (2.0 * lo_t + hi_t) / 3.0;
                        let n2 = (lo_t + 2.0 * hi_t) / 3.0;
                        let b1 = bs0.add(ee.mul(n1));
                        let b2 = bs0.add(ee.mul(n2));
                        let g1 = s.dist(a1) + 2.0 * a1.dist(b1) + b1.dist(e);
                        let g2 = s.dist(a1) + 2.0 * a1.dist(b2) + b2.dist(e);
                        if g1 < g2 {
                            hi_t = n2;
                        } else {
                            lo_t = n1;
                        }
                    }
                    let t = (lo_t + hi_t) * 0.5;
                    let b = bs0.add(ee.mul(t));
                    s.dist(a1) + 2.0 * a1.dist(b) + b.dist(e)
                };
                let f2 = {
                    let mut lo_t = 0.0;
                    let mut hi_t = 1.0;
                    for _ in 0..iters_inner {
                        let n1 = (2.0 * lo_t + hi_t) / 3.0;
                        let n2 = (lo_t + 2.0 * hi_t) / 3.0;
                        let b1 = bs0.add(ee.mul(n1));
                        let b2 = bs0.add(ee.mul(n2));
                        let g1 = s.dist(a2) + 2.0 * a2.dist(b1) + b1.dist(e);
                        let g2 = s.dist(a2) + 2.0 * a2.dist(b2) + b2.dist(e);
                        if g1 < g2 {
                            hi_t = n2;
                        } else {
                            lo_t = n1;
                        }
                    }
                    let t = (lo_t + hi_t) * 0.5;
                    let b = bs0.add(ee.mul(t));
                    s.dist(a2) + 2.0 * a2.dist(b) + b.dist(e)
                };
                if f1 < f2 {
                    hi_s = m2;
                } else {
                    lo_s = m1;
                }
            }
            let sopt = (lo_s + hi_s) * 0.5;
            let aopt = as0.add(es.mul(sopt));
            // inner again to get final
            let mut lo_t = 0.0;
            let mut hi_t = 1.0;
            for _ in 0..iters_inner {
                let n1 = (2.0 * lo_t + hi_t) / 3.0;
                let n2 = (lo_t + 2.0 * hi_t) / 3.0;
                let b1 = bs0.add(ee.mul(n1));
                let b2 = bs0.add(ee.mul(n2));
                let g1 = s.dist(aopt) + 2.0 * aopt.dist(b1) + b1.dist(e);
                let g2 = s.dist(aopt) + 2.0 * aopt.dist(b2) + b2.dist(e);
                if g1 < g2 {
                    hi_t = n2;
                } else {
                    lo_t = n1;
                }
            }
            let topt = (lo_t + hi_t) * 0.5;
            let bopt = bs0.add(ee.mul(topt));
            let val = s.dist(aopt) + 2.0 * aopt.dist(bopt) + bopt.dist(e);
            if val < best { best = val; }
        }
    }

    // Candidate 4: through interior with vertex-edge and edge-vertex combos
    for i in 0..n {
        if !vis_s[i] { continue; }
        let ai = poly[i];
        // ai fixed, minimize over edges visible from E
        for &(_ie, bs0, bs1) in &edges_e {
            let ee = bs1.sub(bs0);
            let mut lo_t = 0.0;
            let mut hi_t = 1.0;
            for _ in 0..60 {
                let n1 = (2.0 * lo_t + hi_t) / 3.0;
                let n2 = (lo_t + 2.0 * hi_t) / 3.0;
                let b1 = bs0.add(ee.mul(n1));
                let b2 = bs0.add(ee.mul(n2));
                let g1 = s.dist(ai) + 2.0 * ai.dist(b1) + b1.dist(e);
                let g2 = s.dist(ai) + 2.0 * ai.dist(b2) + b2.dist(e);
                if g1 < g2 { hi_t = n2; } else { lo_t = n1; }
            }
            let t = (lo_t + hi_t) * 0.5;
            let b = bs0.add(ee.mul(t));
            let val = s.dist(ai) + 2.0 * ai.dist(b) + b.dist(e);
            if val < best { best = val; }
        }
    }
    for j in 0..n {
        if !vis_e_vec[j] { continue; }
        let bj = poly[j];
        for &(_is, as0, as1) in &edges_s {
            let es = as1.sub(as0);
            let mut lo_s = 0.0;
            let mut hi_s = 1.0;
            for _ in 0..60 {
                let m1 = (2.0 * lo_s + hi_s) / 3.0;
                let m2 = (lo_s + 2.0 * hi_s) / 3.0;
                let a1 = as0.add(es.mul(m1));
                let a2 = as0.add(es.mul(m2));
                let f1 = s.dist(a1) + 2.0 * a1.dist(bj) + bj.dist(e);
                let f2 = s.dist(a2) + 2.0 * a2.dist(bj) + bj.dist(e);
                if f1 < f2 { hi_s = m2; } else { lo_s = m1; }
            }
            let sopt = (lo_s + hi_s) * 0.5;
            let a = as0.add(es.mul(sopt));
            let val = s.dist(a) + 2.0 * a.dist(bj) + bj.dist(e);
            if val < best { best = val; }
        }
    }

    // Candidate 5: through interior vertex-vertex
    for i in 0..n {
        if !vis_s[i] { continue; }
        for j in 0..n {
            if !vis_e_vec[j] { continue; }
            let val = s.dist(poly[i]) + 2.0 * poly[i].dist(poly[j]) + poly[j].dist(e);
            if val < best { best = val; }
        }
    }

    println!("{:.10}", best);
}