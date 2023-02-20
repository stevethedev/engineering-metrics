# Authentication Library

This library provides authentication services for the rest of the application.

## Use-Cases

### Login

The login use-case is used to authenticate a user. It takes a username and returns a
unique token that can be used to authenticate the user in the future.

### Logout

The logout use-case is used to invalidate a user's token. It takes a token and
invalidates it.

### Register

The register use-case is used to register a new user. It takes a username and
password and creates a new user.

### Verify

The verify use-case is used to verify a user's token. It takes a token and returns
the user's record.
