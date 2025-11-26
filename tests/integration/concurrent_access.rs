//! Integration tests for concurrent user access
//!
//! This module tests the system's ability to handle multiple concurrent users
//! and operations, ensuring data consistency and proper isolation.
//!
//! Tests include:
//! - Concurrent authentication
//! - Concurrent message posting
//! - Concurrent file operations
//! - Data consistency verification
//! - Session isolation

use impulse_auth::{flows::AuthService, lockout::AccountLockout, rate_limit::RateLimiter, PasswordHasher};
use impulse_message::{
    formats::JamMessageBase,
    traits::MessageBase,
    types::NewMessage,
};
use impulse_user::{InMemoryUserManager, UserManager};
use impulse_types::user::User;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

#[tokio::test]
async fn test_concurrent_authentication() {
    // Setup
    let auth = Arc::new(RwLock::new(AuthService::new_with_protection(
        Duration::from_secs(1800),
        RateLimiter::new_default(),
        AccountLockout::new_default(),
    )));
    let hasher = PasswordHasher::new();

    // Create test credentials
    let password = "SecureP@ss123";
    let password_hash = hasher.hash_password(password).expect("Failed to hash");

    // Spawn multiple concurrent login attempts
    let mut handles = vec![];
    for i in 0..5 {
        let auth_clone = auth.clone();
        let username = format!("user{}", i);
        let password = password.to_string();
        let hash = password_hash.clone();

        let handle = tokio::spawn(async move {
            let mut auth = auth_clone.write().await;
            auth.authenticate(&username, &password, &hash).await
        });

        handles.push(handle);
    }

    // Wait for all logins to complete
    let results: Vec<_> = futures::future::join_all(handles)
        .await
        .into_iter()
        .map(|r| r.expect("Task panicked"))
        .collect();

    // All logins should succeed
    for (i, result) in results.iter().enumerate() {
        assert!(result.is_ok(), "Login {} should succeed", i);
    }

    // Verify all sessions are valid
    let auth_read = auth.read().await;
    for result in results {
        let token = result.expect("Login should have succeeded");
        assert!(auth_read.validate_session(&token).await.is_ok());
    }
}

#[tokio::test]
async fn test_concurrent_user_creation() {
    // Setup
    let manager = Arc::new(RwLock::new(InMemoryUserManager::new()));

    // Spawn multiple concurrent user creation tasks
    let mut handles = vec![];
    for i in 0..10 {
        let manager_clone = manager.clone();
        let username = format!("user{}", i);

        let handle = tokio::spawn(async move {
            let mut mgr = manager_clone.write().await;
            let user = User::new(&username).expect("Failed to create user");
            mgr.create_user(user).await
        });

        handles.push(handle);
    }

    // Wait for all creations to complete
    let results: Vec<_> = futures::future::join_all(handles)
        .await
        .into_iter()
        .map(|r| r.expect("Task panicked"))
        .collect();

    // All creations should succeed
    for (i, result) in results.iter().enumerate() {
        assert!(result.is_ok(), "User creation {} should succeed", i);
    }

    // Verify all users were created
    let mgr = manager.read().await;
    let users = mgr.list_users().await.expect("Failed to list users");
    assert_eq!(users.len(), 10, "Should have 10 users");
}

