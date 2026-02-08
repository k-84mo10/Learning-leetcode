impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut secret_digit: Vec<usize> = secret.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
        let mut guess_digit: Vec<usize> = guess.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();

        let mut bulls = 0;
        let mut cows = 0;
        let mut secret_map = vec![0; 10];
        let mut guess_map = vec![0; 10];

        for (&s, &g) in secret_digit.iter().zip(guess_digit.iter()) {
            if s == g { 
                bulls += 1; 
            } else {
                secret_map[s] += 1;
                guess_map[g] += 1;
            }
        }

        for i in 0..10 {
            cows += secret_map[i].min(guess_map[i]);
        }

        format!("{}A{}B", bulls, cows)
    }
}