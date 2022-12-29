
test:
  just test-default
  just test-no-default
  just test-std
  just test-alloc
  just test-compliant

test-default:
  cargo test --features=derive-debug

test-std:
  cargo test --features=std

test-no-default:
  cargo test --features=derive-debug --no-default-features

test-alloc:
  cargo test --features=derive-debug,alloc

test-compliant:
  cargo test --features=derive-debug,compliant

fmt:
  cargo +nightly fmt