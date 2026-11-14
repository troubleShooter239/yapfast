# Yapfast stream group fix

This is davey 0.1.4 from crates.io under its original MIT license
(LICENSE, as the crate's manifest declares; the published crate ships no
licence file).

davey names a session's MLS group after the voice channel it's given, and
the only way to change it, `reinit`, clears the session's storage. A Go Live
stream's group isn't its voice channel's, and Yapfast learns the real id only
from the voice server's first proposals or commit. By then it has sent a key
package, and clearing storage throws away that package's private keys, so the
welcome built from it fails with `NoMatchingKeyPackage`.

Changes, marked `Yapfast:` in the source:

- `DaveSession::set_channel_id` changes the group id and keeps the signing key
  and stored key packages. A pending group starts again under the new id; a
  joined group is left as it is.
- `read_leb128` stops at ten bytes rather than shifting past a `u64`, which
  panics in a debug build when a received frame's nonce runs that long.

Dependency maintenance: the OpenMLS crates come from one upstream revision,
`6100df3fb1d8c3eeab53e349fcbd621f98c44fe2`, including its updated P-256 and HPKE
providers. No separate OpenMLS source copies are maintained under vendor/.
The upstream manifest at this revision reports OpenMLS 0.9.0. process_commit
merges a pending commit only for authenticated OwnPendingCommit results;
mismatched own commits remain errors. The exported secret wrapper is converted
to the bytes the existing media ratchet consumes, without logging the secret.
Unused hex-literal is removed. napi and num-traits are kept for macro expansions.

OpenMLS was already a dependency of the original davey 0.1.4, and remains one
in the published upstream davey 0.2.0; it implements this Rust DAVE session's MLS.
