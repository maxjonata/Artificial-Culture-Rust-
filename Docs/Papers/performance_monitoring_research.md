# Performance Monitoring Research and Implementation

## Industry-Standard Baseline Values

### System Resource Thresholds

Based on research from game development, server monitoring, and real-time simulation best practices:

#### CPU Usage
- **Normal Operation**: 20-40% for typical game applications
- **Warning Threshold**: 80% sustained usage
- **Critical Threshold**: 90%+ sustained usage
- **Rationale**: Leaves headroom for frame spikes and background processes. Based on server monitoring standards where 80% CPU is considered the upper limit for stable operation.

#### Memory (RAM) Usage
- **Normal Operation**: 60-70% of available system memory
- **Warning Threshold**: 85% of available memory
- **Critical Threshold**: 95% of available memory
- **Rationale**: Prevents swap thrashing which causes severe performance degradation. Game development best practice is to stay well below memory limits to avoid garbage collection spikes.

#### GPU Usage
- **Normal Operation**: 70-85% for graphics-intensive applications
- **Warning Threshold**: 90% sustained usage
- **Critical Threshold**: 95%+ sustained usage
- **Rationale**: GPU usage can spike to 100% briefly without issues, but sustained high usage indicates potential bottlenecks. Allows headroom for complex scenes.

#### Disk I/O Usage
- **Normal Operation**: 20-40% utilization
- **Warning Threshold**: 80% utilization
- **Critical Threshold**: 90%+ utilization
- **Rationale**: High disk I/O can cause frame stuttering in real-time applications. Based on system administration best practices where 80% I/O utilization indicates potential bottlenecks.

### Frame Performance Targets

#### Frame Time Budgets (60 FPS Target)
- **Total Frame Budget**: 16.67ms (60 FPS)
- **Rendering Budget**: 8-10ms (50-60% of frame time)
- **AI Processing Budget**: 3-5ms (18-30% of frame time)
- **Physics Budget**: 2-3ms (12-18% of frame time)
- **Other Systems Budget**: 1-2ms (6-12% of frame time)

#### Frame Time Variance
- **Acceptable Jitter**: <2ms standard deviation
- **Warning Threshold**: 5ms standard deviation
- **Critical Threshold**: 10ms+ standard deviation
- **Rationale**: Consistent frame times are more important than peak performance for user experience.

#### FPS Drop Tolerance
- **Brief Drops**: <30 FPS for <500ms acceptable
- **Warning**: <60 FPS for >1 second
- **Critical**: <30 FPS for >1 second
- **Severe**: <15 FPS for any duration

### Agent-Based Simulation Specific Metrics

#### Entity Count Scaling
- **Baseline**: 100-500 entities for initial testing
- **Linear Scaling**: Performance should degrade linearly with entity count
- **Warning Threshold**: 2x entity count with >50% performance degradation
- **Critical Threshold**: Performance cliff (exponential degradation)

#### Memory Usage Patterns
- **Per-Entity Memory**: 1-10KB per agent (depending on complexity)
- **System Memory**: 50-200MB for core systems
- **Total Budget**: <2GB for simulation (leaves room for OS and other processes)

#### AI System Performance
- **Individual System Budget**: <5ms per system per frame
- **Total AI Budget**: <10ms per frame (60% of 60 FPS budget)
- **Batch Processing**: Large operations should be spread across multiple frames

## Implementation Details

### Monitoring Frequency
- **High-Frequency Metrics**: Frame time, FPS (every frame)
- **Medium-Frequency Metrics**: CPU, Memory, GPU (every 100ms)
- **Low-Frequency Metrics**: Disk I/O, Entity count correlation (every 1000ms)

### Alert Severity Levels
1. **Warning**: Performance concern but not critical
2. **Critical**: Immediate performance impact affecting user experience
3. **Severe**: Simulation integrity at risk, may cause crashes or data loss

### Logging Strategy
- **JSON Lines Format**: Each alert is a separate JSON object on its own line
- **Timestamped Files**: New log file for each application run
- **Buffered Writing**: Alerts buffered and flushed periodically to reduce I/O overhead
- **Crash Resilience**: Buffer flushed immediately on critical alerts

### Performance Impact of Monitoring
- **Target Overhead**: <1% of total frame time
- **Sampling Strategy**: 10Hz monitoring reduces overhead while maintaining responsiveness
- **Conditional Logging**: Only log when thresholds are exceeded
- **Efficient Data Structures**: Ring buffers for historical data

## Research Sources

1. **Unity Performance Best Practices**: Frame time budgets and profiling guidelines
2. **Server Monitoring Standards**: CPU, Memory, and I/O thresholds from DevOps practices
3. **Game Development Forums**: Real-world performance targets from shipped games
4. **Academic Papers**: Agent-based simulation scaling research
5. **Bevy Engine Documentation**: ECS performance characteristics and optimization guides

## Usage Examples

### Basic Setup
```rust
app.add_plugins(PerformanceAlertsPlugin);
```

### Custom Configuration
```rust
app.insert_resource(PerformanceMonitorConfig {
    cpu_usage_threshold: 75.0,  // Lower threshold for mobile
    target_fps: 30.0,           // 30 FPS target for mobile
    ..Default::default()
});
```

### Reading Logs
```bash
# View recent alerts
tail -f logs/performance_alerts_*.jsonl

# Parse specific alert types
grep "HighFrameTime" logs/performance_alerts_*.jsonl | jq .

# Analyze performance trends
cat logs/performance_alerts_*.jsonl | jq '.timestamp, .alert.current_fps' | paste - -
```

## Future Enhancements

1. **Machine Learning Integration**: Predict performance issues before they occur
2. **Adaptive Thresholds**: Adjust thresholds based on hardware capabilities
3. **Performance Regression Detection**: Compare against baseline performance profiles
4. **Real-time Dashboard**: Web interface for monitoring live performance metrics
5. **Automated Optimization**: Suggest or apply performance optimizations automatically
