#[allow(dead_code)]

pub mod solution {

    fn diff_char_calculator(substr1: String, substr2: String) -> usize {
        let mut countmap: Vec<u8> = vec![0; 124];
        // 65-90 => A-Z
        // 97-122 => a-z
        // assumes both strings to be equilength

        for current_char in substr1.chars().into_iter() {
            for current_char2 in substr2.chars().into_iter() {
                if current_char == current_char2 {
                    countmap[current_char as usize] = countmap[current_char as usize] + 1;
                }
            }
        }
        let mut ctr = 0;
        loop {
            if countmap[ctr] > 0 {
                if ctr >= 97 && ctr <= 122 {
                    return ctr as usize - 96;
                }
                if ctr >= 65 && ctr <= 90 {
                    return ctr as usize - 38;
                }
            }
            if ctr == 123 {
                return 0;
            }
            ctr += 1;
        }
    }
    fn diff_char_calculator2(substr1: String, substr2: String, substr3: String) -> usize {
        let mut countmap: Vec<u8> = vec![0; 124];
        // 65-90 => A-Z
        // 97-122 => a-z
        // assumes both strings to be equilength

        for current_char in substr1.chars().into_iter() {
            for current_char2 in substr2.chars().into_iter() {
                for current_char3 in substr3.chars().into_iter() {
                    if current_char == current_char2 && current_char2 == current_char3 {
                        countmap[current_char as usize] = countmap[current_char as usize] + 1;
                    }
                }
            }
        }
        let mut ctr = 0;
        loop {
            if countmap[ctr] > 0 {
                if ctr >= 97 && ctr <= 122 {
                    return ctr as usize - 96;
                }
                if ctr >= 65 && ctr <= 90 {
                    return ctr as usize - 38;
                }
            }
            if ctr == 123 {
                return 0;
            }
            ctr += 1;
        }
    }
    pub fn solve1() -> usize {
        let contents = crate::fs::read_to_string("./inputs/day3").expect("Cannot read file");
        return contents
            .split("\n")
            .into_iter()
            .map(|row| {
                let len = row.len();
                let substr1 = &row[0..len / 2];
                let substr2 = &row[len / 2..];
                let diff: usize =
                    diff_char_calculator(String::from(substr1), String::from(substr2));
                return diff;
                //return a list of diff
            })
            .sum::<usize>();
    }
    pub fn solve2() -> usize {
        // let contents: Vec<String> = crate::fs::read_to_string("./inputs/day3").expect("Cannot read file").split("\n").collect();
        // let mut ctr: usize = 0;
        // loop {
        //     diff_char_calculator2(contents[ctr], contents[ctr+1], contents[ctr+2]);
        //     ctr = ctr+ 3;
        //     if ctr <= contents.len() -3 {
        //         break;
        //     }
        // }
        // println!("todo day 3 part 2");
        return 1;
    }
}
// vJrwpWtwJgWr hcsFMMfFFhFp
