This repo is for replicating an issue with Figment (or my usage of Figment).

The issue seems to be related to parsing nested structs and correctly applying the default values to the nested structs. I have defaults created for the top level struct, and the nested structs, but I get parsing errors if the actual nested structs in the config file don't have all fields.

The issue can be replicated by running `cargo run -- config.toml`.
