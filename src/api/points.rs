use serde::Deserialize;

use crate::api::ENDPOINTS;
use crate::errors::RoboltError;
use crate::Robolt;

impl<State> Robolt<State> {
	pub async fn points(&self, user_id: u64, universe_id: u64) -> Result<u64, RoboltError> {
		self.request_builder(format!(
			"{}/v1/universes/{}/users/{}/all-time",
			ENDPOINTS.points, user_id, universe_id
		))
		.send::<AllTimeScore>()
		.await
		.map(|res| res.all_time_score)
	}
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AllTimeScore {
	all_time_score: u64,
}