#[tokio::test]
async fn test_concurrent_message_posting() {
    // Setup
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let base_path = temp_dir.path().join("testbase");

    let mut base = JamMessageBase::new(&base_path);
    base.create().await.expect("Failed to create base");

    let base = Arc::new(RwLock::new(base));

    // Spawn multiple concurrent message posting tasks
    let mut handles = vec![];
    for i in 0..10 {
        let base_clone = base.clone();
        let subject = format!("Message {}", i);

        let handle = tokio::spawn(async move {
            let mut base = base_clone.write().await;
            let msg = NewMessage::new("Alice", "Bob", subject);
            base.post_message(msg).await
        });

        handles.push(handle);
    }

    // Wait for all posts to complete
    let results: Vec<_> = futures::future::join_all(handles)
        .await
        .into_iter()
        .map(|r| r.expect("Task panicked"))
        .collect();

    // All posts should succeed
    for (i, result) in results.iter().enumerate() {
        assert!(result.is_ok(), "Message post {} should succeed", i);
    }

    // Verify all messages were posted
    let base_read = base.read().await;
    let count = base_read
        .message_count()
        .await
        .expect("Failed to get count");
    assert_eq!(count, 10, "Should have 10 messages");

    // Verify message numbers are sequential and unique
    let mut msg_nums: Vec<u32> = results
        .into_iter()
        .map(|r| r.expect("Post should have succeeded"))
        .collect();
    msg_nums.sort();

    for (i, num) in msg_nums.iter().enumerate() {
        assert_eq!(*num, (i + 1) as u32, "Message numbers should be sequential");
    }
}

#[tokio::test]
async fn test_concurrent_read_write() {
    // Setup
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let base_path = temp_dir.path().join("testbase");

    let mut base = JamMessageBase::new(&base_path);
    base.create().await.expect("Failed to create base");

    // Post initial message
    let msg = NewMessage::new("Alice", "Bob", "Initial Message");
    base.post_message(msg).await.expect("Failed to post");

    let base = Arc::new(RwLock::new(base));

    // Spawn concurrent readers and writers
    let mut handles = vec![];

    // 5 readers
    for _ in 0..5 {
        let base_clone = base.clone();
        let handle = tokio::spawn(async move {
            let base = base_clone.read().await;
            base.read_message(1).await
        });
        handles.push(handle);
    }

    // 5 writers
    for i in 0..5 {
        let base_clone = base.clone();
        let subject = format!("Concurrent Message {}", i);
        let handle = tokio::spawn(async move {
            let mut base = base_clone.write().await;
            let msg = NewMessage::new("Bob", "Alice", subject);
            base.post_message(msg).await
        });
        handles.push(handle);
    }

    // Wait for all operations
    let results = futures::future::join_all(handles).await;

    // All operations should succeed
    for (i, result) in results.iter().enumerate() {
        assert!(
            result.is_ok(),
            "Operation {} should not panic",
            i
        );
    }

    // Verify final state
    let base_read = base.read().await;
    let count = base_read
        .message_count()
        .await
        .expect("Failed to get count");
    assert_eq!(count, 6, "Should have 6 messages (1 initial + 5 concurrent)");
}

#[tokio::test]
async fn test_session_isolation() {
    // Setup
    let auth = Arc::new(RwLock::new(AuthService::new_with_protection(
        Duration::from_secs(1800),
        RateLimiter::new_default(),
        AccountLockout::new_default(),
    )));
    let hasher = PasswordHasher::new();

    let password = "SecureP@ss123";
    let hash1 = hasher.hash_password(password).expect("Failed to hash");
    let hash2 = hasher.hash_password(password).expect("Failed to hash");

    // Create two sessions for different users
    let mut auth_write = auth.write().await;
    let token1 = auth_write
        .authenticate("user1", password, &hash1)
        .await
        .expect("Login 1 should succeed");
    let token2 = auth_write
        .authenticate("user2", password, &hash2)
        .await
        .expect("Login 2 should succeed");
    drop(auth_write);

    // Both sessions should be valid
    let auth_read = auth.read().await;
    assert!(auth_read.validate_session(&token1).await.is_ok());
    assert!(auth_read.validate_session(&token2).await.is_ok());
    drop(auth_read);

    // Logout from session 1
    let mut auth_write = auth.write().await;
    auth_write
        .logout(&token1)
        .await
        .expect("Logout should succeed");
    drop(auth_write);

    // Session 1 should be invalid, session 2 should still be valid
    let auth_read = auth.read().await;
    assert!(auth_read.validate_session(&token1).await.is_err());
    assert!(auth_read.validate_session(&token2).await.is_ok());
}

