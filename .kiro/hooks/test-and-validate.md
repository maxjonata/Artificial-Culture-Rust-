# Test and Validate Hook

**Trigger**: Manual button click
**Description**: Runs comprehensive tests and validation for AI systems

## Hook Actions

1. **Run Unit Tests**
   - Execute `cargo test` for all AI modules
   - Focus on component validation and system integration
   - Verify Normalized<f32> type safety

2. **Run Integration Tests**
   - Test full AI pipeline with sample agents
   - Validate personality-driven behavior differences
   - Check performance with 100+ agents

3. **Validate Emergent Behavior**
   - Spawn test scenario with conflicting personalities
   - Monitor for expected social dynamics
   - Check for misunderstanding cascade effects

4. **Performance Profiling**
   - Run performance benchmarks
   - Validate 60fps target with full AI load
   - Check memory usage patterns

5. **Debug UI Verification**
   - Ensure all components are visible in inspector
   - Validate real-time value updates
   - Check event emission and handling

## Success Criteria

- All unit tests pass
- Integration tests show expected emergent behaviors
- Performance targets met (60fps with 100+ agents)
- Debug UI shows all AI components correctly
- No memory leaks or performance degradation over time

## Failure Actions

- Generate detailed test report
- Highlight failing systems and components
- Suggest specific areas for investigation
- Create performance bottleneck analysis