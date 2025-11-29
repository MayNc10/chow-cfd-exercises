pub fn runge(x: f32, p: f32, t: f32, h: f32, f: &dyn Fn(f32, f32, f32) -> f32) -> (f32, f32) {
    let d1x = h * p;
    let d1p = h * f(x, p, t);
    let d2x = h * (p + d1p/2.0);
    let d2p = h * f(x + d1x/2.0, p+d1p/2.0, t+h/2.0);
    let d3x = h * (p + d2p/2.0);
    let d3p = h * f(x + d2x/2.0, p + d2p/2.0, t + h/2.0);
    let d4x = h * (p + d3p);
    let d4p = h * f(x + d3x, p + d3p, t + h);
    (
        x + (d1x + 2.0 * d2x + 2.0 * d3x + d4x) / 6.0, 
        p + (d1p + 2.0 * d2p + 2.0 * d3p + d4p) / 6.0
    )
} 