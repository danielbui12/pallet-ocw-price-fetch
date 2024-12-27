use super::*;

impl<T: Config> Pallet<T> {
	/// A helper function to fetch the price and send a raw unsigned transaction.
	pub fn fetch_price_and_send_raw_unsigned(
		block_number: BlockNumberFor<T>,
	) -> Result<(), &'static str> {
        // Make sure we don't fetch the price if unsigned transaction is going to be rejected
		// anyway.
		let next_unsigned_at = NextUnsignedAt::<T>::get();
		if next_unsigned_at > block_number {
			return Err("Too early to send unsigned transaction")
		}

		let price = Self::fetch_price().map_err(|_| "Failed to fetch price")?;
        // TODO:
		// SubmitTransaction::<T, Call<T>>::submit_transaction(
        //     Call::submit_price_unsigned { block_number, price }
        // ).map_err(|()| "Unable to submit unsigned transaction.")?;

		Ok(())
	}


	/// Fetch current price and return the result in cents.
	fn fetch_price() -> Result<u32, http::Error> {
		let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(2_000));
		let request =
			http::Request::get("https://min-api.cryptocompare.com/data/price?fsym=BTC&tsyms=USD");
		let pending = request.deadline(deadline).send().map_err(|_| http::Error::IoError)?;
    	let response = pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;
		if response.code != 200 {
			log::warn!("Unexpected status code: {}", response.code);
			return Err(http::Error::Unknown)
		}

		let body = response.body().collect::<Vec<u8>>();

		// Create a str slice from the body.
		let body_str = alloc::str::from_utf8(&body).map_err(|_| {
			log::warn!("No UTF8 body");
			http::Error::Unknown
		})?;

		let price = match Self::parse_price(body_str) {
			Some(price) => Ok(price),
			None => {
				log::warn!("Unable to extract price from the response: {:?}", body_str);
				Err(http::Error::Unknown)
			},
		}?;

		log::warn!("Got price: {} cents", price);

		Ok(price)
	}

	fn parse_price(price_str: &str) -> Option<u32> {
		let val = serde_json::from_str(price_str);
		let price = match val.ok()? {
			JsonValue::Object(obj) => {
                // TODO
				// let (_, v) = obj.into_iter().find(|(k, _)| k.iter().copied().eq("USD".chars()))?;
				// match v {
				// 	JsonValue::Number(number) => number,
				// 	_ => return None,
				// }
                0
			},
			_ => return None,
		};

        // TODO
        Some(12u32)
		// let exp = price.fraction_length.saturating_sub(2);
		// Some(price.integer as u32 * 100 + (price.fraction / 10_u64.pow(exp)) as u32)
	}

	/// Add new price to the list.
	pub fn add_price(maybe_who: Option<T::AccountId>, price: u32) {
		log::info!("Adding to the average: {}", price);
		<Prices<T>>::mutate(|prices| {
			if prices.try_push(price).is_err() {
				prices[(price % T::MaxPrices::get()) as usize] = price;
			}
		});

		let average = Self::average_price()
			.expect("The average is not empty, because it was just mutated; qed");
		log::info!("Current average price is: {}", average);
		// here we are raising the NewPrice event
		Self::deposit_event(Event::NewPrice { price, maybe_who });
	}

	/// Calculate current average price.
	pub fn average_price() -> Option<u32> {
		let prices = Prices::<T>::get();
		if prices.is_empty() {
			None
		} else {
			Some(prices.iter().fold(0_u32, |a, b| a.saturating_add(*b)) / prices.len() as u32)
		}
	}

	pub fn validate_transaction_parameters(
		block_number: &BlockNumberFor<T>,
		new_price: &u32,
	) -> TransactionValidity {
        // Now let's check if the transaction has any chance to succeed.
		let next_unsigned_at = NextUnsignedAt::<T>::get();
		if &next_unsigned_at > block_number {
			return InvalidTransaction::Stale.into()
		}

		let current_block = <frame_system::Pallet<T>>::block_number();
		if &current_block < block_number {
			return InvalidTransaction::Future.into()
		}
		let avg_price = Self::average_price()
			.map(|price| if &price > new_price { price - new_price } else { new_price - price })
			.unwrap_or(0);

		ValidTransaction::with_tag_prefix("OCWPriceFetch")
			.priority(T::UnsignedPriority::get().saturating_add(avg_price as _))
			.and_provides(next_unsigned_at)
			.longevity(5)
			.propagate(true)
			.build()
	}
}
