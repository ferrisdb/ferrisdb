# Security Policy

## Supported Versions

FerrisDB is an educational project and should not be used in production environments.
However, we still take security seriously for learning purposes.

| Version | Supported          |
| ------- | ------------------ |
| main    | :white_check_mark: |
| < main  | :x:                |

## Reporting a Vulnerability

If you discover a security vulnerability in FerrisDB, please help us learn from it:

1. **DO NOT** create a public GitHub issue for the vulnerability.
2. Email us at: `security@ferrisdb.org` (Note: This is a placeholder email)
3. Alternatively, create a private security advisory on GitHub:
   - Go to the Security tab
   - Click on "Report a vulnerability"
   - Fill out the form with details

### What to include in your report

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

### What to expect

Since this is an educational project:

- We'll acknowledge receipt within 7 days
- We'll work on understanding and fixing the issue as a learning exercise
- We'll credit you in the fix (unless you prefer to remain anonymous)
- We'll write a blog post about what we learned (with your permission)

## Security Best Practices for Contributors

When contributing to FerrisDB:

1. **Never commit secrets** (API keys, passwords, tokens)
2. **Use environment variables** for sensitive configuration
3. **Validate all inputs** to prevent injection attacks
4. **Follow Rust safety guidelines** - minimize use of `unsafe`
5. **Keep dependencies updated** - monitor for security advisories
6. **Write tests** for security-critical code paths

## Learning Resources

This project is great for learning about database security:

- **Authentication & Authorization**: How databases control access
- **Encryption**: Data at rest and in transit
- **Input Validation**: Preventing injection attacks
- **Memory Safety**: Rust's approach to preventing vulnerabilities
- **Network Security**: TLS and secure communications

## Disclaimer

**FerrisDB is an educational project and NOT suitable for production use.**
It lacks many security features required for real-world deployment:

- No authentication/authorization system
- No encryption at rest
- No TLS support (yet)
- No audit logging
- Limited input validation

For production use, consider established databases like PostgreSQL, MySQL, or FoundationDB.
