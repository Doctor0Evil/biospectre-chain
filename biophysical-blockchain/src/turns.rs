use chrono::{Datelike, Utc};

#[derive(Clone, Debug)]
pub struct DailyTurnState {
    pub date_yyyymmdd: u32,
    pub turns_used: u8,
}

impl DailyTurnState {
    pub fn new_for_today() -> Self {
        let now = Utc::now().date_naive();
        let d = (now.year() as u32) * 10_000 + (now.month() as u32) * 100 + now.day() as u32;
        DailyTurnState { date_yyyymmdd: d, turns_used: 0 }
    }

    pub fn can_consume_turn(&mut self, max_turns: u8) -> bool {
        let now = Utc::now().date_naive();
        let d = (now.year() as u32) * 10_000 + (now.month() as u32) * 100 + now.day() as u32;
        
        // Reset counter if it's a new day
        if d != self.date_yyyymmdd {
            self.date_yyyymmdd = d;
            self.turns_used = 0;
        }
        
        // Consume a turn if available
        if self.turns_used < max_turns {
            self.turns_used += 1;
            true
        } else {
            false
        }
    }
}
