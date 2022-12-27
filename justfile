
test:
  just test-default
  just test-no-default
  just test-std
  just test-alloc
  just test-compliant

test-default:
  cargo test

test-std:
  cargo test --features=std

test-no-default:
  cargo test --no-default-features

test-alloc:
  cargo test --features=alloc

test-compliant:
  cargo test --features=compliant

fmt:
  cargo +nightly fmt