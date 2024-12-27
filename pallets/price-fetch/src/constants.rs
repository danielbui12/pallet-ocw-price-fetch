pub const WHITELIST_CRYPTO: [(&[u8], &[u8]); 4] = [
    (b"BTC", b"https://min-api.cryptocompare.com/data/price?fsym=BTC&tsyms=USD"),
    (b"ETH", b"https://min-api.cryptocompare.com/data/price?fsym=ETH&tsyms=USD"),
    (b"DOT", b"https://min-api.cryptocompare.com/data/price?fsym=DOT&tsyms=USD"),
    (b"USDT", b"https://min-api.cryptocompare.com/data/price?fsym=USDT&tsyms=USD"),
];
pub const NUMERATOR: u64 = 1_000_000_000; // 10^9
