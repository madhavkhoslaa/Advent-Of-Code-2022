pub mod solution {
    pub fn main()-> Vec<i32>{
        let contents = crate::fs::read_to_string("./inputs/day1").expect("Cannot read file");
        let elves_calories : Vec<i32>= contents
            .split("\n\n")
            .into_iter()
            .map(|calorie|-> i32 {
                calorie
                    .split("\n")
                    .into_iter()
                    .map(|cal| -> i32 {
                        cal.parse::<i32>().unwrap_or(0)
                    })
                    .sum::<i32>()
            })
            .collect();
            return elves_calories;
    }

    pub fn solve()-> i32{
        return main().into_iter().sum();
    }


    pub fn solve2() -> i32 {
        let mut result = main();
        result.sort_by(|a, b| b.cmp(a));
        return result[0] + result[1] + result[2];
    }

}
