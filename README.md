# rt_timer

Real time timer utility crate.

## Example

```rust
use rt_timer::Timer;

fn main() {
    let mut timer = Timer::from_hz(500.0);
    let mut log_gate = timer.gate(50.0);

    loop {
        timer.wait();

        if log_gate.tick() {
            println!(
                "t={:.3}s dt={:.3}ms",
                timer.elapsed().as_secs_f64(),
                timer.dt().as_secs_f64() * 1_000.0,
            );
        }
    }
}
```
