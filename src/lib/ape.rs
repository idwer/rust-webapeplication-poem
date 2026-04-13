use serde::Deserialize;
use serde::Serialize;
use validator::Validate;

#[derive(Serialize)]
pub struct ApeIndexOutput {
    pub height: i16,
    pub wingspan: i16,
    pub ape_index: f32,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct ApeIndexInput {
    #[validate(range(
        min = 1,
        max = 300,
        message = "Expected numerical input for height should be between 1 and 300"
    ))]
    pub height: i16,
    #[validate(range(
        min = 1,
        max = 300,
        message = "Expected numerical input for wingspan should be between 1 and 300"
    ))]
    pub wingspan: i16,
}

impl ApeIndexInput {
    // height and wingspan can be signed ints, the validator macro will handle input filtering
    fn ape_index(height: i16, wingspan: i16) -> f32 {
        wingspan as f32 / height as f32
    }

    pub fn ape_index_from_json(self) -> ApeIndexOutput {
        ApeIndexOutput {
            height: self.height,
            wingspan: self.wingspan,
            ape_index: Self::ape_index(self.height, self.wingspan),
        }
    }
}

#[cfg(test)]
mod tests_ape_lib {
    use super::*;

    #[test]
    fn test_ape_index() {
        assert_eq!(ApeIndexInput::ape_index(100, 106), 1.06);
    }
}
