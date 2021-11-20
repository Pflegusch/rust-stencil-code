use std::cmp;
use std::time::Instant;
// use std::thread;
// use std::sync::{Arc, Mutex};
// use rayon::prelude::*;
// use packed_simd_2::*;


#[allow(dead_code)]
fn print_grid(n: usize, vec: &Vec<f32>) {
    for x in 0..n {
        for y in 0..n {
            print!("{} ", vec[x*n + y]);
        }
        print!("\n");
    }
    print!("\n");
}

fn test_grids(n: usize, vec_1: &Vec<f32>, vec_2: &Vec<f32>) -> bool {
    let epsilon: f32 = 0.000000001;
    for x in 0..n {
        for y in 0..n {
            if vec_1[x*n + y] - vec_2[x*n + y] > epsilon {
                return false
            }
        }
    }
    return true
}

fn initialize_grid(n: usize, seed_1: f32, seed_2: f32, vec: &mut Vec<f32>) {
    for x in 0..n {
        for y in 0..n {
            vec[x*n + y] = x as f32 % seed_1 + 0.1 * y as f32 * seed_2 - 0.6;        
        }
    }
}

fn transpose(n: usize, vec: &Vec<f32>) -> Vec<f32> {
    let mut transposed: Vec<f32> = vec![0.0; n*n];
    for x in 0..n {
        for y in 0..n {
            transposed[x*n + y] = vec[y*n + x];
        }
    }
    transposed
}

fn reference_solution(n: usize, k: i16, vec: &mut Vec<f32>) {
    let mut tmp: Vec<f32> = vec![0.0; n*n];

    for x in 0..n {
        for y in 0..n {
            let left_boundary: i16 = cmp::max(y as i16 - k, 0);
            let right_boundary: i16 = cmp::min(y as i16 + k, n as i16 - 1);
            let bottom_boundary: i16 = cmp::max(x as i16 - k, 0);
            let upper_boundary: i16 = cmp::min(x as i16 + k, n as i16 - 1);
            let neighbors_count: i16 = 
                upper_boundary - bottom_boundary + right_boundary - left_boundary;
            let mut sum: f32 = 0.0;

            for i in left_boundary..right_boundary + 1 {
                if i != y as i16 {
                    sum += vec[x*n + i as usize];
                }
            }

            for i in bottom_boundary..upper_boundary + 1 {
                if i != x as i16 {
                    sum += vec[i as usize *n + y];
                }
            }

            tmp[x*n + y] = sum / neighbors_count as f32;
        }
    }

    for x in 0..n {
        for y in 0..n {
            vec[x*n + y] = tmp[x*n + y];
        }
    }
}

fn vanilla_local_mean(n: usize, k: i16, vec: &mut Vec<f32>, transposed: &Vec<f32>) {
    let mut tmp: Vec<f32> = vec![0.0; n*n];

    for x in 0..n {
        for y in 0..n {
            let left_boundary: i16 = cmp::max(y as i16 - k, 0);
            let right_boundary: i16 = cmp::min(y as i16 + k, n as i16 - 1);
            let bottom_boundary: i16 = cmp::max(x as i16 - k, 0);
            let upper_boundary: i16 = cmp::min(x as i16 + k, n as i16 - 1);
            let neighbors_count: i16 = 
                upper_boundary - bottom_boundary + right_boundary - left_boundary;
            let mut sum: f32 = 0.0;

            for i in left_boundary..right_boundary + 1 {
                if i != y as i16 {
                    sum += vec[x*n + i as usize];
                }
            }

            for i in bottom_boundary..upper_boundary + 1 {
                if i != x as i16 {
                    sum += transposed[y*n + i as usize];
                }
            }

            tmp[x*n + y] = sum / neighbors_count as f32;
        }
    }

    for x in 0..n {
        for y in 0..n {
            vec[x*n + y] = tmp[x*n + y];
        }
    }
}

