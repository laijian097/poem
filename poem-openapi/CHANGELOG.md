# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

# [1.0.27] 2021-11-16

- Description is a required field for responses. [#86](https://github.com/poem-web/poem/issues/86)

# [1.0.26] 2021-11-15

- Add `version` and `title` parameters to `OpenAPIService::new`. [#87](https://github.com/poem-web/poem/issues/87)

# [1.0.19] 2021-11-03

- Add `checker` attribute for `SecurityScheme` macro.
- Use Rust 2021 edition.

# [1.0.18] 2021-11-02

- Some configurations no longer need `'static`.

# [1.0.12] 2021-10-27

- Correctly determine the type of payload.

# [1.0.11] 2021-10-27

- Bump `poem` to `1.0.11`.

# [1.0.10] 2021-10-26

- Make the return type of operation function more flexible.

# [1.0.9] 2021-10-26

- Add `Any` type.

# [1.0.8] 2021-10-25

- Add `read_only_all` and `write_only_all` to `ObjectArgs`. [#71](https://github.com/poem-web/poem/pull/71)

# [1.0.7] 2021-10-21

- Fix Json parsing not working for unsigned integers. [#68](https://github.com/poem-web/poem/pull/68)

# [1.0.4] 2021-10-15

- Bump `poem` from `1.0.3` to `1.0.4`.

# [1.0.3] 2021-10-14

- Add `prefix_path` and `tag` attributes for `#[OpenApi]`. [#57](https://github.com/poem-web/poem/pull/57)
- `OpenApiService::swagger_ui` method no longer needs the `absolute_uri` parameter.
- Add `inline` attribute for `Object` macro.
- Add generic support for `ApiRequest` and `ApiResponse` macros.

## [1.0.2] 2021-10-11

- Add `write_only` and `read_only` attributes for object fields.
- Add `OpenApiService::spec` method to get the generated OAS specification file.
- Implements `Default` trait for `poem_openapi::types::multipart::JsonField<T>`.
- Implements `ParseFromMultipartField` for some types.

## [1.0.1] 2021-10-10

- Add `Request::remote_addr` method.
