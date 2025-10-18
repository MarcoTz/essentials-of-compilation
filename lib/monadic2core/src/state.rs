pub struct ExplicateState {
    pub current_label: String,
    used_labels: Vec<String>,
}

impl ExplicateState {
    pub fn new() -> ExplicateState {
        ExplicateState {
            current_label: "start".to_owned(),
            used_labels: vec!["start".to_owned()],
        }
    }

    pub fn fresh_label(&mut self) {
        self.used_labels.push(self.current_label.clone());
        let prefix = "block_";
        let mut num = 0;
        let mut label = format!("{prefix}{num}");
        while self.used_labels.contains(&&label) {
            num += 1;
            label = format!("{prefix}{num}");
        }
        self.current_label = label
    }

    pub fn last_label(&self) -> String {
        self.used_labels.last().unwrap().clone()
    }
}

impl Default for ExplicateState {
    fn default() -> ExplicateState {
        ExplicateState::new()
    }
}
