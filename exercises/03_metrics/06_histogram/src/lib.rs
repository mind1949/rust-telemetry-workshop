use std::thread;
use std::time::Duration;

static DO_SOMETHING_ONCE: std::sync::Once = std::sync::Once::new();

fn do_something(t: Duration) {
    thread::sleep(t);
    // TODO: register how long it takes to run this function using an `invocation_duration_seconds`
    //   histogram.
    DO_SOMETHING_ONCE.call_once(|| {
        metrics::describe_histogram!(
            "invocation_duration_seconds",
            metrics::Unit::Seconds,
            "invocation_duration_seconds"
        );
    });
    let histogram = metrics::histogram!("invocation_duration_seconds");
    histogram.record(t);
}

#[cfg(test)]
mod tests {
    use crate::do_something;
    use helpers::init_test_recorder;
    use metrics::Unit;
    use metrics_util::MetricKind;
    use std::time::Duration;

    #[test]
    fn histogram() {
        let snapshotter = init_test_recorder();

        for i in 0..7 {
            do_something(Duration::from_millis(i * 5));
        }

        let metrics = snapshotter.snapshot().into_vec();
        assert_eq!(metrics.len(), 1);
        let (metric_key, unit, description, value) = &metrics[0];
        assert_eq!(metric_key.kind(), MetricKind::Histogram);
        assert_eq!(metric_key.key().name(), "invocation_duration_seconds");
        assert_eq!(unit.unwrap(), Unit::Seconds);
    }
}
