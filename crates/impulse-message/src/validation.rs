//! Message validation functionality

use crate::error::{MessageError, Result};
use crate::types::{NewMessage, ValidationLimits};

/// Message validator
pub struct MessageValidator {
    limits: ValidationLimits,
}

impl MessageValidator {
    /// Create a new validator with default limits
    pub fn new() -> Self {
        Self {
            limits: ValidationLimits::default(),
        }
    }

    /// Create a validator with custom limits
    pub fn with_limits(limits: ValidationLimits) -> Self {
        Self { limits }
    }

    /// Validate a new message
    ///
    /// # Arguments
    /// * `message` - The message to validate
    ///
    /// # Errors
    /// Returns validation error if message doesn't meet requirements
    pub fn validate(&self, message: &NewMessage) -> Result<()> {
        // Validate required fields
        self.validate_required_fields(message)?;

        // Validate field lengths
        self.validate_field_lengths(message)?;

        Ok(())
    }

    /// Validate required fields are present
    fn validate_required_fields(&self, message: &NewMessage) -> Result<()> {
        if message.from.trim().is_empty() {
            return Err(MessageError::RequiredFieldMissing("from".to_string()));
        }

        if message.to.trim().is_empty() {
            return Err(MessageError::RequiredFieldMissing("to".to_string()));
        }

        if message.subject.trim().is_empty() {
            return Err(MessageError::RequiredFieldMissing("subject".to_string()));
        }

        if message.body.trim().is_empty() {
            return Err(MessageError::RequiredFieldMissing("body".to_string()));
        }

        Ok(())
    }

    /// Validate field lengths
    fn validate_field_lengths(&self, message: &NewMessage) -> Result<()> {
        // Validate subject length
        if message.subject.len() < self.limits.min_subject_len {
            return Err(MessageError::SubjectTooShort {
                min: self.limits.min_subject_len,
            });
        }

        if message.subject.len() > self.limits.max_subject_len {
            return Err(MessageError::SubjectTooLong {
                max: self.limits.max_subject_len,
                actual: message.subject.len(),
            });
        }

        // Validate body length
        if message.body.len() < self.limits.min_body_len {
            return Err(MessageError::BodyTooShort {
                min: self.limits.min_body_len,
            });
        }

        if message.body.len() > self.limits.max_body_len {
            return Err(MessageError::BodyTooLong {
                max: self.limits.max_body_len,
                actual: message.body.len(),
            });
        }

        Ok(())
    }

    /// Get validation limits
    pub fn limits(&self) -> &ValidationLimits {
        &self.limits
    }
}

impl Default for MessageValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_message() {
        let validator = MessageValidator::new();
        let message = NewMessage::new("Alice", "Bob", "Test Subject")
            .with_body("This is a test message body.");

        assert!(validator.validate(&message).is_ok());
    }

    #[test]
    fn test_empty_from() {
        let validator = MessageValidator::new();
        let message =
            NewMessage::new("", "Bob", "Test Subject").with_body("This is a test message body.");

        let result = validator.validate(&message);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MessageError::RequiredFieldMissing(_)
        ));
    }

    #[test]
    fn test_empty_to() {
        let validator = MessageValidator::new();
        let message =
            NewMessage::new("Alice", "", "Test Subject").with_body("This is a test message body.");

        let result = validator.validate(&message);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MessageError::RequiredFieldMissing(_)
        ));
    }

    #[test]
    fn test_empty_subject() {
        let validator = MessageValidator::new();
        let message = NewMessage::new("Alice", "Bob", "").with_body("This is a test message body.");

        let result = validator.validate(&message);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MessageError::RequiredFieldMissing(_)
        ));
    }

    #[test]
    fn test_empty_body() {
        let validator = MessageValidator::new();
        let message = NewMessage::new("Alice", "Bob", "Test Subject");

        let result = validator.validate(&message);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MessageError::RequiredFieldMissing(_)
        ));
    }

    #[test]
    fn test_subject_too_long() {
        let validator = MessageValidator::new();
        let long_subject = "a".repeat(100);
        let message =
            NewMessage::new("Alice", "Bob", long_subject).with_body("This is a test message body.");

        let result = validator.validate(&message);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MessageError::SubjectTooLong { .. }
        ));
    }

    #[test]
    fn test_body_too_long() {
        let validator = MessageValidator::new();
        let long_body = "a".repeat(100_000);
        let message = NewMessage::new("Alice", "Bob", "Test Subject").with_body(long_body);

        let result = validator.validate(&message);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            MessageError::BodyTooLong { .. }
        ));
    }

    #[test]
    fn test_custom_limits() {
        let limits = ValidationLimits {
            max_subject_len: 10,
            max_body_len: 50,
            max_line_width: 79,
            min_subject_len: 1,
            min_body_len: 1,
        };
        let validator = MessageValidator::with_limits(limits);

        let message = NewMessage::new("Alice", "Bob", "Short").with_body("This is within limits.");
        assert!(validator.validate(&message).is_ok());

        let message =
            NewMessage::new("Alice", "Bob", "Too Long Subject").with_body("This is a test.");
        assert!(validator.validate(&message).is_err());
    }
}
