# Config

## Config File

`cakestry` will read configuration from file `.cakestry/config.toml`.

## Environment Variables

The log filter can be set by the environment variable `CAKESTRY_LOG`. The detailed syntax of the
accepted value is described in https://docs.rs/tracing-subscriber/0.3.23/tracing_subscriber/filter/struct.EnvFilter.html#directives.

For example, to allow debug level events be emitted to `cakestry.log`, we can set `CAKESTRY_LOG` to `"debug"`.
