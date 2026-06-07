use tracewake_content::fixtures::{self, GoldenFixture};
use tracewake_core::ids::ActorId;

const DEFAULT_FIXTURE_ID: &str = "strongbox_001";
const DEFAULT_ACTOR_ID: &str = "actor_tomas";

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Launch {
    Run {
        golden: Box<GoldenFixture>,
        actor_id: ActorId,
    },
    List,
    Help,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LaunchError {
    UnknownFixture(String),
    UnknownActor {
        fixture_id: String,
        actor_id: String,
        available: Vec<String>,
    },
    TooManyArgs,
    BadActorId(String),
}

pub fn resolve(args: &[String]) -> Result<Launch, LaunchError> {
    match args {
        [] => {
            let golden = fixtures::by_id(DEFAULT_FIXTURE_ID)
                .expect("default fixture must exist in fixture catalog");
            let actor_id = ActorId::new(DEFAULT_ACTOR_ID).expect("default actor id must be valid");
            debug_assert!(golden
                .fixture
                .actors
                .iter()
                .any(|actor| actor.actor_id == actor_id));
            Ok(Launch::Run {
                golden: Box::new(golden),
                actor_id,
            })
        }
        [flag] if flag == "--list" || flag == "-l" => Ok(Launch::List),
        [flag] if flag == "--help" || flag == "-h" => Ok(Launch::Help),
        [fixture_id] => {
            let golden = resolve_fixture(fixture_id)?;
            let actor_id = first_actor(&golden).expect("golden fixtures must author actors");
            Ok(Launch::Run {
                golden: Box::new(golden),
                actor_id,
            })
        }
        [fixture_id, actor_id] => {
            let golden = resolve_fixture(fixture_id)?;
            let actor_id = ActorId::new(actor_id.clone())
                .map_err(|err| LaunchError::BadActorId(err.to_string()))?;
            if !golden
                .fixture
                .actors
                .iter()
                .any(|actor| actor.actor_id == actor_id)
            {
                return Err(LaunchError::UnknownActor {
                    fixture_id: fixture_id.clone(),
                    actor_id: actor_id.to_string(),
                    available: actor_ids(&golden),
                });
            }
            Ok(Launch::Run {
                golden: Box::new(golden),
                actor_id,
            })
        }
        _ => Err(LaunchError::TooManyArgs),
    }
}

pub fn render_catalog() -> String {
    fixtures::all()
        .into_iter()
        .map(|golden| {
            format!(
                "{}\t{}",
                golden.fixture.fixture_id.as_str(),
                actor_ids(&golden).join(", ")
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn usage() -> String {
    [
        "Usage:",
        "  cargo run -p tracewake-tui",
        "  cargo run -p tracewake-tui -- <fixture_id> [actor_id]",
        "  cargo run -p tracewake-tui -- --list",
        "  cargo run -p tracewake-tui -- --help",
    ]
    .join("\n")
}

pub fn render_error(error: &LaunchError) -> String {
    match error {
        LaunchError::UnknownFixture(fixture_id) => {
            format!("unknown fixture `{fixture_id}`\ntry `cargo run -p tracewake-tui -- --list`")
        }
        LaunchError::UnknownActor {
            fixture_id,
            actor_id,
            available,
        } => format!(
            "unknown actor `{actor_id}` for fixture `{fixture_id}`\navailable actors: {}",
            available.join(", ")
        ),
        LaunchError::TooManyArgs => {
            format!("too many arguments\n{}", usage())
        }
        LaunchError::BadActorId(message) => {
            format!("invalid actor id: {message}")
        }
    }
}

fn resolve_fixture(fixture_id: &str) -> Result<GoldenFixture, LaunchError> {
    fixtures::by_id(fixture_id).ok_or_else(|| LaunchError::UnknownFixture(fixture_id.to_string()))
}

fn first_actor(golden: &GoldenFixture) -> Option<ActorId> {
    golden
        .fixture
        .actors
        .first()
        .map(|actor| actor.actor_id.clone())
}

fn actor_ids(golden: &GoldenFixture) -> Vec<String> {
    golden
        .fixture
        .actors
        .iter()
        .map(|actor| actor.actor_id.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn resolve_defaults_to_strongbox_tomas() {
        let launch = resolve(&[]).unwrap();

        match launch {
            Launch::Run { golden, actor_id } => {
                assert_eq!(golden.fixture.fixture_id.as_str(), "strongbox_001");
                assert_eq!(actor_id.as_str(), "actor_tomas");
            }
            other => panic!("expected run launch, got {other:?}"),
        }
    }

    #[test]
    fn resolve_fixture_binds_first_authored_actor() {
        let launch = resolve(&args(&["ordinary_workday_001"])).unwrap();

        match launch {
            Launch::Run { golden, actor_id } => {
                assert_eq!(golden.fixture.fixture_id.as_str(), "ordinary_workday_001");
                assert_eq!(actor_id, golden.fixture.actors.first().unwrap().actor_id);
            }
            other => panic!("expected run launch, got {other:?}"),
        }
    }

    #[test]
    fn resolve_fixture_and_actor_binds_named_actor() {
        let launch = resolve(&args(&["debug_attach_001", "actor_jules"])).unwrap();

        match launch {
            Launch::Run { golden, actor_id } => {
                assert_eq!(golden.fixture.fixture_id.as_str(), "debug_attach_001");
                assert_eq!(actor_id.as_str(), "actor_jules");
            }
            other => panic!("expected run launch, got {other:?}"),
        }
    }

    #[test]
    fn resolve_unknown_fixture_reports_id() {
        assert_eq!(
            resolve(&args(&["missing_fixture"])),
            Err(LaunchError::UnknownFixture("missing_fixture".to_string()))
        );
    }

    #[test]
    fn resolve_unknown_actor_reports_available_actor_ids() {
        let error = resolve(&args(&["strongbox_001", "actor_mara"])).unwrap_err();

        assert_eq!(
            error,
            LaunchError::UnknownActor {
                fixture_id: "strongbox_001".to_string(),
                actor_id: "actor_mara".to_string(),
                available: vec!["actor_elena".to_string(), "actor_tomas".to_string()],
            }
        );
    }

    #[test]
    fn resolve_rejects_malformed_actor_id() {
        assert!(matches!(
            resolve(&args(&["strongbox_001", "Actor Tomas"])),
            Err(LaunchError::BadActorId(_))
        ));
    }

    #[test]
    fn resolve_list_and_help_flags() {
        assert_eq!(resolve(&args(&["--list"])), Ok(Launch::List));
        assert_eq!(resolve(&args(&["-l"])), Ok(Launch::List));
        assert_eq!(resolve(&args(&["--help"])), Ok(Launch::Help));
        assert_eq!(resolve(&args(&["-h"])), Ok(Launch::Help));
    }

    #[test]
    fn resolve_rejects_too_many_args() {
        assert_eq!(
            resolve(&args(&["strongbox_001", "actor_tomas", "extra"])),
            Err(LaunchError::TooManyArgs)
        );
    }

    #[test]
    fn catalog_includes_known_fixture_and_actor() {
        let catalog = render_catalog();

        assert!(catalog.contains("strongbox_001"));
        assert!(catalog.contains("actor_tomas"));
    }
}
