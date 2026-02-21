// ALGORITHM A.2.1 int FindSpan(n,p,u,U) p.
// n - set as Max index of U
// p - degree
// u - input value along the curve/knot vector
// U - Knot vector
// Returns - span index of u
pub fn find_span_knot(degree: u32, knots: &Vec<u32>, knot: u32) -> u32 {
    let n = knots.len() as u32 - degree - 2;
    if knot >= knots[n as usize + 1] {
        n
    } else {
        if knot <= knots[degree as usize] {
            degree
        } else {
            let mut low = degree;
            let mut high = n + 1;
            let mut mid = (low + high) / 2;
            while (knot < knots[mid as usize] || knot >= knots[mid as usize + 1]) && low < high {
                if knot < knots[mid as usize] {
                    high = mid;
                } else {
                    low = mid;
                }
                mid = (low + high) / 2;
            }
            mid
        }
    }
}

pub fn find_span_param(degree: u32, knots: &Vec<f64>, param: f64, tolerance: f64) -> u32 {
    let n = knots.len() as u32 - degree - 2;
    if param >= knots[n as usize + 1] - tolerance {
        n
    } else {
        if param <= knots[degree as usize] + tolerance {
            degree
        } else {
            let mut low = degree;
            let mut high = n + 1;
            let mut mid = (low + high) / 2;
            while (param < knots[mid as usize] - tolerance || param >= knots[mid as usize + 1] - tolerance) && low < high {
                if param < knots[mid as usize] - tolerance {
                    high = mid;
                } else {
                    low = mid;
                }
                mid = (low + high) / 2;
            }
            mid
        }
    }
}

pub fn find_start_knot(degree: u32, knots: &Vec<u32>, knot: u32) -> u32 {
    let n = knots.len() as u32 - degree - 1;
    if knot >= knots[n as usize + 1] {
        n
    } else {
        if knot <= knots[degree as usize] {
            0
        } else {
            let mut low = degree + 1;
            let mut high = n - 1;
            let mut mid = (low + high) / 2;
            while !(knot > knots[(mid - 1) as usize] && knot <= knots[mid as usize]) {
                if knot <= knots[mid as usize] {
                    high = mid;
                } else if knot >= knots[mid as usize + 1] {
                    low = mid + 1;
                } else {
                    low = mid;
                }
                mid = (low + high) / 2;
            }
            mid
        }
    }
}

pub fn find_start_param(degree: u32, knots: &Vec<f64>, param: f64, tolerance: f64) -> u32 {
    let n = knots.len() as u32 - degree - 1;
    if param >= knots[n as usize + 1] - tolerance {
        n
    } else {
        if param <= knots[degree as usize] + tolerance {
            0
        } else {
            let mut low = degree + 1;
            let mut high = n - 1;
            let mut mid = (low + high) / 2;
            while !(param > knots[(mid - 1) as usize] + tolerance && param <= knots[mid as usize] + tolerance) && low < high {
                if param <= knots[mid as usize] + tolerance {
                    high = mid;
                } else if param >= knots[mid as usize + 1] - tolerance {
                    low = mid + 1;
                } else {
                    low = mid;
                }
                mid = (low + high) / 2;
            }
            mid
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::knot_utility_functions::find_span_knot;
    use crate::knot_utility_functions::find_span_param;
    use crate::knot_utility_functions::find_start_knot;
    use crate::knot_utility_functions::find_start_param;
    
    static K_TOLERANCE: f64 = f64::EPSILON;

    #[test]
    fn test_find_span_knot_min() {
        let degree = 2;
        let knots: Vec<u32> = vec![0, 0, 0, 1, 2, 2, 2];
        let ret = find_span_knot(degree, &knots, 0);
        assert_eq!(ret, 2);
    }
    
    #[test]
    fn test_find_span_knot_mid() {
        let degree = 2;
        let knots: Vec<u32> = vec![0, 0, 0, 1, 2, 2, 2];
        let ret = find_span_knot(degree, &knots, 1);
        assert_eq!(ret, 3);
    }
    
    #[test]
    fn test_find_span_knot_max() {
        let degree = 2;
        let knots: Vec<u32> = vec![0, 0, 0, 1, 2, 2, 2];
        let ret = find_span_knot(degree, &knots, 2);
        assert_eq!(ret, 3);
        let ret = find_span_knot(degree, &knots, 3);
        assert_eq!(ret, 3);
    }
    
    #[test]
    fn test_find_span_param_min() {
        let degree = 2;
        let knots: Vec<f64> = vec![0., 0., 0., 1., 2., 2., 2.];
        let ret = find_span_param(degree, &knots, -1., K_TOLERANCE);
        assert_eq!(ret, 2);
        let ret = find_span_param(degree, &knots, 0., K_TOLERANCE);
        assert_eq!(ret, 2);
    }
    
    #[test]
    fn test_find_span_param_mid() {
        let degree = 2;
        let knots: Vec<f64> = vec![0., 0., 0., 1., 2., 2., 2.];
        let ret = find_span_param(degree, &knots, 0.5, K_TOLERANCE);
        assert_eq!(ret, 2);
        let ret = find_span_param(degree, &knots, 1., K_TOLERANCE);
        assert_eq!(ret, 3);
        let ret = find_span_param(degree, &knots, 1.5, K_TOLERANCE);
        assert_eq!(ret, 3);
    }
    
    #[test]
    fn test_find_span_param_max() {
        let degree = 2;
        let knots: Vec<f64> = vec![0., 0., 0., 1., 2., 2., 2.];
        let ret = find_span_param(degree, &knots, 2., K_TOLERANCE);
        assert_eq!(ret, 3);
        let ret = find_span_param(degree, &knots, 3., K_TOLERANCE);
        assert_eq!(ret, 3);
    }
    
    #[test]
    fn test_find_start_knot() {
        let degree = 3;
        let knots: Vec<u32> = vec![0, 0, 0, 0, 1, 2, 2, 2, 3, 3, 4, 4, 4, 4];
        let ret = find_start_knot(degree, &knots, 0);
        assert_eq!(ret, 0);
        let ret = find_start_knot(degree, &knots, 1);
        assert_eq!(ret, 4);
        let ret = find_start_knot(degree, &knots, 2);
        assert_eq!(ret, 5);
        let ret = find_start_knot(degree, &knots, 3);
        assert_eq!(ret, 8);
        let ret = find_start_knot(degree, &knots, 4);
        assert_eq!(ret, 10);
        let ret = find_start_knot(degree, &knots, 5);
        assert_eq!(ret, 10);
    }
    
    #[test]
    fn test_find_start_param() {
        let degree = 3;
        let knots: Vec<f64> = vec![0., 0., 0., 0., 1., 2., 2., 2., 3., 3., 4., 4., 4., 4.];
        let ret = find_start_param(degree, &knots, -1., K_TOLERANCE);
        assert_eq!(ret, 0);
        let ret = find_start_param(degree, &knots, 0., K_TOLERANCE);
        assert_eq!(ret, 0);
        let ret = find_start_param(degree, &knots, 1., K_TOLERANCE);
        assert_eq!(ret, 4);
        let ret = find_start_param(degree, &knots, 2., K_TOLERANCE);
        assert_eq!(ret, 5);
        let ret = find_start_param(degree, &knots, 3., K_TOLERANCE);
        assert_eq!(ret, 8);
        let ret = find_start_param(degree, &knots, 4., K_TOLERANCE);
        assert_eq!(ret, 10);
        let ret = find_start_param(degree, &knots, 5., K_TOLERANCE);
        assert_eq!(ret, 10);
    }
}