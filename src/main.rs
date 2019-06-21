use rand::prelude::*;
use rand::distributions::Open01;
use std::time::Instant;

fn main() {
    let h1 = vec![195, 204, 395, 409, 595, 618,795,836,995,1072,1195,1344,1395];

    let h2 = vec![10, 130,240,270,360,420,440,500,600,670,720,890,940,1060,1120,1130,1140,1400];
    println!("a");
    println!("{:#?}", dynamic(h2.clone()));
    println!("{:?}", recursive(h2.clone(), &mut vec![0], 0));
    let hotels = generate_hotels_std(100);
    let before = Instant::now();
    println!("Dynamic: {}", dynamic(hotels.clone()));
    let after = Instant::now();
    println!("{:#?}", after-before);
    let before1 = Instant::now();
    println!("Greedy: {}", greedy(hotels.clone(), &mut vec![0], 0).1);
    let after1 = Instant::now();
    println!("{:#?}", after1-before1)
}

fn generate_hotels(n: usize) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    let mut now: i64 = rng.gen_range(100,300);
    let mut rtn = vec![];
    for _ in 0..n {
        rtn.push(now);
        now += rng.gen_range(100,300);
    }
    return rtn
}

use num_traits::cast::ToPrimitive;
fn generate_hotels_std(n: usize) -> Vec<i64> {
    let mut x: f64 = 0.0;
    let mut y = 0;
    let mut rtn = vec![];
    for _ in 0..n {
        x = SmallRng::from_entropy().sample(Open01);
        y += (x*400.0).ceil().to_i64().unwrap();
        rtn.push(y);
    }
    return rtn
}

fn dynamic(h: Vec<i64>) -> i64 {
    let n    = h.len();
    let mut tmp   = h.clone();
    let mut start = vec![0];
    start.append(&mut tmp);
    // memoize points with dist < h[n]
    let mut dist: Vec<Vec<i64>> = vec![];
    for i in 0..n {
        dist.push(start[i..]
                  .into_iter()
                  .map(|x| (x-start[i]-200).pow(2))
                  .collect());
    }
   // optimize the memoized data -> find min
    for i in (0..n).rev() { // from location i
        for j in 1..dist[i].len() { // we go forward by j
            for k in 1..j { // or we go forward k and j-k
                if dist[i][k] + dist[i+k][j-k] < dist[i][j] {
                    dist[i][j] = dist[i][k]+dist[i+k][j-k]
                }
            }
        }
    }
    return dist[0][n]
}

fn cost(x: i64, total: i64) -> i64 {
    let tmp = 200-x.abs();
    return total + tmp*tmp
}

fn greedy(hotels: Vec<i64>, stops: &mut Vec<i64>, totalcost: i64) -> (Vec<i64>, i64) {
    if hotels.len() == 0 {
        return (stops.clone(), totalcost)
    }
    else {
        // We begin where we last stopped
        let here = stops[stops.len()-1];
        // Then we greedily pick one of the two locations closest to 200km away
        let mut endwhile = false;
        let mut index = 1;
        while !endwhile {
            if hotels[index]-here-200 < 0 {
                index += 1;
            }
            else {
                endwhile = true;
            }
            if index == hotels.len() {
                let last = hotels[index-1];
                stops.push(last);
                return greedy(vec![], stops, cost(last-here, totalcost));
            }
        }
        // If we can be done we push on to finish
        if index+1 == hotels.len() {
            let last = hotels[index];
            stops.push(last);
            return greedy(vec![], stops, cost(last-here, totalcost))
        }
        // But generally we just pick the better of the two options
        else {
            let short = (hotels[index]-here).abs();
            let long  = hotels[index+1]-here;
            if short >= long {
                stops.push(hotels[index+1]);
                return greedy(hotels[(index+1)..].to_vec(), stops, cost(long, totalcost))
            }
            else {
                stops.push(hotels[index]);
                return greedy(hotels[index..].to_vec(), stops, cost(short,totalcost))
            }
        }
    }
}


fn recursive(hotels: Vec<i64>, stops: &mut Vec<i64>, totalcost: i64) -> (Vec<i64>, i64) {
    if hotels.len() == 0 {
        return (stops.clone(), totalcost)
    }
    if hotels.len() == 1 {
        let endcost = cost(hotels[0]-stops[stops.len()-1], totalcost);
        stops.push(hotels[0]);
        return (stops.clone(), endcost)
    }
    else {
        let dist = hotels[0] - stops[stops.len()-1];
        let rest = hotels[1..].to_vec();
        let keep_going = recursive(rest.clone(), &mut stops.clone(), totalcost);
        stops.push(hotels[0]);
        let stop_here  = recursive(rest.clone(), &mut stops.clone(), cost(dist, totalcost));
        if keep_going.1 > stop_here.1 {
            return stop_here
        }
        else {
            return keep_going
        }
    }
}