fn blocked_local_mean(n: usize, b: u8, k: i16, vec: &mut Vec<f32>, transposed: &Vec<f32>) {
    if n as u8 % b != 0{
        println!("N must be divisible through B");
        return;
    }

    let mut tmp: Vec<f32> = vec![0.0; n*n];
    
    for i in (0..n).step_by(b as usize) {
        for j in (0..n).step_by(b as usize) {
            for x in i..i+b as usize {
                for y in j..j+b as usize {
                    let left_boundary: i16 = cmp::max(y as i16 - k, 0);
                    let right_boundary: i16 = cmp::min(y as i16 + k, n as i16 - 1);
                    let bottom_boundary: i16 = cmp::max(x as i16 - k, 0);
                    let upper_boundary: i16 = cmp::min(x as i16 + k, n as i16 - 1);
                    let neighbors_count: i16 = 
                        upper_boundary - bottom_boundary + right_boundary - left_boundary;
                    let mut sum: f32 = 0.0;

                    for i in left_boundary..right_boundary + 1 {
                        if i != y as i16 {
                            sum += vec[x*n + i as usize];
                        }
                    }

                    for i in bottom_boundary..upper_boundary + 1 {
                        if i != x as i16 {
                            sum += transposed[y*n + i as usize];
                        }
                    }

                    tmp[x*n + y] = sum / neighbors_count as f32;
                }
            }
        }
    }

    for x in 0..n {
        for y in 0..n {
            vec[x*n + y] = tmp[x*n + y];
        }
    }
}

// fn blocked_vectorized_local_mean(n: usize, b: u8, k: i16, vec: &mut Vec<f32>, transposed: &Vec<f32>) {
//     if n as u8 % b != 0{
//         println!("N must be divisible through B");
//         return;
//     }

//     let mut tmp: Vec<f32> = vec![0.0; n*n];
    
//     for i in (0..n).step_by(b as usize) {
//         for j in (0..n).step_by(b as usize) {
//             for x in i..i+b as usize {
//                 for y in j..j+b as usize {
//                     let left_boundary: i16 = cmp::max(y as i16 - k, 0);
//                     let right_boundary: i16 = cmp::min(y as i16 + k, n as i16 - 1);
//                     let bottom_boundary: i16 = cmp::max(x as i16 - k, 0);
//                     let upper_boundary: i16 = cmp::min(x as i16 + k, n as i16 - 1);
//                     let neighbors_count: i16 = 
//                         upper_boundary - bottom_boundary + right_boundary - left_boundary;
//                     let mut sum: f32 = 0.0;

//                     // for (int i = left_boundary; i < y; i+=4) {
//                     //     Vec4f tmp;
//                     //     if (i + 4 < y)
//                     //         tmp.load(&vec[x*N + i]);
//                     //     else
//                     //         tmp.load_partial(y - i, &vec[x*N + i]);

//                     //     result_vec += tmp;
//                     // }

//                     for i in (left_boundary as usize..y).step_by(4) {
//                         if i + 4 < y {
//                             let tmp = f32x4::from_slice_aligned(&vec[x*n..x*n+i+4]);
//                             println!("Vec 0 if: {}", tmp.extract(0));
//                             println!("Vec 1 if: {}", tmp.extract(1));
//                             println!("Vec 2 if: {}", tmp.extract(2));
//                             println!("Vec 3 if: {}", tmp.extract(3));
//                         } else {
//                             let tmp = f32x4::from_slice_aligned(&vec[x*n..x*n+i+4]);
//                             println!("Vec 0 else: {}", tmp.extract(0));
//                             println!("Vec 1 else: {}", tmp.extract(1));
//                             println!("Vec 2 else: {}", tmp.extract(2));
//                             println!("Vec 3 else: {}", tmp.extract(3));
//                         }
//                     }
                    
//                 }
//             }
//         }
//     }

