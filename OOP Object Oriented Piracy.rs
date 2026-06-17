struct Ship {
  draft: u32,
  crew: u32,
}

impl Ship {
    fn is_worth_it(&self) -> bool {
        return self.draft as f32 - 1.5 * self.crew as f32 > 20.0;
    }
}