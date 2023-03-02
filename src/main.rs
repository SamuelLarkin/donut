// https://www.a1k0n.net/2011/07/20/donut-math.html
//use std::f32::consts::PI;
use std::cmp::{
    max,
    min,
};
use std::{
    thread,
    time::Duration,
};
//use term_size;


fn main() {
    const THETA_SPACING: usize = 7;  // 0.07
    const PHI_SPACING:   usize = 2;  // 0.02

    const R1: f32 = 1.;
    const R2: f32 = 2.;
    const K2: f32 = 8.;

    const SCREEN_WIDTH:  usize = 100;
    const SCREEN_HEIGHT: usize = 40;
    //let (SCREEN_WIDTH, SCREEN_HEIGHT) = term_size::dimensions().unwrap();

    //const K1: f32 = SCREEN_WIDTH as f32 * K2 * 3. / (19. * (R1 + R2));
    const K1: f32 = (SCREEN_WIDTH as f32 * K2  / (R1 + R2)) * 0.19;

    let mut A: f32=0.;
    let mut B: f32=0.;

    print!("\x1b[2J");
    //print!("\033[2J");
    loop {
        let cos_a: f32 = A.cos();
        let sin_a: f32 = A.sin();
        let cos_b: f32 = B.cos();
        let sin_b: f32 = B.sin();
        let mut output  = [[' '; SCREEN_HEIGHT]; SCREEN_WIDTH];
        let mut zbuffer = [[0.; SCREEN_HEIGHT]; SCREEN_WIDTH];

        for _theta in (0usize..628).step_by(THETA_SPACING) {
            let theta = _theta as f32 / 100.;
            let costheta = theta.cos();
            let sintheta = theta.sin();

            for _phi in (0usize..628).step_by(PHI_SPACING) {
                let phi = _phi as f32 / 100.;
                //println!("{:?} {:?}", theta, phi);
                let cosphi = phi.cos();
                let sinphi = phi.sin();

                // the x,y coordinate of the circle, before revolving (factored
                // out of the above equations)
                let circlex: f32 = R2 + R1 * costheta;
                let circley: f32 = R1 * sintheta;
                
                let x = circlex * (cos_b*cosphi + sin_a*sin_b*sinphi)
                    - circley*cos_a*sin_b;
                let y = circlex * (sin_b*cosphi - sin_a*cos_b*sinphi)
                    + circley*cos_a*cos_b;
                let z = K2 + cos_a*circlex*sinphi + circley*sin_a;
                let ooz = 1. / z;  // one over z

                let _xp: isize = (SCREEN_WIDTH/2) as isize + (K1*ooz*x) as isize;
                let xp: usize = min((SCREEN_WIDTH - 1) as isize, max(0isize, _xp)) as usize;
                let _yp: isize = (SCREEN_HEIGHT/2) as isize - (K1*ooz*y) as isize;
                let yp: usize = min((SCREEN_HEIGHT - 1) as isize, max(0isize, _yp)) as usize;

                let luminance: f32 = cosphi*costheta*sin_b
                    - cos_a*costheta*sinphi
                    - sin_a*sintheta
                    + cos_b*(cos_a*sintheta - costheta*sin_a*sinphi);
                if luminance > 0. {
                    if ooz > zbuffer[xp][yp] {
                        zbuffer[xp][yp] = ooz;
                        let luminance_index = (luminance * 8.) as usize;
                        output[xp][yp] = ".,-~:;=!*#$@"
                            .chars()
                            .nth(luminance_index)
                            .unwrap();
                    }
                }
            }
        }

        print!("\x1b[H");
        if true {
            for j in 0..SCREEN_HEIGHT {
                for i in 0..SCREEN_WIDTH {
                    print!("{}", output[i][j]);
                }
                print!("\n");
            }
        }

        A += 0.004;
        B += 0.002;

        thread::sleep(Duration::from_millis(40));
    };
}