//     for x in 0..n {
//         for y in 0..n {
//             vec[x*n + y] = tmp[x*n + y];
//         }
//     }
// }

fn blocked_local_mean_multithreaded(n: usize, b: u8, k: i16, vec: &mut Vec<f32>, transposed: &Vec<f32>) {
    if n as u8 % b != 0{
        println!("N must be divisible through B");
        return;
    }

    let mut tmp: Vec<f32> = vec![0.0; n*n];

    for i in (0..n).step_by(b as usize) {
        for j in (0..n).step_by(b as usize) {

            for x in i..i+b as usize {
                for y in j..j+b as usize {

                    let left_boundary: i16 = cmp::max(y as i16 - k, 0);
                    let right_boundary: i16 = cmp::min(y as i16 + k, n as i16 - 1);
                    let bottom_boundary: i16 = cmp::max(x as i16 - k, 0);
                    let upper_boundary: i16 = cmp::min(x as i16 + k, n as i16 - 1);
                    let neighbors_count: i16 = 
                        upper_boundary - bottom_boundary + right_boundary - left_boundary;
                    let mut sum: f32 = 0.0;

                    for i in left_boundary..right_boundary + 1 {
                        if i != y as i16 {
                            sum += vec[x*n + i as usize];
                        }
                    }

                    for i in bottom_boundary..upper_boundary + 1 {
                        if i != x as i16 {
                            sum += transposed[y*n + i as usize];
                        }
                    }

                    tmp[x*n + y] = sum / neighbors_count as f32;
                }
            }
        }
    }

    for x in 0..n {
        for y in 0..n {
            vec[x*n + y] = tmp[x*n + y];
        }
    }
}

fn main() {
    let n: usize = 4;
    let k: i16 = 4;
    let b: u8 = 2;

    let mut reference: Vec<f32> = vec![0.0; n*n];
    let mut vanilla: Vec<f32> = vec![0.0; n*n];
    let mut blocked: Vec<f32> = vec![0.0; n*n];
    let mut vectorized: Vec<f32> = vec![0.0; n*n];

    let seed_1: f32 = 5.0;
    let seed_2: f32 = 0.125;
    initialize_grid(n, seed_1, seed_2, &mut reference);
    initialize_grid(n, seed_1, seed_2, &mut vanilla);
    initialize_grid(n, seed_1, seed_2, &mut blocked);
    initialize_grid(n, seed_1, seed_2, &mut vectorized);

    let vanilla_transposed: Vec<f32> = transpose(n, &vanilla);
    let blocked_transposed: Vec<f32> = transpose(n, &blocked);
    let vectorized_transposed: Vec<f32> = transpose(n, &vectorized);

    let mut now = Instant::now();
    reference_solution(n, k, &mut reference);
    println!("Reference:     {}ms", now.elapsed().as_millis());

    now = Instant::now();
    vanilla_local_mean(n, k, &mut vanilla, &vanilla_transposed);
    println!("Vanilla:       {}ms", now.elapsed().as_millis());

    now = Instant::now();
    blocked_local_mean(n, b, k, &mut blocked, &blocked_transposed);
    println!("Blocked:       {}ms", now.elapsed().as_millis());

    //print_grid(n, &vectorized);
    now = Instant::now();
    //blocked_vectorized_local_mean(n, b, k, &mut vectorized, &vectorized_transposed);
    println!("Vectorized:    {}ms", now.elapsed().as_millis());

    now = Instant::now();
    blocked_local_mean_multithreaded(n, b, k, &mut vectorized, &vectorized_transposed);
    println!("Multithreaded: {}ms",now.elapsed().as_millis());

    assert_eq!(test_grids(n, &reference, &vanilla), true);
    assert_eq!(test_grids(n, &reference, &blocked), true);
    assert_eq!(test_grids(n, &reference, &vectorized), true); //todo true
    // assert_eq!(test_grids(n, &reference, &multithreaded), true);
}