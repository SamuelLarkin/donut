// https://www.a1k0n.net/2011/07/20/donut-math.html
use std::f32::consts::PI;
use std::cmp::{
    max,
    min,
};

fn main() {
    const theta_spacing: usize = 7;
    const phi_spacing:   usize = 2;

    const R1: f32 = 1.;
    const R2: f32 = 2.;
    const K2: f32 = 5.;

    const screen_width:  usize = 80;
    const screen_height: usize = 22;

    const K1: f32 = screen_width as f32 * K2 * 3. / (8. * (R1 + R2));

    let mut A: f32=0.;
    let mut B: f32=0.;

    println!("TEST");
    print!("\x1b[2J");
    loop {
        let cosA: f32 = A.cos();
        let sinA: f32 = A.sin();
        let cosB: f32 = B.cos();
        let sinB: f32 = B.sin();
        let mut output  = [[' '; screen_height]; screen_width];
        let mut zbuffer = [[0.; screen_height]; screen_width];

        for theta in (0usize..628).step_by(theta_spacing) {
            let theta = theta as f32 / 100.;
            let costheta = theta.cos();
            let sintheta = theta.sin();

            for phi in (0usize..628).step_by(phi_spacing) {
                let phi = phi as f32 / 100.;
                let cosphi = phi.cos();
                let sinphi = phi.sin();

                // the x,y coordinate of the circle, before revolving (factored
                // out of the above equations)
                let circlex: f32 = R2 + R1 * costheta;
                let circley: f32 = R1 * sintheta;
                
                let x = circlex * (cosB*cosphi + sinA*sinB*sinphi) - circley*cosA*sinB;
                let y = circlex * (sinB*cosphi - sinA*cosB*sinphi) + circley*cosA*cosB;
                let z = K2 + cosA*circlex*sinphi + circley*sinA;
                let ooz = 1. / z;  // one over z

                let xp: usize = (screen_width/2) as usize + (K1*ooz*x) as usize;
                //println!("{:?} - {:?}", screen_height/2, K1*ooz*y);
                let yp: isize = (screen_height/2) as isize - (K1*ooz*y) as isize;
                let yp: usize = min((screen_height - 1) as isize, max(0isize, yp)) as usize;

                let L: f32 = cosphi*costheta*sinB
                    - cosA*costheta*sinphi
                    - sinA*sintheta
                    + cosB*(cosA*sintheta - costheta*sinA*sinphi);
                if L > 0. {
                    if ooz > zbuffer[xp][yp] {
                        zbuffer[xp][yp] = ooz;
                        let luminance_index = L as usize * 8;
                        output[xp][yp] = ".,-~:;=!*#$@"
                            .chars()
                            .nth(luminance_index)
                            .unwrap();
                    }
                }
            }
        }

        print!("\x1b[H");
        for j in 0..screen_height {
            for i in 0..screen_width {
                print!("{}", output[i][j]);
            }
            print!("\n");
        }

        A += 0.00004;
        B += 0.00002;
    };
}
