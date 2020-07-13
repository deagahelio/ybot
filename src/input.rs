use std::convert::{TryInto, TryFrom};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Action {
    Press,
    Release,
}

impl TryFrom<u8> for Action {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Action::Press),
            1 => Ok(Action::Release),
            _ => Err("invalid action value"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Input {
    pub x_pos: f32,
    pub y_pos: f32,
    pub action: Action,
}

impl Input {
    pub fn new(x_pos: f32, y_pos: f32, action: Action) -> Self {
        Self {
            x_pos,
            y_pos,
            action,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut input = Vec::with_capacity(9);
        input.extend_from_slice(&self.x_pos.to_le_bytes());
        input.extend_from_slice(&self.y_pos.to_le_bytes());
        input.push(self.action as u8);
        input
    }

    pub fn deserialize(input: &[u8]) -> Self {
        Self {
            x_pos: f32::from_le_bytes(input[0..4].try_into().unwrap()),
            y_pos: f32::from_le_bytes(input[4..8].try_into().unwrap()),
            action: input[8].try_into().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialization() {
        let input = Input::new(95.6715, 6756.13846, Action::Release);
        let same_input = Input::deserialize(&input.clone().serialize());

        assert_eq!(input, same_input);
    }
}