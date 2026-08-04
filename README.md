# rust-logging
Logging in Rust

## Logging Levels
  1. trace
  2. debug
  3. info
  4. warn
  5. error

## Logging Hierarchy
Typical of logging systems ...
  * trace encompasses ...
    * trace
    * debug
    * info
    * warn
    * error
  * debug encompasses ...
    * debug
    * info
    * warn
    * error
  * info encompasses ...
    * info
    * warn
    * error
  * warn encompasses ...
    * warn
    * error
  * error encompasses only error

## Env Var
  * The env var `RUST_LOG` controls the logging hierarchy - the extent of logging output
