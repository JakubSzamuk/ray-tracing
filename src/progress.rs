use std::fmt::format;

pub struct Progress {
  bar: String,
  total: u64,
  step: u64,
  current: u64,
  eta: u64,
}

impl Progress {
  pub fn new(width: u64, height: u64) -> Progress {
    let total = width * height;

    let increment: u64 = total / 30 as u64;

    Progress {
      bar: "[]".to_string(),
      total: total,
      step: increment,
      current: 0,
      eta: 0,
    }
  }

  pub fn update(&mut self, current: u64, time_taken: u64) -> String {
    if current != self.current {
      self.bar = format!("[{}{}]", "#".repeat((current / self.step) as usize), " ".repeat((30 - current / self.step) as usize));
    }
    if time_taken > 5 {
      self.eta = (time_taken / ((self.total - self.current) / self.total)) - time_taken;
    }
    self.current = current;

    format!("{}   {}/{}   estimated time remaining: {}", self.bar, self.current, self.total, self.eta)
  }
}