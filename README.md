# spin_sleep_tokio

This is a fork of spin_sleep.

`std::thread::sleep` replaced by `tokio::time::sleep`.

Accurate sleeping. Only use native sleep as far as it can be trusted, then spin.

The problem with `tokio:time::sleep` is it isn't always very accurate, and this accuracy varies
on platform and state. Spinning is as accurate as we can get, but consumes the CPU
rather ungracefully.

This library adds a middle ground, using a configurable native accuracy setting allowing
thread::sleep to wait the bulk of a sleep time, and spin the final section to guarantee
accuracy.

### SpinSleeper

The simplest usage with default native accuracy is a drop in replacement for `tokio::time::sleep`.

```rust
spin_sleep_tokio::sleep(Duration::new(1, 12_550_000));
```

#### Configure

More advanced usage, including setting a custom native accuracy, can be achieved by
constructing a `SpinSleeper`.

```rust
// Create a new sleeper that trusts native thread::sleep with 100μs accuracy
let spin_sleeper = spin_sleep_tokio::SpinSleeper::new(100_000)
    .with_spin_strategy(spin_sleep_tokio::SpinStrategy::YieldThread);

// Sleep for 1.01255 seconds, this will:
//  - thread:sleep for 1.01245 seconds, i.e., 100μs less than the requested duration
//  - spin until total 1.01255 seconds have elapsed
spin_sleeper.sleep(Duration::new(1, 12_550_000));
```

Sleep can also be requested in `f64` seconds or `u64` nanoseconds
(useful when used with `time` crate)

```rust
spin_sleeper.sleep_s(1.01255);
spin_sleeper.sleep_ns(1_012_550_000);
```

OS-specific default settings should be good enough for most cases.

```rust
let sleeper = SpinSleeper::default();
```

### LoopHelper

For controlling & report rates (e.g., game FPS) this crate provides `LoopHelper`. A `SpinSleeper` is used to maximise
sleeping accuracy.

```rust
use spin_sleep_tokio::LoopHelper;

let mut loop_helper = LoopHelper::builder()
    .report_interval_s(0.5) // report every half a second
    .build_with_target_rate(250.0); // limit to 250 FPS if possible

let mut current_fps = None;

loop {
    let delta = loop_helper.loop_start(); // or .loop_start_s() for f64 seconds

    // compute_something(delta);

    if let Some(fps) = loop_helper.report_rate() {
        current_fps = Some(fps);
    }

    // render_fps(current_fps);

    loop_helper.loop_sleep(); // sleeps to achieve a 250 FPS rate
}
```

### Windows Accuracy

Windows has particularly poor accuracy by default (~15ms), `spin_sleep_tokio` will automatically
select the best accuracy on Windows generally achieving ~1ms native sleep accuracy _(Since 0.3.3)_.

## Minimum supported rust compiler

This crate is maintained with [latest stable rust](https://gist.github.com/alexheretic/d1e98d8433b602e57f5d0a9637927e0c).