#[tokio::test]
async fn test_data_consistency_under_load() {
    // Setup
    let manager = Arc::new(RwLock::new(InMemoryUserManager::new()));

    // Create initial user
    let mut mgr = manager.write().await;
    let user = User::new("testuser").expect("Failed to create user");
    mgr.create_user(user).await.expect("Failed to create");
    drop(mgr);

    // Spawn multiple concurrent update tasks
    let mut handles = vec![];
    for i in 0..20 {
        let manager_clone = manager.clone();
        let location = format!("City {}", i);

        let handle = tokio::spawn(async move {
            let mgr = manager_clone.read().await;
            let mut user = mgr
                .find_by_username("testuser")
                .await
                .expect("Failed to find")
                .expect("User should exist");
            drop(mgr);

            user.set_location(&location);

            let mut mgr = manager_clone.write().await;
            mgr.update_user(user).await
        });

        handles.push(handle);
    }

    // Wait for all updates
    let results: Vec<_> = futures::future::join_all(handles)
        .await
        .into_iter()
        .map(|r| r.expect("Task panicked"))
        .collect();

    // All updates should complete without panic
    // (Some may fail due to concurrent modification, but shouldn't panic)
    let success_count = results.iter().filter(|r| r.is_ok()).count();
    assert!(
        success_count > 0,
        "At least some updates should succeed"
    );

    // Final state should be consistent
    let mgr = manager.read().await;
    let final_user = mgr
        .find_by_username("testuser")
        .await
        .expect("Failed to find")
        .expect("User should exist");

    // Location should be set to one of the attempted values
    assert!(final_user.location().is_some());
}

#[tokio::test]
async fn test_concurrent_search_operations() {
    // Setup
    let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
    let base_path = temp_dir.path().join("testbase");

    let mut base = JamMessageBase::new(&base_path);
    base.create().await.expect("Failed to create base");

    // Post several messages
    for i in 0..10 {
        let msg = NewMessage::new("Alice", "Bob", format!("Message {}", i));
        base.post_message(msg).await.expect("Failed to post");
    }

    let base = Arc::new(RwLock::new(base));

    // Spawn multiple concurrent search tasks
    let mut handles = vec![];
    for i in 0..5 {
        let base_clone = base.clone();
        let search_term = format!("Message {}", i);

        let handle = tokio::spawn(async move {
            let base = base_clone.read().await;
            let criteria = impulse_message::types::SearchCriteria::new()
                .with_subject(&search_term);
            base.search(&criteria).await
        });

        handles.push(handle);
    }

    // Wait for all searches
    let results: Vec<_> = futures::future::join_all(handles)
        .await
        .into_iter()
        .map(|r| r.expect("Task panicked"))
        .collect();

    // All searches should succeed
    for (i, result) in results.iter().enumerate() {
        assert!(result.is_ok(), "Search {} should succeed", i);
        let found = result.as_ref().unwrap();
        assert_eq!(found.len(), 1, "Should find exactly 1 message");
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_high_concurrency_stress() {
    // Setup
    let manager = Arc::new(RwLock::new(InMemoryUserManager::new()));

    // Spawn 100 concurrent operations
    let mut handles = vec![];
    for i in 0..100 {
        let manager_clone = manager.clone();
        let username = format!("user{}", i);

        let handle = tokio::spawn(async move {
            // Create user
            let mut mgr = manager_clone.write().await;
            let user = User::new(&username).expect("Failed to create user");
            mgr.create_user(user).await.expect("Failed to create");
            drop(mgr);

            // Read user back
            let mgr = manager_clone.read().await;
            let found = mgr
                .find_by_username(&username)
                .await
                .expect("Failed to find");
            assert!(found.is_some(), "User should exist");
        });

        handles.push(handle);
    }

    // Wait for all operations
    futures::future::join_all(handles).await;

    // Verify final state
    let mgr = manager.read().await;
    let users = mgr.list_users().await.expect("Failed to list");
    assert_eq!(users.len(), 100, "Should have 100 users");
}
