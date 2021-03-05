Jonas's Tracing Util
====================

Small library containing some helpful methods I frequently use
throughout my ``rust/tokio/actix`` projects, when it comes to logging.
Based on the `tracing <https://github.com/tokio-rs/tracing>`_ and
`tracing-subscriber <https://github.com/tokio-rs/tracing>`_ libraries.

**NOTE**: the library depends on the ``RUST_LOG`` environment
variable. Make sure that you enable logging for this crate, e.g.:

.. code::bash

   RUST_LOG=jonases_tracing_util=info cargo run


TODO
----

* documentation

* publish
