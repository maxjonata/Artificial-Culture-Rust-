# AI Behavior Analysis Hook

**Trigger**: When AI component files are saved
**Description**: Automatically analyzes AI behavior patterns and validates against project philosophy

## Hook Actions

1. **Personality Consistency Check**
   - Analyze if personality traits properly influence all systems
   - Verify Big Five traits are used consistently
   - Check for personality-driven behavior variations

2. **Communication Pipeline Validation**
   - Ensure internal state → expression → perception → interpretation flow
   - Validate information loss at each pipeline stage
   - Check for proper misunderstanding generation

3. **Continuous State Verification**
   - Confirm all social/emotional values use Normalized<f32>
   - Verify no discrete enums for emotional states
   - Check proper value clamping and validation

4. **Event-Driven Architecture Check**
   - Validate proper event emission and handling
   - Check system decoupling through events
   - Verify no direct component access between domains

5. **Performance Impact Analysis**
   - Estimate computational cost of changes
   - Flag potential performance bottlenecks
   - Suggest optimization opportunities

## Analysis Report

Generate report covering:
- Personality influence coverage across systems
- Communication pipeline integrity
- Type safety and validation compliance
- Event architecture adherence
- Performance implications

## Recommendations

Provide specific suggestions for:
- Missing personality modulations
- Pipeline gaps or perfect communication
- Type system violations
- Architectural improvements
- Performance optimizations