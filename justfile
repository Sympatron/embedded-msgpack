
test:
  just test-default
  just test-no-default
  just test-std
  just test-alloc
  just test-compliant

test-default:
  cargo test --features=debug-impls

test-std:
  cargo test --features=std

test-no-default:
  cargo test --features=debug-impls --no-default-features

test-alloc:
  cargo test --features=debug-impls,alloc

test-compliant:
  cargo test --features=debug-impls,compliant

fmt:
  cargo +nightly fmt