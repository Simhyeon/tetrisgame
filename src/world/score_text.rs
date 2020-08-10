pub struct ScoreText {
    pub score: i32,
    pub score_text: String,
}

impl ScoreText {
    pub fn new() -> Self {
        Self { 
            score: 0,
            score_text: String::from("000000"),
        }
    }

    pub fn add_score(&mut self, amount: i32) {
        self.score += amount;
        self.score_text = format!("{:06}", self.score);
    }
}
