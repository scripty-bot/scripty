use num_format::{Locale, ToFormattedString};
use poise::CreateReply;
use serenity::builder::CreateEmbed;
use typesize::TypeSize;

use crate::{Context, Error};

#[poise::command(prefix_command, owners_only, hide_in_help)]
pub async fn cache_info(ctx: Context<'_>) -> Result<(), Error> {
	struct Field {
		name:          String,
		size:          usize,
		value:         String,
		is_collection: bool,
	}

	let serenity_cache = ctx.cache();
	let cache_stats = serenity_cache.get_size_details();

	let mut fields = Vec::new();
	for field in cache_stats {
		let name = format!("`{}`", field.name);
		let size = field.size.to_formatted_string(&Locale::en);
		if let Some(count) = field.collection_items {
			let (count, size_per) = if count == 0 {
				(Cow::Borrowed("0"), Cow::Borrowed("N/A"))
			} else {
				let count_fmt = count.to_formatted_string(&Locale::en);
				let mut size_per = (field.size / count).to_formatted_string(&Locale::en);
				size_per.push('b');

				(Cow::Owned(count_fmt), Cow::Owned(size_per))
			};

			fields.push(Field {
				name,
				size: field.size,
				is_collection: true,
				value: format!("Size: `{size}b`\nCount: `{count}`\nSize per model: `{size_per}`"),
			});
		} else {
			fields.push(Field {
				name,
				size: field.size,
				is_collection: false,
				value: format!("Size: `{size}b`"),
			});
		};
	}

	fields.sort_by_key(|field| field.size);
	fields.sort_by_key(|field| field.is_collection);
	fields.reverse();

	let embed = CreateEmbed::default()
		.title("Cache Statistics")
		.fields(fields.into_iter().map(|f| (f.name, f.value, true)));

	ctx.send(CreateReply::default().embed(embed)).await?;
	Ok(())
}
