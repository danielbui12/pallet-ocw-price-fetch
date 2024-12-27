use super::*;
use polkadot_sdk::{sp_io, sp_std};

impl<T: Config> Pallet<T> {
	/// Add new price to the list.
	pub fn add_price(payload: Vec<PricePayload>) {
        log::info!("payload len: {}", payload.len());
        for PricePayload { price, symbol } in payload.iter() {
            // Convert symbol bytes to string
            let symbol_str = sp_std::str::from_utf8(symbol)
                .expect("Invalid UTF-8 in symbol"); // Handle UTF-8 conversion safely
            
            // Log the price and symbol
            log::info!("Adding {} to the price: {}", symbol_str, price);
            
            // Insert the price into the storage
            <Prices<T>>::insert(symbol.clone(), price.clone());
    
            // Emit an event for the new price
            Self::deposit_event(Event::NewPrice(symbol.clone(), price.clone()));
        }
	}

	/// A helper function to fetch the price and send a raw unsigned transaction.
    pub fn safe_fetch_price<'a>(
        block_number: BlockNumberFor<T>,
        symbol: &'a [u8],
        remote_url: &'a [u8],
	) -> Result<Price, &'static str> {
        // Make sure we don't fetch the price if unsigned transaction is going to be rejected
		// anyway.
		let next_unsigned_at = NextUnsignedAt::<T>::get();        
		if next_unsigned_at > block_number {
			return Err("Too early to send unsigned transaction")
		}

		let price = Self::fetch_price(remote_url).map_err(|e| {
            log::error!("fetch price error: {:?}", e);
            "Failed to fetch price"
        })?;
        
        let symbol_str = sp_std::str::from_utf8(&symbol).unwrap();
        log::info!("{} got price: {} cents", symbol_str, price);
    
		Ok(price)
	}


	/// Fetch current price and return the result in cents.
	fn fetch_price<'a>(remote_url: &'a [u8]) -> Result<Price, http::Error> {
        let remote_url_str = sp_std::str::from_utf8(&remote_url)
            .map_err(|_| http::Error::Unknown)?;
  
		let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(2_000));
		let request =
			http::Request::get(remote_url_str);
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

		Ok(price)
	}

	fn parse_price(price_str: &str) -> Option<Price> {
        let val: JsonValue = serde_json::from_str(price_str).ok()?;
        let obj = val.as_object()?;
        let v = obj.get("USD")?;
        let price = match v {
            JsonValue::Number(num) => {
                if let Some(int_val) = num.as_u64() {
                    // If it's already an integer, scale it
                    int_val * NUMERATOR
                } else if let Some(float_val) = num.as_f64() {
                    // Convert floating-point to scaled integer
                    (float_val * NUMERATOR as f64) as u64
                } else {
                    return None; // Not a valid number
                }
            }
            _ => return None, // Not a number
        };    

		Some(price)
	}

    pub fn ocw_submit_tx(
        block_number: BlockNumberFor<T>,
        payload: Vec<PricePayload>,
    ) -> Result<(), &'static str>{
		SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(
            Call::submit_price_unsigned { payload, block_number }.into()
        ).map_err(|()| "Unable to submit unsigned transaction.")?;
        Ok(())
    }


	pub fn validate_transaction_parameters(
        block_number: &BlockNumberFor<T>
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

        ValidTransaction::with_tag_prefix("OCWPriceFetch")
			.priority(T::UnsignedPriority::get())
			.and_provides(next_unsigned_at)
			.longevity(5)
			.propagate(true)
			.build()
	}
}
