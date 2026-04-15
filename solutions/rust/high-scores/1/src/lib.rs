#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        if self.scores.len() <= 3 {
            let mut top = self.scores.clone();
            top.sort();
            top.reverse();
            return top;
        }

        let mut top: Vec<u32> = vec![0; 3];

        for &score in &self.scores {
            if score > top[0] {
                top.rotate_right(1);
                top[0] = score;
            } else if score > top[1] {
                top.swap(2, 1);
                top[1] = score;
            } else if score > top[2] {
                top[2] = score;
            }
        }

        top
    }
}
