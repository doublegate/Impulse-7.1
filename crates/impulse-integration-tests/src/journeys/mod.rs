//! End-to-end user journey tests
//!
//! Tests complete user workflows from registration through various BBS
//! features to logout. These tests verify that all components work together
//! seamlessly in realistic usage scenarios.

mod complete;
mod scenarios;

pub use complete::UserJourneyTest;

#[cfg(test)]
mod tests {
    use crate::fixtures::BbsTestFixture;

    #[tokio::test]
    async fn test_basic_journey_setup() {
        let fixture = BbsTestFixture::new().await.unwrap();
        let user = fixture.create_regular_user().await.unwrap();
        assert!(user.id > 0);
    }
}
