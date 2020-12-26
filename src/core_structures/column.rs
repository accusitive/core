#[derive(Debug, Clone)]
pub enum ColumnType {
    String(u32),
    Bool(u8),
    // Bytea,
    Integer8,
    Integer16,
    Integer32,
    Integer64,
    SignedInteger8,
    SignedInteger16,
    SignedInteger32,
    SignedInteger64,
    // JSON(u32),
    Array(Box<ColumnType>, u64),
    // SignedBigInteger(u64),
    // BigInteger(u64),
}