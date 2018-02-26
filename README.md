# launchdarkly-rust-client
This was begun as part of a day-long hackathon at LaunchDarkly. I started out knowing nothing about [Rust](rust-lang.org/) except its name. I still don't know much more than that.

The only functionality currently implemented is:

* Polling for feature flags.
* Evaluating feature flags. All currently supported operators should work, except segment matching.

Not yet implemented:

* Reusable user segments.
* A streaming client.
* Analytics events.
* A persistent feature store (e.g. Redis).
* Any kind of logging or error handling.
