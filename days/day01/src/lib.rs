pub fn run_part_one(input: String) -> u32 {
    let mut ans = 0;
    for i in input.lines() {
        let mut done = false;
        for n in input.lines() {
            let num_one: u32 = i.parse().expect("Failed to parse first");
            let num_two: u32 = n.parse().expect("Failed to parse second");
            if num_one + num_two == 2020 {
                ans = num_one*num_two;
                done = true;
                break;
            }
        }
        if done {
            break;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_to() {
        let input = String::from("2000\n75\n43\n8\n20");
        let want = 40000;
        assert_eq!(want, run(input))

    }
}

