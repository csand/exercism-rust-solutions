pub fn sum_of_multiples(max: u32, factors: &Vec<u32>) -> u32 {
    let mut multiples: Vec<u32> = factors.iter().fold(Vec::new(), |mut acc, n| {
        let mut m: Vec<u32> = (1..).take_while(|&x| n * x < max).map(|x| n * x).collect();
        acc.append(&mut m);
        acc
    });
    multiples.sort();
    multiples.dedup();
    multiples.iter().sum()
}
