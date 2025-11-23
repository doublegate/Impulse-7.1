# Contributing to Impulse-Next_BBS

Thank you for your interest in contributing to the Impulse-Next_BBS project! This document provides guidelines and information for contributors.

> **Note**: This project modernizes the classic Impulse 7.1 BBS from Borland Pascal to Rust.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing Requirements](#testing-requirements)
- [Pull Request Process](#pull-request-process)
- [Commit Message Guidelines](#commit-message-guidelines)

## Code of Conduct

This project follows standard open-source etiquette:

- Be respectful and constructive in all interactions
- Welcome newcomers and help them get started
- Focus on what is best for the community and project
- Show empathy towards other contributors

## Getting Started

### Prerequisites

- **Rust**: Version 1.80 or later
- **Git**: Version 2.30 or later
- **IDE**: VS Code with rust-analyzer recommended

### Initial Setup

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/Impulse-7.1.git
   cd Impulse-7.1
   ```

3. Add the upstream repository:
   ```bash
   git remote add upstream https://github.com/doublegate/Impulse-Next_BBS.git
   ```

4. Build the workspace:
   ```bash
   cargo build --workspace
   ```

5. Run the test suite:
   ```bash
   cargo test --workspace
   ```

## Development Workflow

### Branch Naming Convention

Use descriptive branch names following this pattern:

- `feature/description` - New features
- `fix/description` - Bug fixes
- `docs/description` - Documentation updates
- `refactor/description` - Code refactoring
- `test/description` - Test additions or improvements

Examples:
- `feature/add-user-authentication`
- `fix/message-threading-bug`
- `docs/update-api-documentation`

### Working on an Issue

1. Ensure an issue exists for your work (create one if needed)
2. Comment on the issue to indicate you're working on it
3. Create a feature branch from `main`:
   ```bash
   git checkout -b feature/your-feature-name main
   ```

4. Make your changes following the coding standards
5. Add tests for new functionality
6. Update documentation as needed

### Keeping Your Branch Updated

Regularly sync your branch with upstream:

```bash
git fetch upstream
git rebase upstream/main
```

## Coding Standards

### Rust Style Guide

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for consistent formatting
- Fix all `cargo clippy` warnings before submitting

### Code Quality

All code must:

1. **Compile without warnings**
   ```bash
   cargo clippy --all-targets --all-features -- -D warnings
   ```

2. **Be formatted with rustfmt**
   ```bash
   cargo fmt --all
   ```

3. **Include comprehensive documentation**
   - Module-level documentation (`//!`)
   - Public API documentation (`///`)
   - Examples in documentation where appropriate

4. **Use appropriate error handling**
   - Library crates: Use `thiserror` for error types
   - Application crates: Use `anyhow` for error handling
   - Always use `Result<T>` for fallible operations

### Documentation Standards

- All public types must have rustdoc comments
- Include examples in documentation when helpful
- Document panics, safety, and errors sections as appropriate
- Keep documentation up-to-date with code changes

Example:

```rust
/// Validates a user account
///
/// # Errors
///
/// Returns `Error::Validation` if:
/// - Username is empty or too long
/// - Email format is invalid
///
/// # Examples
///
/// ```
/// use impulse_types::user::User;
///
/// let user = User::new("alice");
/// assert!(user.validate().is_ok());
/// ```
pub fn validate(&self) -> Result<()> {
    // Implementation
}
```

## Testing Requirements

### Test Coverage

- All new functionality must include tests
- Aim for high code coverage (>80% preferred)
- Include both unit tests and integration tests where appropriate

### Test Organization

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specific_behavior() {
        // Test implementation
    }
}
```

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run tests for a specific crate
cargo test -p impulse-types

# Run with output
cargo test --workspace -- --nocapture

# Run doc tests
cargo test --workspace --doc
```

## Pull Request Process

### Before Submitting

Ensure your PR passes all checks:

```bash
# Format code
cargo fmt --all

# Check for issues
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test --workspace

# Build in release mode
cargo build --workspace --release
```

### PR Description

Your PR description should include:

1. **Summary**: Brief description of changes
2. **Motivation**: Why this change is needed
3. **Changes**: Detailed list of modifications
4. **Testing**: How the changes were tested
5. **Related Issues**: Link to related issues (closes #123)

Example:

```markdown
## Summary
Adds user authentication validation

## Motivation
Closes #42 - Need to validate user credentials during login

## Changes
- Added password hashing with Argon2id
- Implemented session token generation
- Added rate limiting for failed login attempts

## Testing
- Added unit tests for password validation
- Added integration tests for login flow
- Manually tested with various password strengths

## Related Issues
Closes #42
```

### Review Process

1. Submit your PR against the `main` branch
2. Ensure CI checks pass
3. Address reviewer feedback promptly
4. Keep the PR scope focused and manageable
5. Squash commits if requested

### Merging

- PRs require at least one approval from a maintainer
- All CI checks must pass
- No merge conflicts with `main`
- Maintainers will merge approved PRs

## Commit Message Guidelines

### Format

Use [Conventional Commits](https://www.conventionalcommits.org/) format:

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

### Examples

```
feat(auth): add Argon2id password hashing

Implements secure password hashing using the Argon2id algorithm
to replace the legacy password storage mechanism.

Closes #42
```

```
fix(message): correct threading parent_id validation

The parent_id field was not being validated properly, allowing
invalid references. Added validation to ensure parent messages
exist in the same area.

Fixes #87
```

```
docs(readme): update build instructions

Added detailed steps for Windows users and clarified
dependency requirements.
```

### Commit Best Practices

- Keep commits atomic and focused
- Write clear, descriptive commit messages
- Reference issues in commit messages
- Squash work-in-progress commits before merging

## Questions?

If you have questions about contributing:

- Open an issue for discussion
- Check existing issues and pull requests
- Read the project documentation in the `docs/` directory

## License

By contributing to Impulse 7.1 BBS, you agree that your contributions will be licensed under the same terms as the project (MIT OR Apache-2.0).

Thank you for contributing to Impulse 7.1 BBS!
