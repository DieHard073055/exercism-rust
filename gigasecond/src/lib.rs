use time::PrimitiveDateTime as DateTime;
use time::OffsetDateTime as OffsetDateTime;


// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let offset_date = OffsetDateTime::from_unix_timestamp(start.assume_utc().unix_timestamp() + 1000000000).unwrap();
    DateTime::new(offset_date.date(), offset_date.time())
}
