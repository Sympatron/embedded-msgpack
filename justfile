
test:
  just test-default
  just test-no-default
  just test-std
  just test-alloc
  just test-compliant

test-default:
  cargo nextest run --features=derive-debug

test-std:
  cargo nextest run --features=std

test-no-default:
  cargo nextest run --features=derive-debug --no-default-features

test-alloc:
  cargo nextest run --features=derive-debug,alloc

test-compliant:
  cargo nextest run --features=derive-debug,compliant

fuzz:
  cargo +nightly fuzz run decode_from_slice -- -max_total_time=300

fmt:
  cargo +nightly fmt