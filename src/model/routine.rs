//! A user-defined function or a stored procedure.
use crate::model::argument::Argument;
use crate::model::routine_reference::RoutineReference;
use crate::model::standard_sql_data_type::StandardSqlDataType;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Routine {
    /// Required. The type of routine.
    pub routine_type: RoutineType,
    /// Optional.
    pub arguments: Option<Vec<Argument>>,
    /// Optional if language = "SQL"; required otherwise. If absent, the return type is inferred from definition_body at query time in each query that references this routine. If present, then the evaluated result will be cast to the specified returned type at query time. For example, for the functions created with the following statements: * `CREATE FUNCTION Add(x FLOAT64, y FLOAT64) RETURNS FLOAT64 AS (x + y);` * `CREATE FUNCTION Increment(x FLOAT64) AS (Add(x, 1));` * `CREATE FUNCTION Decrement(x FLOAT64) RETURNS FLOAT64 AS (Add(x, -1));` The return_type is `{type_kind: "FLOAT64"}` for `Add` and `Decrement`, and is absent for `Increment` (inferred as FLOAT64 at query time). Suppose the function `Add` is replaced by `CREATE OR REPLACE FUNCTION Add(x INT64, y INT64) AS (x + y);` Then the inferred return type of `Increment` is automatically changed to INT64 at query time, while the return type of `Decrement` remains FLOAT64.
    pub return_type: Option<StandardSqlDataType>,
    /// Optional. Defaults to "SQL".
    pub language: Option<Language>,
    /// Required. The body of the routine. For functions, this is the expression in the AS clause. If language=SQL, it is the substring inside (but excluding) the parentheses. For example, for the function created with the following statement: `CREATE FUNCTION JoinLines(x string, y string) as (concat(x, "\n", y))` The definition_body is `concat(x, "\n", y)` (\n is not replaced with linebreak). If language=JAVASCRIPT, it is the evaluated string in the AS clause. For example, for the function created with the following statement: `CREATE FUNCTION f() RETURNS STRING LANGUAGE js AS 'return "\n";\n'` The definition_body is `return "\n";\n` Note that both \n are replaced with linebreaks.
    pub definition_body: String,
    /// Output only. The time when this routine was created, in milliseconds since the epoch.
    pub creation_time: Option<i64>,
    /// Optional. [Experimental] The determinism level of the JavaScript UDF if defined.
    pub determinism_level: Option<DeterminismLevel>,
    /// Optional. [Experimental] The description of the routine if defined.
    pub description: Option<String>,
    /// Optional. If language = "JAVASCRIPT", this field stores the path of the imported JAVASCRIPT libraries.
    pub imported_libraries: Option<Vec<String>>,
    /// Output only. A hash of this resource.
    pub etag: Option<String>,
    /// Required. Reference describing the ID of this routine.
    pub routine_reference: RoutineReference,
    /// Output only. The time when this routine was last modified, in milliseconds since the epoch.
    pub last_modified_time: Option<i64>,
}

/// Required. The type of routine.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RoutineType {
    ///
    RoutineTypeUnspecified,
    /// Non-builtin permanent scalar function.
    ScalarFunction,
    /// Stored procedure.
    Procedure,
}

/// Optional. Defaults to "SQL".
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Language {
    ///
    LanguageUnspecified,
    /// SQL language.
    Sql,
    /// JavaScript language.
    Javascript,
}

/// Optional. [Experimental] The determinism level of the JavaScript UDF if defined.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeterminismLevel {
    /// The determinism of the UDF is unspecified.
    DeterminismLevelUnspecified,
    /// The UDF is deterministic, meaning that 2 function calls with the same inputs always produce the same result, even across 2 query runs.
    Deterministic,
    /// The UDF is not deterministic.
    NotDeterministic,
}
