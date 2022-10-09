/// Represents the frequency with which an operator is applied.
/// Ensures that the frequency is on the range [0, 1].
pub struct OperatorFreq {
    value: f64,
}

impl OperatorFreq {
    pub fn new(value: f64) -> Result<Self, &'static str> {
        if value < 0.0 || value > 1.0 {
            return Err("operator frequency must be in the range [0, 1]");
        }

        Ok(Self { value })
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}
