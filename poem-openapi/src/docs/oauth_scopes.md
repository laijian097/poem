Define a OAuth scopes.

# Macro parameters

| Attribute  | description                                                                                                                                                                     | Type   | Optional |
|------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|--------|----------|
| rename_all | Rename all the items according to the given case convention. The possible values are "lowercase", "UPPERCASE", "PascalCase", "camelCase", "snake_case", "SCREAMING_SNAKE_CASE". | string | Y        |

# Item parameters

| Attribute | description           | Type   | Optional |
|-----------|-----------------------|--------|----------|
| rename    | Rename the scope name | string | Y        |

# Examples

```rust
use poem_openapi::OAuthScopes;

#[derive(OAuthScopes)]
enum GithubScopes {
    /// Read data
    Read,
    /// Write data
    Write,
}
```