pub mod solution {

    fn contains(pair1: String, pair2: String) -> bool {
        let _pair1: Vec<u8> = pair1
            .split("-")
            .map(|range| -> u8 { range.parse::<u8>().unwrap_or(0) })
            .collect();
        let _pair2: Vec<u8> = pair2
            .split("-")
            .map(|range| -> u8 { range.parse::<u8>().unwrap_or(0) })
            .collect();
        //check if pair1 contains pair 2
        if _pair1[1] >= _pair2[1] && _pair1[0] <= _pair2[0] {
            // println!("{:?}, {:?}, true", _pair1, _pair2);
            return true;
        }
        if _pair2[1] >= _pair1[1] && _pair2[0] <= _pair1[0] {
            // println!("{:?}, {:?}, true", _pair1, _pair2);
            return true;
        }
        // println!("{:?}, {:?}, false", _pair1, _pair2);
        return false;
    }
    pub fn solve1() -> usize {
        let contents = crate::fs::read_to_string("./inputs/day4").expect("Cannot read file");
        let mut ctr = 0;
        let range_pairs = contents.split("\n").collect::<Vec<&str>>();
        for pair in range_pairs {
            if pair != "" {
                let pair = pair.split(",").collect::<Vec<&str>>();
                let left_pair = pair[0];
                let right_pair = pair[1];
                if contains(String::from(left_pair), String::from(right_pair)) {
                    ctr = ctr + 1;
                }
            }
        }
        return ctr;
    }
}
