pub struct Question {
    pub question: String,
    pub answers: Vec<String>,
    pub answer_index: usize,
}

pub struct Game {
    pub questions: Vec<Question>,
    pub score: u8,
    pub current_question_index: usize,
}

impl Question {
    pub fn new(question: String, answers: Vec<String>, answer_index: usize) -> Self {
        Question {
            question,
            answers,
            answer_index,
        }
    }
}

impl Game {
    pub fn new(questions: Vec<Question>) -> Self {
        Game {
            questions,
            score: 0,
            current_question_index: 0,
        }
    }

    /// Returns true if the quiz is over,
    /// if the quiz isn't over, it scores the current question and moves to the next
    pub fn next_question(self: &mut Game, answer: usize) -> bool {
        self.current_question_index += 1;

        if answer == self.questions[self.current_question_index - 1].answer_index {
            self.score += 1;
        }

        if self.current_question_index >= self.questions.len() {
            println!("You have reached the end of the quiz");
            return true;
        }

        return false;
    }
}
