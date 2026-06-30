use session_ledger::domain::bundle::{Bundle, BundleKind, ContinuationBundle};

/// Build a set of sample compiled bundles for development / demo purposes.
#[must_use]
pub fn sample_bundles() -> Vec<ContinuationBundle> {
    vec![
        ContinuationBundle {
            source_id: "forge-session-001".into(),
            bundles: vec![
                Bundle::new(
                    BundleKind::Acceptance,
                    serde_json::json!({
                        "ready": true,
                        "scope_sized": true,
                        "user_turns": 5,
                    }),
                ),
                Bundle::new(
                    BundleKind::Intent,
                    serde_json::json!({
                        "goal": "Fix login timeout regression after auth refactor",
                        "acceptance_signals": [
                            "all existing auth tests pass",
                            "session expiry extends beyond 30 min",
                            "user story AC-417 verified",
                        ],
                        "constraints": ["must not touch password reset flow", "must preserve MFA"],
                        "user_turn_count": 5,
                    }),
                ),
                Bundle::new(
                    BundleKind::Context,
                    serde_json::json!({
                        "cwd": "/home/dev/auth-service",
                        "title": "Login timeout fix",
                    }),
                ),
                Bundle::new(
                    BundleKind::Contract,
                    serde_json::json!({
                        "skipped_by": ["existing auth tests pass"],
                        "watch_files": ["src/auth/session.rs"],
                    }),
                ),
            ],
        },
        ContinuationBundle {
            source_id: "codex-session-003".into(),
            bundles: vec![
                Bundle::new(
                    BundleKind::Acceptance,
                    serde_json::json!({
                        "ready": true,
                        "scope_sized": true,
                        "user_turns": 12,
                    }),
                ),
                Bundle::new(
                    BundleKind::Intent,
                    serde_json::json!({
                        "goal": "Add usage-based billing to the API proxy layer",
                        "acceptance_signals": [
                            "stripe integration complete",
                            "rate limit headers reflect remaining quota",
                            "billing dashboard renders for admin users",
                        ],
                        "constraints": ["no credit card required for dev tier", "must log all billing events"],
                        "user_turn_count": 12,
                    }),
                ),
                Bundle::new(
                    BundleKind::Context,
                    serde_json::json!({
                        "cwd": "/home/dev/api-gateway",
                        "title": "Usage billing",
                    }),
                ),
                Bundle::new(
                    BundleKind::Contract,
                    serde_json::json!({
                        "skipped_by": ["existing billing tests pass"],
                        "watch_files": ["src/middleware/usage.rs", "src/billing/"],
                    }),
                ),
            ],
        },
        ContinuationBundle {
            source_id: "claude-session-007".into(),
            bundles: vec![
                Bundle::new(
                    BundleKind::Acceptance,
                    serde_json::json!({
                        "ready": true,
                        "scope_sized": true,
                        "user_turns": 3,
                    }),
                ),
                Bundle::new(
                    BundleKind::Intent,
                    serde_json::json!({
                        "goal": "Update CI pipeline to use self-hosted runner for ARM builds",
                        "acceptance_signals": [
                            "ARM artifacts publish to GHCR",
                            "build time under 15 min",
                            "x86_64 builds unaffected",
                        ],
                        "constraints": ["do not modify existing x86_64 workflow matrix", "runner must be ephemeral"],
                        "user_turn_count": 3,
                    }),
                ),
                Bundle::new(
                    BundleKind::Context,
                    serde_json::json!({
                        "cwd": "/home/dev/infra",
                        "title": "ARM CI runner",
                    }),
                ),
            ],
        },
    ]
}
