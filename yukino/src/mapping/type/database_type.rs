use std::any::TypeId;

#[allow(dead_code)]
pub enum DatabaseType {
    SmallInteger,
    UnsignedSmallInteger,
    Integer,
    UnsignedInteger,
    BigInteger,
    UnsignedBigInteger,

    Float,
    Double,
    Decimal(usize, usize),

    Binary,

    Date,
    DateTime,
    Timestamp,

    Character,
    String,
    Text,

    CLOB,
    BLOB,

    Object(TypeId),
    Map,
    List
}