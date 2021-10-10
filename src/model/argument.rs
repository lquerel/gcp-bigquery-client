//! Input/output argument of a function or a stored procedure.
use crate::model::standard_sql_data_type::StandardSqlDataType;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Argument {
    /// Optional. Specifies whether the argument is input or output. Can be set for procedures only.
    pub mode: Option<Mode>,
    /// Optional. The name of this argument. Can be absent for function return argument.
    pub name: Option<String>,
    /// Required unless argument_kind = ANY_TYPE.
    pub data_type: Option<StandardSqlDataType>,
    /// Optional. Defaults to FIXED_TYPE.
    pub argument_kind: Option<ArgumentKind>,
}

/// Optional. Specifies whether the argument is input or output. Can be set for procedures only.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Mode {
    ///
    ModeUnspecified,
    /// The argument is input-only.
    In,
    /// The argument is output-only.
    Out,
    /// The argument is both an input and an output.
    Inout,
}

/// Optional. Defaults to FIXED_TYPE.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ArgumentKind {
    ///
    ArgumentKindUnspecified,
    /// The argument is a variable with fully specified type, which can be a struct or an array, but not a table.
    FixedType,
    /// The argument is any type, including struct or array, but not a table. To be added: FIXED_TABLE, ANY_TABLE
    AnyType,
}
