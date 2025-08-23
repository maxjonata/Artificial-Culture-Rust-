# Performance Monitoring and Alerting System

## Overview

This comprehensive performance monitoring and alerting system provides real-time monitoring of critical performance metrics for the Artificial Society simulation. It implements industry-standard thresholds and provides early warning alerts to help identify bottlenecks before they impact simulation performance.

## Features

### 🔍 **Comprehensive Monitoring**
- **System Resources**: CPU, Memory, GPU, Disk I/O usage
- **Frame Performance**: Frame time, FPS, frame jitter detection
- **AI System Performance**: Individual system execution times, total AI processing budget
- **Entity Scaling**: Performance correlation with agent count

### 🚨 **Smart Alerting**
- **Severity Levels**: Warning, Critical, Severe alerts based on impact
- **Sustained Monitoring**: Prevents false alarms from brief spikes
- **Baseline Comparison**: Alerts include normal values for context
- **JSON Logging**: Structured logs for analysis and debugging

### 📊 **Industry-Standard Thresholds**
- **CPU Usage**: >80% sustained (server monitoring standard)
- **Memory Usage**: >85% of available memory (prevents swap thrashing)
- **Frame Time**: >16.67ms (60 FPS target)
- **Frame Jitter**: >5ms variance (smooth gameplay standard)
- **AI Processing**: <10ms per frame budget (60% of frame time)

## Quick Start

### 1. Add the Plugin
```rust
use artificial_culture_rust::presentation::performance_alerts::PerformanceAlertsPlugin;

App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(PerformanceAlertsPlugin)
    .run();
```

### 2. Custom Configuration (Optional)
```rust
use artificial_culture_rust::presentation::performance_alerts::{
    PerformanceAlertsPlugin, PerformanceMonitorConfig
};

App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(PerformanceAlertsPlugin)
    .insert_resource(PerformanceMonitorConfig {
        cpu_usage_threshold: 70.0,  // Lower threshold for mobile
        target_fps: 30.0,           // 30 FPS target for mobile
        monitoring_interval_ms: 200, // Check every 200ms
        ..Default::default()
    })
    .run();
```

### 3. Handle Alerts (Optional)
```rust
fn performance_alert_handler(mut alert_events: EventReader<PerformanceAlert>) {
    for alert in alert_events.read() {
        match alert {
            PerformanceAlert::HighCpuUsage { current, threshold, duration_ms } => {
                warn!("High CPU: {:.1}% (threshold: {:.1}%) for {}ms", 
                      current, threshold, duration_ms);
            },
            PerformanceAlert::HighFrameTime { current_ms, target_ms, fps_equivalent } => {
                warn!("High frame time: {:.2}ms (target: {:.2}ms, ~{:.1} FPS)", 
                      current_ms, target_ms, fps_equivalent);
            },
            _ => warn!("Performance alert: {:?}", alert),
        }
    }
}
```

## Log Analysis

### View Real-time Alerts
```bash
# Monitor alerts as they happen
tail -f logs/performance_alerts_*.jsonl

# Filter specific alert types
grep "HighFrameTime" logs/performance_alerts_*.jsonl | jq .
```

### Analyze Performance Trends
```bash
# Extract FPS data over time
cat logs/performance_alerts_*.jsonl | \
  jq 'select(.alert_type == "LowFpsDrops") | {timestamp: .timestamp, fps: .alert.current_fps}' | \
  jq -s 'sort_by(.timestamp)'

# Count alert types
cat logs/performance_alerts_*.jsonl | jq -r '.alert_type' | sort | uniq -c
```

### Performance Dashboard (Future)
The JSON logs can be imported into monitoring tools like:
- **Grafana**: For real-time dashboards
- **Elasticsearch**: For log analysis and search
- **Custom Tools**: Parse JSON for application-specific analysis

## Testing

The performance monitoring system includes comprehensive tests:

```bash
# Run all performance monitoring tests
cargo test tests::presentation --lib

# Run specific test modules
cargo test tests::presentation::performance_alerts --lib
cargo test tests::presentation::performance_logging --lib
```

The test suite covers:
- Plugin initialization and configuration
- Alert type creation and serialization
- JSON logging functionality
- System integration and error handling

## Configuration Reference

### PerformanceMonitorConfig Fields

| Field | Default | Description |
|-------|---------|-------------|
| `cpu_usage_threshold` | 80.0 | CPU usage % that triggers alerts |
| `memory_usage_threshold` | 85.0 | Memory usage % that triggers alerts |
| `gpu_usage_threshold` | 90.0 | GPU usage % that triggers alerts |
| `target_frame_time_ms` | 16.67 | Target frame time (60 FPS) |
| `target_fps` | 60.0 | Target FPS for performance monitoring |
| `frame_jitter_threshold_ms` | 5.0 | Frame time variance threshold |
| `monitoring_interval_ms` | 100 | How often to check metrics (10Hz) |
| `log_buffer_size` | 100 | Alerts to buffer before flushing |
| `flush_interval_ms` | 5000 | How often to flush logs to disk |

### Alert Types

| Alert Type | Trigger Condition | Severity |
|------------|------------------|----------|
| `HighCpuUsage` | CPU > threshold for sustained period | Warning/Critical |
| `HighMemoryUsage` | Memory > threshold | Critical/Severe |
| `HighFrameTime` | Frame time > target | Warning/Severe |
| `LowFpsDrops` | FPS < target for >1 second | Warning/Critical |
| `HighFrameJitter` | Frame variance > threshold | Warning |
| `EntityCountPerformanceImpact` | Entity count causing degradation | Warning |

## Performance Impact

The monitoring system itself is designed for minimal overhead:
- **Target Impact**: <1% of total frame time
- **Sampling Rate**: 10Hz reduces CPU usage while maintaining responsiveness
- **Efficient Storage**: Ring buffers for historical data
- **Conditional Logging**: Only writes when alerts are triggered

## Research & Standards

The thresholds and monitoring approach are based on:
- **Game Development**: Unity performance guidelines, Unreal Engine best practices
- **Server Monitoring**: DevOps standards for production systems
- **Academic Research**: Agent-based simulation performance studies
- **Industry Standards**: Real-time application performance requirements

See `docs/performance_monitoring_research.md` for detailed research and references.

## Troubleshooting

### Common Issues

**No alerts appearing**: Check that the monitoring interval isn't too high and thresholds aren't too strict for your hardware.

**Too many alerts**: Increase thresholds or monitoring intervals to reduce sensitivity.

**Log files not created**: Ensure the `logs/` directory exists and is writable.

**Performance impact**: Reduce monitoring frequency or disable specific alert types.

### Debug Mode
Enable debug logging to see monitoring activity:
```rust
// In your logging configuration
RUST_LOG=debug cargo run
```

## Future Enhancements

- **GPU Monitoring**: Direct GPU usage monitoring (currently estimated)
- **Disk I/O Monitoring**: Real disk I/O usage tracking
- **Network Monitoring**: Network usage for multiplayer scenarios
- **Predictive Alerts**: Machine learning to predict performance issues
- **Auto-optimization**: Automatic performance tuning suggestions
- **Web Dashboard**: Real-time web interface for monitoring
