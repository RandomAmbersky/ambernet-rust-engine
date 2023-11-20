use asn_core::math::Size2D;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Default)]
pub struct Image {
	pub source: String,
	pub size: Size2D<u32>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
struct ImageInitial {
	#[serde(rename = "@source")]
	source: String,
	#[serde(rename = "@width")]
	width: u32,
	#[serde(rename = "@height")]
	height: u32,
}

pub fn image_deserialize<'de, D>(deserializer: D) -> Result<Image, D::Error>
	where
		D: Deserializer<'de>,
{
	let initial: ImageInitial = ImageInitial::deserialize(deserializer)?;

	Ok(Image {
		source: initial.source,
		size: Size2D {
			width: initial.width,
			height: initial.height,
		},
	})
}

pub fn image_serialize<S>(image: &Image, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
{
	let initial: ImageInitial = ImageInitial {
		source: image.source.clone(),
		width: image.size.width,
		height: image.size.height,
	};

	initial.serialize(serializer)
}
