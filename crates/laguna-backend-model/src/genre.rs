use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// GitHub copilot generated.
/// Different genres for Movie-based content.
#[allow(missing_docs)]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, sqlx::Type, ToSchema)]
pub enum Genre {
  Action,
  Adventure,
  Animation,
  Biography,
  Comedy,
  Crime,
  Documentary,
  Drama,
  Family,
  Fantasy,
  FilmNoir,
  GameShow,
  History,
  Horror,
  Musical,
  Mystery,
  News,
  RealityTV,
  Romance,
  SciFi,
  Short,
  Sport,
  TalkShow,
  Thriller,
  War,
  Western,
}
