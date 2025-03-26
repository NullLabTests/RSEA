use rand::Rng;
use std::collections::HashMap;

struct LearningAgent {
    q_table: HashMap<(i32, i32, i32), f64>,
    learning_rate: f64,
    discount_factor: f64,
    exploration_rate: f64,
    task: String,
    mastery_threshold: f64,
}

impl LearningAgent {
    fn new() -> Self {
        LearningAgent {
            q_table: HashMap::new(),
            learning_rate: 0.1,
            discount_factor: 0.9,
            exploration_rate: 0.3,
            task: String::from("addition"),
            mastery_threshold: 0.9,
        }
    }

    fn choose_action(&mut self, a: i32, b: i32) -> i32 {
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < self.exploration_rate {
            rng.gen_range(0..21)
        } else {
            let mut best_action = 0;
            let mut best_value = f64::MIN;
            for action in 0..21 {
                let value = self.q_table.get(&(a, b, action)).unwrap_or(&0.0);
                if *value > best_value {
                    best_value = *value;
                    best_action = action;
                }
            }
            best_action
        }
    }

    fn update_q_table(&mut self, a: i32, b: i32, action: i32, reward: f64, next_a: i32, next_b: i32) {
        let old_value = *self.q_table.get(&(a, b, action)).unwrap_or(&0.0);
        let next_max = (0..21).map(|act| *self.q_table.get(&(next_a, next_b, act)).unwrap_or(&0.0)).fold(f64::MIN, f64::max);
        let new_value = old_value + self.learning_rate * (reward + self.discount_factor * next_max - old_value);
        self.q_table.insert((a, b, action), new_value);
    }

    fn get_reward(&self, a: i32, b: i32, action: i32) -> f64 {
        if self.task == "addition" {
            if action == a + b { 1.0 } else { -0.1 }
        } else if self.task == "multiplication" {
            if action == a * b { 1.0 } else { -0.1 }
        } else {
            0.0
        }
    }

    fn check_mastery(&self, recent_rewards: &[f64]) -> bool {
        let avg_reward = recent_rewards.iter().sum::<f64>() / recent_rewards.len() as f64;
        avg_reward > self.mastery_threshold
    }

    fn evolve(&mut self) {
        if self.task == "addition" {
            println!("Mastered addition! Evolving to multiplication...");
            self.task = String::from("multiplication");
            self.q_table.clear();
            self.exploration_rate = 0.5;
        } else {
            println!("Already at max task complexity, resetting to addition...");
            self.task = String::from("addition");
            self.q_table.clear();
            self.exploration_rate = 0.3;
        }
    }
}

fn main() {
    let mut agent = LearningAgent::new();
    let mut recent_rewards = Vec::new();
    let mut rng = rand::thread_rng();
    let mut episode = 0;

    loop {
        let a = rng.gen_range(1..11);
        let b = rng.gen_range(1..11);

        let action = agent.choose_action(a, b);
        let reward = agent.get_reward(a, b, action);

        agent.update_q_table(a, b, action, reward, a, b);

        recent_rewards.push(reward);
        if recent_rewards.len() > 50 {
            recent_rewards.remove(0);
        }

        if recent_rewards.len() == 50 && agent.check_mastery(&recent_rewards) {
            agent.evolve();
            recent_rewards.clear();
        }

        if episode % 10 == 0 {
            println!("Episode {}: Task = {}, Action = {}, Reward = {}", episode, agent.task, action, reward);
        }

        episode += 1;
    }
}
