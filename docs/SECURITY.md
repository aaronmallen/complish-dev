# Security Policy

## Our Commitment

Security is paramount when dealing with financial data. We take security vulnerabilities seriously and are committed to
addressing them promptly and transparently.

## What Constitutes a Security Vulnerability

A security vulnerability is an issue that could:

- Allow unauthorized access to financial data
- Enable data manipulation or destruction
- Permit privilege escalation
- Bypass authentication or authorization mechanisms
- Enable denial of service attacks
- Expose sensitive information (API keys, passwords, personal data)

**Not security vulnerabilities:**

- General bugs that don't compromise security
- Feature requests or enhancements
- Performance issues
- Documentation errors

## Reporting a Vulnerability

If you discover a security issue, please bring it to our attention right away!

### Reporting Process

Please **DO NOT** file a public issue. Instead, report security vulnerabilities through
[GitHub's private vulnerability reporting feature](https://github.com/aaronmallen/complish-dev/security/advisories/new).

Your report should include:

- Description of the vulnerability
- Steps to reproduce the issue
- Potential impact of the vulnerability
- Affected versions (if known)
- Suggested fix (if any)
- Your contact information for follow-up questions

### What to Expect

After you've submitted your report:

1. **Acknowledgment** - You'll receive confirmation within 24 hours
2. **Investigation** - We'll investigate and keep you updated on our findings
3. **Resolution** - Once we've determined the impact and developed a fix:

- We'll patch the vulnerability
- We'll coordinate disclosure timing with you
- We'll make an announcement to the community if warranted
- You'll be credited for the discovery (unless you prefer to remain anonymous)

### Response Timeline

- **24 hours** - Initial response acknowledging receipt
- **72 hours** - Preliminary assessment of impact and severity
- **7 days** - Detailed investigation results and remediation plan
- **30 days** - Target for patch release (may vary based on complexity)

## Disclosure Policy

We follow responsible disclosure practices:

1. **Confirm** the problem and determine affected versions
2. **Audit** code to find any similar problems
3. **Prepare** fixes for all supported versions
4. **Coordinate** with the reporter on disclosure timing
5. **Release** patches as soon as possible
6. **Publish** a security advisory with appropriate details

## Supported Versions

| Version | Support |
| :-----: | :-----: |
|   TBD   |   ✅    |

_Note: As the project is in early development, version support will be defined with the first stable release._

### Key

| Symbol |    Meaning     |
| :----: | :------------: |
|   ✅   |   Supported    |
|   ❌   | Not Supported  |
|   🧪   |  Experimental  |
|   🚧   | In Development |

## Security Best Practices

When contributing to Complish, please follow these security guidelines:

- Never commit sensitive data (passwords, API keys, personal information)
- Use parameterized queries to prevent SQL injection
- Validate and sanitize all user inputs
- Follow the principle of least privilege
- Use secure defaults in configuration
- Keep dependencies up to date

## Comments on this Policy

If you have suggestions on how this process could be improved, please submit a pull request or open an issue for
discussion.

## Contact

For urgent security matters that require immediate attention, you can also reach out to
[@aaronmallen](https://github.com/aaronmallen) directly.

---

**Remember**: Your security is our priority. When you self-host Complish, you're in control - and we want to make sure
that control is as secure as possible.
