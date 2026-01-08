# Claude Code Preferences

## Code Style

- Always run `forge fmt --check` for linting
- Mappings should use descriptive variable names: `mapping(address acceptedToken => bool isAccepted) public supportedTokens;`
- No emojis in documentation, comments, or deploy scripts

## Security

- Always review code for vulnerabilities and security issues
- Look for common smart contract vulnerabilities (reentrancy, overflow, access control, etc.)

## Testing

- Update tests when refactoring contracts
- Ensure comprehensive test coverage for new functionality
