# SQL Function Parser Implementation Plan

## Project Overview

**Objective**: Add full support for SQL function parsing to the sql_parser Elixir library.

**Current State**:
- The codebase currently uses sqlparser v0.30.0
- SQL functions (scalar, aggregate, window) are marked as `NotImplemented` in `native/sqlparser_parse/src/datatypes.rs:1253`
- The upgrade-sqlparser branch has changes to upgrade to sqlparser v0.56.0

**Target State**:
- Full support for scalar functions (e.g., `UPPER()`, `CONCAT()`, `LENGTH()`)
- Full support for aggregate functions (e.g., `COUNT()`, `SUM()`, `AVG()`)
- Full support for window functions (e.g., `ROW_NUMBER() OVER()`, `RANK() OVER()`)
- Support for function modifiers (DISTINCT, FILTER, ORDER BY, etc.)

## Architecture Analysis

### sqlparser v0.56.0 Function AST Structure

The Function struct in sqlparser v0.56.0 has the following structure:

```rust
pub struct Function {
    pub name: ObjectName,
    pub uses_odbc_syntax: bool,
    pub parameters: FunctionArguments,
    pub args: FunctionArguments,
    pub filter: Option<Box<Expr>>,
    pub null_treatment: Option<NullTreatment>,
    pub over: Option<WindowType>,
}

pub enum FunctionArguments {
    None,
    Subquery(Box<Query>),
    List(FunctionArgumentList),
}

pub struct FunctionArgumentList {
    pub duplicate_treatment: Option<DuplicateTreatment>,
    pub args: Vec<FunctionArg>,
    pub clauses: Vec<FunctionArgumentClause>,
}

pub enum FunctionArg {
    Named { name: Ident, arg: FunctionArgExpr, operator: FunctionArgOperator },
    ExprNamed { name: Expr, arg: FunctionArgExpr, operator: FunctionArgOperator },
    Unnamed(FunctionArgExpr),
}

pub enum FunctionArgExpr {
    Expr(Expr),
    QualifiedWildcard(ObjectName),
    Wildcard,
}
```

## Implementation Phases

### Phase 1: Update Elixir Struct Definitions

**File**: `lib/sql_parser/document.ex`

**Action**: Add the following struct definitions at the end of the file (after line 128):

```elixir
# Core Function struct
defmodule SqlParser.Function do
  defstruct [:name, :uses_odbc_syntax, :parameters, :args, :filter, :null_treatment, :over]
end

# Function Arguments
defmodule SqlParser.FunctionArguments do
  defstruct [:type, :value]  # type: :none | :subquery | :list
end

defmodule SqlParser.FunctionArgumentList do
  defstruct [:duplicate_treatment, :args, :clauses]
end

defmodule SqlParser.FunctionArg do
  defstruct [:type, :name, :arg, :operator]  # type: :named | :expr_named | :unnamed
end

defmodule SqlParser.FunctionArgExpr do
  defstruct [:type, :value]  # type: :expr | :qualified_wildcard | :wildcard
end

# Window Functions
defmodule SqlParser.WindowType do
  defstruct [:type, :value]  # type: :window_spec | :named_window
end

defmodule SqlParser.WindowSpec do
  defstruct [:window_name, :partition_by, :order_by, :window_frame]
end

defmodule SqlParser.WindowFrame do
  defstruct [:units, :start_bound, :end_bound]
end

defmodule SqlParser.WindowFrameBound do
  defstruct [:type, :value]
end

# Function Argument Clauses
defmodule SqlParser.FunctionArgumentClause do
  defstruct [:type, :value]  # type: :ignore_or_respect_nulls | :order_by | :limit | etc.
end
```

**Notes for AI**:
- Elixir uses atoms for enum-like values (e.g., `:distinct`, `:all`, `:none`)
- Each struct should be a separate `defmodule`
- Fields can have default values, but for these structs they should all be explicit

### Phase 2: Add Rust Type Definitions

**File**: `native/sqlparser_parse/src/datatypes.rs`

**Action 2.1**: Add atom definitions (after line 38, in the atoms section):

```rust
mod function_atoms {
    rustler::atoms! {
        // Core function type
        function,

        // FunctionArguments types
        none,
        subquery,
        list,

        // FunctionArg types
        named,
        expr_named,
        unnamed,

        // FunctionArgExpr types
        expr,
        qualified_wildcard,
        wildcard,

        // FunctionArgOperator
        equals,
        right_arrow,
        assignment,
        colon,
        value,

        // DuplicateTreatment
        distinct,
        all,

        // NullTreatment
        ignore_nulls,
        respect_nulls,

        // WindowType
        window_spec,
        named_window,

        // WindowFrameUnits
        rows,
        range,
        groups,

        // WindowFrameBound types
        current_row,
        preceding,
        following,
        unbounded_preceding,
        unbounded_following,

        // FunctionArgumentClause types
        ignore_or_respect_nulls,
        order_by,
        limit,
        on_overflow,
        having_min,
        having_max,
        separator
    }
}
```

**Action 2.2**: Add struct definitions (before the final `Statement` enum, around line 1440):

```rust
// =============================================================================
// Function Support Structures
// =============================================================================

#[derive(NifStruct, Clone)]
#[module = "SqlParser.Function"]
pub struct Function {
    pub name: ObjectName,
    pub uses_odbc_syntax: bool,
    pub parameters: FunctionArguments,
    pub args: FunctionArguments,
    pub filter: Option<Box<Expr>>,
    pub null_treatment: Option<Atom>,
    pub over: Option<WindowType>,
}

#[derive(NifStruct, Clone)]
#[module = "SqlParser.FunctionArguments"]
pub struct FunctionArguments {
    pub r#type: Atom,
    pub value: FunctionArgumentsValue,
}

#[derive(NifUntaggedEnum, Clone)]
pub enum FunctionArgumentsValue {
    None(Atom),
    Subquery(Box<Query>),
    List(FunctionArgumentList),
}

#[derive(NifStruct, Clone)]
#[module = "SqlParser.FunctionArgumentList"]
pub struct FunctionArgumentList {
    pub duplicate_treatment: Option<Atom>,
    pub args: Vec<FunctionArg>,
    pub clauses: Vec<FunctionArgumentClause>,
}

#[derive(NifStruct, Clone)]
#[module = "SqlParser.FunctionArg"]
pub struct FunctionArg {
    pub r#type: Atom,
    pub name: Option<FunctionArgName>,
    pub arg: FunctionArgExpr,
    pub operator: Option<Atom>,
}

#[derive(NifUntaggedEnum, Clone)]
pub enum FunctionArgName {
    Ident(Ident),
    Expr(Expr),
}

#[derive(NifStruct, Clone)]
#[module = "SqlParser.FunctionArgExpr"]
pub struct FunctionArgExpr {
    pub r#type: Atom,
    pub value: FunctionArgExprValue,
}

#[derive(NifUntaggedEnum, Clone)]
pub enum FunctionArgExprValue {
    Expr(Expr),
    QualifiedWildcard(ObjectName),
    Wildcard(Atom),
}

#[derive(NifStruct, Clone)]
#[module = "SqlParser.WindowType"]
pub struct WindowType {
    pub r#type: Atom,
    pub value: WindowTypeValue,
}

#[derive(NifUntaggedEnum, Clone)]
pub enum WindowTypeValue {
    WindowSpec(WindowSpec),
    NamedWindow(Ident),
}

#[derive(NifStruct, Clone)]
#[module = "SqlParser.WindowSpec"]
pub struct WindowSpec {
    pub window_name: Option<Ident>,
    pub partition_by: Vec<Expr>,
    pub order_by: Vec<OrderByExpr>,
    pub window_frame: Option<WindowFrame>,
}

#[derive(NifStruct, Clone)]
#[module = "SqlParser.WindowFrame"]
pub struct WindowFrame {
    pub units: Atom,
    pub start_bound: WindowFrameBound,
    pub end_bound: Option<WindowFrameBound>,
}

#[derive(NifStruct, Clone)]
#[module = "SqlParser.WindowFrameBound"]
pub struct WindowFrameBound {
    pub r#type: Atom,
    pub value: Option<Box<Expr>>,
}

#[derive(NifStruct, Clone)]
#[module = "SqlParser.FunctionArgumentClause"]
pub struct FunctionArgumentClause {
    pub r#type: Atom,
    pub value: FunctionArgumentClauseValue,
}

#[derive(NifUntaggedEnum, Clone)]
pub enum FunctionArgumentClauseValue {
    IgnoreOrRespectNulls(Atom),
    OrderBy(Vec<OrderByExpr>),
    Limit(Expr),
    NotImplemented(Atom),
}
```

**Action 2.3**: Update ExprEnum to include Function (around line 1018):

```rust
#[derive(NifUntaggedEnum)]
#[derive(Clone)]
pub enum ExprEnum {
    Identifier(Ident),
    CompoundIdentifier(Vec<Ident>),
    CompositeAccess(CompositeAccess),
    UnaryOp(UnaryOp),
    IsFalse(Box<Expr>),
    IsNotFalse(Box<Expr>),
    IsTrue(Box<Expr>),
    IsNotTrue(Box<Expr>),
    IsNull(Box<Expr>),
    IsNotNull(Box<Expr>),
    IsUnknown(Box<Expr>),
    IsNotUnknown(Box<Expr>),
    InList(InList),
    InSubquery(InSubquery),
    InUnnest(InUnnest),
    Between(Between),
    BinaryOp(BinaryOp),
    Like(Like),
    ILike(ILike),
    SimilarTo(SimilarTo),
    AnyOp(Box<Expr>),
    AllOp(Box<Expr>),
    Nested(Box<Expr>),
    Value(Value),
    Function(Function),  // ADD THIS LINE
    NotImplemented(Atom),
}
```

### Phase 3: Implement Parsing Direction (sqlparser → Elixir)

**File**: `native/sqlparser_parse/src/datatypes.rs`

**Action 3.1**: Update Expr::new() to handle Function (replace line 1253):

In the `impl Expr` section, in the `pub fn new()` method, replace:

```rust
            | sqlparser::ast::Expr::Function(_)
```

With:

```rust
            sqlparser::ast::Expr::Function(func) => Expr {
                r#type: function_atoms::function(),
                value: ExprEnum::Function(Function::from(func)),
            },
```

**Action 3.2**: Implement From<sqlparser::ast::Function> for Function (add after WindowFrameBound struct):

```rust
impl From<sqlparser::ast::Function> for Function {
    fn from(func: sqlparser::ast::Function) -> Self {
        Self {
            name: ObjectName {
                names: func.name.0
                    .iter()
                    .filter_map(|p| p.as_ident())
                    .map(|i| Ident::from(i.clone()))
                    .collect(),
            },
            uses_odbc_syntax: func.uses_odbc_syntax,
            parameters: FunctionArguments::from(func.parameters),
            args: FunctionArguments::from(func.args),
            filter: func.filter.map(|e| Box::new(Expr::new(*e))),
            null_treatment: func.null_treatment.map(|nt| match nt {
                sqlparser::ast::NullTreatment::IgnoreNulls => function_atoms::ignore_nulls(),
                sqlparser::ast::NullTreatment::RespectNulls => function_atoms::respect_nulls(),
            }),
            over: func.over.map(WindowType::from),
        }
    }
}

impl From<sqlparser::ast::FunctionArguments> for FunctionArguments {
    fn from(args: sqlparser::ast::FunctionArguments) -> Self {
        match args {
            sqlparser::ast::FunctionArguments::None => Self {
                r#type: function_atoms::none(),
                value: FunctionArgumentsValue::None(function_atoms::none()),
            },
            sqlparser::ast::FunctionArguments::Subquery(query) => Self {
                r#type: function_atoms::subquery(),
                value: FunctionArgumentsValue::Subquery(Box::new(Query::new(*query))),
            },
            sqlparser::ast::FunctionArguments::List(list) => Self {
                r#type: function_atoms::list(),
                value: FunctionArgumentsValue::List(FunctionArgumentList::from(list)),
            },
        }
    }
}

impl From<sqlparser::ast::FunctionArgumentList> for FunctionArgumentList {
    fn from(list: sqlparser::ast::FunctionArgumentList) -> Self {
        Self {
            duplicate_treatment: list.duplicate_treatment.map(|dt| match dt {
                sqlparser::ast::DuplicateTreatment::Distinct => function_atoms::distinct(),
                sqlparser::ast::DuplicateTreatment::All => function_atoms::all(),
            }),
            args: list.args.iter().map(|a| FunctionArg::from(a.clone())).collect(),
            clauses: list.clauses.iter().map(|c| FunctionArgumentClause::from(c.clone())).collect(),
        }
    }
}

impl From<sqlparser::ast::FunctionArg> for FunctionArg {
    fn from(arg: sqlparser::ast::FunctionArg) -> Self {
        match arg {
            sqlparser::ast::FunctionArg::Named { name, arg, operator } => Self {
                r#type: function_atoms::named(),
                name: Some(FunctionArgName::Ident(Ident::from(name))),
                arg: FunctionArgExpr::from(arg),
                operator: Some(match operator {
                    sqlparser::ast::FunctionArgOperator::Equals => function_atoms::equals(),
                    sqlparser::ast::FunctionArgOperator::RightArrow => function_atoms::right_arrow(),
                    sqlparser::ast::FunctionArgOperator::Assignment => function_atoms::assignment(),
                    sqlparser::ast::FunctionArgOperator::Colon => function_atoms::colon(),
                    sqlparser::ast::FunctionArgOperator::Value => function_atoms::value(),
                }),
            },
            sqlparser::ast::FunctionArg::ExprNamed { name, arg, operator } => Self {
                r#type: function_atoms::expr_named(),
                name: Some(FunctionArgName::Expr(Expr::new(name))),
                arg: FunctionArgExpr::from(arg),
                operator: Some(match operator {
                    sqlparser::ast::FunctionArgOperator::Equals => function_atoms::equals(),
                    sqlparser::ast::FunctionArgOperator::RightArrow => function_atoms::right_arrow(),
                    sqlparser::ast::FunctionArgOperator::Assignment => function_atoms::assignment(),
                    sqlparser::ast::FunctionArgOperator::Colon => function_atoms::colon(),
                    sqlparser::ast::FunctionArgOperator::Value => function_atoms::value(),
                }),
            },
            sqlparser::ast::FunctionArg::Unnamed(arg) => Self {
                r#type: function_atoms::unnamed(),
                name: None,
                arg: FunctionArgExpr::from(arg),
                operator: None,
            },
        }
    }
}

impl From<sqlparser::ast::FunctionArgExpr> for FunctionArgExpr {
    fn from(expr: sqlparser::ast::FunctionArgExpr) -> Self {
        match expr {
            sqlparser::ast::FunctionArgExpr::Expr(e) => Self {
                r#type: function_atoms::expr(),
                value: FunctionArgExprValue::Expr(Expr::new(e)),
            },
            sqlparser::ast::FunctionArgExpr::QualifiedWildcard(obj) => Self {
                r#type: function_atoms::qualified_wildcard(),
                value: FunctionArgExprValue::QualifiedWildcard(ObjectName {
                    names: obj.0.iter().filter_map(|p| p.as_ident()).map(|i| Ident::from(i.clone())).collect(),
                }),
            },
            sqlparser::ast::FunctionArgExpr::Wildcard => Self {
                r#type: function_atoms::wildcard(),
                value: FunctionArgExprValue::Wildcard(function_atoms::wildcard()),
            },
        }
    }
}

impl From<sqlparser::ast::WindowType> for WindowType {
    fn from(wt: sqlparser::ast::WindowType) -> Self {
        match wt {
            sqlparser::ast::WindowType::WindowSpec(spec) => Self {
                r#type: function_atoms::window_spec(),
                value: WindowTypeValue::WindowSpec(WindowSpec::from(spec)),
            },
            sqlparser::ast::WindowType::NamedWindow(name) => Self {
                r#type: function_atoms::named_window(),
                value: WindowTypeValue::NamedWindow(Ident::from(name)),
            },
        }
    }
}

impl From<sqlparser::ast::WindowSpec> for WindowSpec {
    fn from(spec: sqlparser::ast::WindowSpec) -> Self {
        Self {
            window_name: spec.window_name.map(Ident::from),
            partition_by: spec.partition_by.iter().map(|e| Expr::new(e.clone())).collect(),
            order_by: spec.order_by.iter().map(|o| OrderByExpr {
                expr: Expr::new(o.expr.clone()),
                asc: o.options.asc,
                nulls_first: o.options.nulls_first,
            }).collect(),
            window_frame: spec.window_frame.map(WindowFrame::from),
        }
    }
}

impl From<sqlparser::ast::WindowFrame> for WindowFrame {
    fn from(frame: sqlparser::ast::WindowFrame) -> Self {
        Self {
            units: match frame.units {
                sqlparser::ast::WindowFrameUnits::Rows => function_atoms::rows(),
                sqlparser::ast::WindowFrameUnits::Range => function_atoms::range(),
                sqlparser::ast::WindowFrameUnits::Groups => function_atoms::groups(),
            },
            start_bound: WindowFrameBound::from(frame.start_bound),
            end_bound: frame.end_bound.map(WindowFrameBound::from),
        }
    }
}

impl From<sqlparser::ast::WindowFrameBound> for WindowFrameBound {
    fn from(bound: sqlparser::ast::WindowFrameBound) -> Self {
        match bound {
            sqlparser::ast::WindowFrameBound::CurrentRow => Self {
                r#type: function_atoms::current_row(),
                value: None,
            },
            sqlparser::ast::WindowFrameBound::Preceding(None) => Self {
                r#type: function_atoms::unbounded_preceding(),
                value: None,
            },
            sqlparser::ast::WindowFrameBound::Preceding(Some(expr)) => Self {
                r#type: function_atoms::preceding(),
                value: Some(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::WindowFrameBound::Following(None) => Self {
                r#type: function_atoms::unbounded_following(),
                value: None,
            },
            sqlparser::ast::WindowFrameBound::Following(Some(expr)) => Self {
                r#type: function_atoms::following(),
                value: Some(Box::new(Expr::new(*expr))),
            },
        }
    }
}

impl From<sqlparser::ast::FunctionArgumentClause> for FunctionArgumentClause {
    fn from(clause: sqlparser::ast::FunctionArgumentClause) -> Self {
        match clause {
            sqlparser::ast::FunctionArgumentClause::IgnoreOrRespectNulls(nt) => Self {
                r#type: function_atoms::ignore_or_respect_nulls(),
                value: FunctionArgumentClauseValue::IgnoreOrRespectNulls(match nt {
                    sqlparser::ast::NullTreatment::IgnoreNulls => function_atoms::ignore_nulls(),
                    sqlparser::ast::NullTreatment::RespectNulls => function_atoms::respect_nulls(),
                }),
            },
            sqlparser::ast::FunctionArgumentClause::OrderBy(order_by) => Self {
                r#type: function_atoms::order_by(),
                value: FunctionArgumentClauseValue::OrderBy(
                    order_by.iter().map(|o| OrderByExpr {
                        expr: Expr::new(o.expr.clone()),
                        asc: o.options.asc,
                        nulls_first: o.options.nulls_first,
                    }).collect()
                ),
            },
            sqlparser::ast::FunctionArgumentClause::Limit(expr) => Self {
                r#type: function_atoms::limit(),
                value: FunctionArgumentClauseValue::Limit(Expr::new(expr)),
            },
            _ => Self {
                r#type: result_atoms::not_implemented(),
                value: FunctionArgumentClauseValue::NotImplemented(result_atoms::not_implemented()),
            },
        }
    }
}
```

### Phase 4: Implement Generation Direction (Elixir → sqlparser)

**File**: `native/sqlparser_parse/src/datatypes.rs`

**Action 4.1**: Update From<Expr> for sqlparser::ast::Expr (around line 112):

In the `impl From<Expr> for sqlparser::ast::Expr` section, add before the catch-all patterns:

```rust
            ExprEnum::Function(func) => {
                sqlparser::ast::Expr::Function(sqlparser::ast::Function::from(func))
            }
```

**Action 4.2**: Add reverse From implementations (add after the forward implementations):

```rust
impl From<Function> for sqlparser::ast::Function {
    fn from(func: Function) -> Self {
        sqlparser::ast::Function {
            name: sqlparser::ast::ObjectName::from(func.name),
            uses_odbc_syntax: func.uses_odbc_syntax,
            parameters: sqlparser::ast::FunctionArguments::from(func.parameters),
            args: sqlparser::ast::FunctionArguments::from(func.args),
            filter: func.filter.map(|e| Box::new(sqlparser::ast::Expr::from(*e))),
            null_treatment: func.null_treatment.map(|nt| {
                if nt == function_atoms::ignore_nulls() {
                    sqlparser::ast::NullTreatment::IgnoreNulls
                } else {
                    sqlparser::ast::NullTreatment::RespectNulls
                }
            }),
            over: func.over.map(sqlparser::ast::WindowType::from),
        }
    }
}

impl From<FunctionArguments> for sqlparser::ast::FunctionArguments {
    fn from(args: FunctionArguments) -> Self {
        match args.value {
            FunctionArgumentsValue::None(_) => sqlparser::ast::FunctionArguments::None,
            FunctionArgumentsValue::Subquery(query) => {
                sqlparser::ast::FunctionArguments::Subquery(Box::new(sqlparser::ast::Query::from(*query)))
            }
            FunctionArgumentsValue::List(list) => {
                sqlparser::ast::FunctionArguments::List(sqlparser::ast::FunctionArgumentList::from(list))
            }
        }
    }
}

impl From<FunctionArgumentList> for sqlparser::ast::FunctionArgumentList {
    fn from(list: FunctionArgumentList) -> Self {
        Self {
            duplicate_treatment: list.duplicate_treatment.map(|dt| {
                if dt == function_atoms::distinct() {
                    sqlparser::ast::DuplicateTreatment::Distinct
                } else {
                    sqlparser::ast::DuplicateTreatment::All
                }
            }),
            args: list.args.iter().map(|a| sqlparser::ast::FunctionArg::from(a.clone())).collect(),
            clauses: list.clauses.iter().map(|c| sqlparser::ast::FunctionArgumentClause::from(c.clone())).collect(),
        }
    }
}

impl From<FunctionArg> for sqlparser::ast::FunctionArg {
    fn from(arg: FunctionArg) -> Self {
        let operator = arg.operator.map(|op| {
            if op == function_atoms::equals() {
                sqlparser::ast::FunctionArgOperator::Equals
            } else if op == function_atoms::right_arrow() {
                sqlparser::ast::FunctionArgOperator::RightArrow
            } else if op == function_atoms::assignment() {
                sqlparser::ast::FunctionArgOperator::Assignment
            } else if op == function_atoms::colon() {
                sqlparser::ast::FunctionArgOperator::Colon
            } else {
                sqlparser::ast::FunctionArgOperator::Value
            }
        }).unwrap_or(sqlparser::ast::FunctionArgOperator::Equals);

        match arg.name {
            Some(FunctionArgName::Ident(ident)) => sqlparser::ast::FunctionArg::Named {
                name: sqlparser::ast::Ident::from(ident),
                arg: sqlparser::ast::FunctionArgExpr::from(arg.arg),
                operator,
            },
            Some(FunctionArgName::Expr(expr)) => sqlparser::ast::FunctionArg::ExprNamed {
                name: sqlparser::ast::Expr::from(expr),
                arg: sqlparser::ast::FunctionArgExpr::from(arg.arg),
                operator,
            },
            None => sqlparser::ast::FunctionArg::Unnamed(sqlparser::ast::FunctionArgExpr::from(arg.arg)),
        }
    }
}

impl From<FunctionArgExpr> for sqlparser::ast::FunctionArgExpr {
    fn from(expr: FunctionArgExpr) -> Self {
        match expr.value {
            FunctionArgExprValue::Expr(e) => sqlparser::ast::FunctionArgExpr::Expr(sqlparser::ast::Expr::from(e)),
            FunctionArgExprValue::QualifiedWildcard(obj) => {
                sqlparser::ast::FunctionArgExpr::QualifiedWildcard(sqlparser::ast::ObjectName::from(obj))
            }
            FunctionArgExprValue::Wildcard(_) => sqlparser::ast::FunctionArgExpr::Wildcard,
        }
    }
}

impl From<WindowType> for sqlparser::ast::WindowType {
    fn from(wt: WindowType) -> Self {
        match wt.value {
            WindowTypeValue::WindowSpec(spec) => {
                sqlparser::ast::WindowType::WindowSpec(sqlparser::ast::WindowSpec::from(spec))
            }
            WindowTypeValue::NamedWindow(name) => {
                sqlparser::ast::WindowType::NamedWindow(sqlparser::ast::Ident::from(name))
            }
        }
    }
}

impl From<WindowSpec> for sqlparser::ast::WindowSpec {
    fn from(spec: WindowSpec) -> Self {
        Self {
            window_name: spec.window_name.map(sqlparser::ast::Ident::from),
            partition_by: spec.partition_by.iter().map(|e| sqlparser::ast::Expr::from(e.clone())).collect(),
            order_by: spec.order_by.iter().map(|o| sqlparser::ast::OrderByExpr::from(o.clone())).collect(),
            window_frame: spec.window_frame.map(sqlparser::ast::WindowFrame::from),
        }
    }
}

impl From<WindowFrame> for sqlparser::ast::WindowFrame {
    fn from(frame: WindowFrame) -> Self {
        Self {
            units: if frame.units == function_atoms::rows() {
                sqlparser::ast::WindowFrameUnits::Rows
            } else if frame.units == function_atoms::range() {
                sqlparser::ast::WindowFrameUnits::Range
            } else {
                sqlparser::ast::WindowFrameUnits::Groups
            },
            start_bound: sqlparser::ast::WindowFrameBound::from(frame.start_bound),
            end_bound: frame.end_bound.map(sqlparser::ast::WindowFrameBound::from),
        }
    }
}

impl From<WindowFrameBound> for sqlparser::ast::WindowFrameBound {
    fn from(bound: WindowFrameBound) -> Self {
        if bound.r#type == function_atoms::current_row() {
            sqlparser::ast::WindowFrameBound::CurrentRow
        } else if bound.r#type == function_atoms::unbounded_preceding() {
            sqlparser::ast::WindowFrameBound::Preceding(None)
        } else if bound.r#type == function_atoms::preceding() {
            sqlparser::ast::WindowFrameBound::Preceding(bound.value.map(|e| Box::new(sqlparser::ast::Expr::from(*e))))
        } else if bound.r#type == function_atoms::unbounded_following() {
            sqlparser::ast::WindowFrameBound::Following(None)
        } else {
            sqlparser::ast::WindowFrameBound::Following(bound.value.map(|e| Box::new(sqlparser::ast::Expr::from(*e))))
        }
    }
}

impl From<FunctionArgumentClause> for sqlparser::ast::FunctionArgumentClause {
    fn from(clause: FunctionArgumentClause) -> Self {
        match clause.value {
            FunctionArgumentClauseValue::IgnoreOrRespectNulls(nt) => {
                sqlparser::ast::FunctionArgumentClause::IgnoreOrRespectNulls(
                    if nt == function_atoms::ignore_nulls() {
                        sqlparser::ast::NullTreatment::IgnoreNulls
                    } else {
                        sqlparser::ast::NullTreatment::RespectNulls
                    }
                )
            }
            FunctionArgumentClauseValue::OrderBy(order_by) => {
                sqlparser::ast::FunctionArgumentClause::OrderBy(
                    order_by.iter().map(|o| sqlparser::ast::OrderByExpr::from(o.clone())).collect()
                )
            }
            FunctionArgumentClauseValue::Limit(expr) => {
                sqlparser::ast::FunctionArgumentClause::Limit(sqlparser::ast::Expr::from(expr))
            }
            FunctionArgumentClauseValue::NotImplemented(_) => {
                // This shouldn't happen in practice, use a safe default
                sqlparser::ast::FunctionArgumentClause::IgnoreOrRespectNulls(
                    sqlparser::ast::NullTreatment::RespectNulls
                )
            }
        }
    }
}
```

### Phase 5: Add Comprehensive Tests

**File**: `test/sql_parser_test.exs`

**Action**: Add test cases at the end of the file:

```elixir
describe "function parsing" do
  test "scalar function - no args" do
    assert {:ok, [query]} = SqlParser.parse("SELECT CURRENT_TIMESTAMP")

    assert %SqlParser.Query{
      body: %SqlParser.Select{
        projection: [
          %SqlParser.UnnamedExpr{
            expr: %SqlParser.Expr{
              type: :function,
              value: %SqlParser.Function{
                name: %SqlParser.ObjectName{
                  names: [%SqlParser.Ident{value: "CURRENT_TIMESTAMP"}]
                },
                args: %SqlParser.FunctionArguments{type: :none}
              }
            }
          }
        ]
      }
    } = query
  end

  test "scalar function - with single arg" do
    assert {:ok, [query]} = SqlParser.parse("SELECT UPPER(name) FROM users")

    assert %SqlParser.Query{
      body: %SqlParser.Select{
        projection: [
          %SqlParser.UnnamedExpr{
            expr: %SqlParser.Expr{
              type: :function,
              value: %SqlParser.Function{
                name: %SqlParser.ObjectName{
                  names: [%SqlParser.Ident{value: "UPPER"}]
                },
                args: %SqlParser.FunctionArguments{
                  type: :list
                }
              }
            }
          }
        ]
      }
    } = query
  end

  test "scalar function - multiple args" do
    assert {:ok, [_query]} = SqlParser.parse("SELECT CONCAT(first_name, ' ', last_name) FROM users")
  end

  test "aggregate function - COUNT(*)" do
    assert {:ok, [query]} = SqlParser.parse("SELECT COUNT(*) FROM users")

    assert %SqlParser.Query{
      body: %SqlParser.Select{
        projection: [
          %SqlParser.UnnamedExpr{
            expr: %SqlParser.Expr{
              type: :function,
              value: %SqlParser.Function{
                name: %SqlParser.ObjectName{
                  names: [%SqlParser.Ident{value: "COUNT"}]
                },
                args: %SqlParser.FunctionArguments{
                  type: :list,
                  value: %SqlParser.FunctionArgumentList{
                    args: [
                      %SqlParser.FunctionArg{
                        type: :unnamed,
                        arg: %SqlParser.FunctionArgExpr{type: :wildcard}
                      }
                    ]
                  }
                }
              }
            }
          }
        ]
      }
    } = query
  end

  test "aggregate function - with DISTINCT" do
    assert {:ok, [query]} = SqlParser.parse("SELECT COUNT(DISTINCT user_id) FROM orders")

    assert %SqlParser.Query{
      body: %SqlParser.Select{
        projection: [
          %SqlParser.UnnamedExpr{
            expr: %SqlParser.Expr{
              type: :function,
              value: %SqlParser.Function{
                args: %SqlParser.FunctionArguments{
                  type: :list,
                  value: %SqlParser.FunctionArgumentList{
                    duplicate_treatment: :distinct
                  }
                }
              }
            }
          }
        ]
      }
    } = query
  end

  test "aggregate function - with FILTER" do
    assert {:ok, [query]} = SqlParser.parse("SELECT COUNT(*) FILTER (WHERE active = true) FROM users")

    assert %SqlParser.Query{
      body: %SqlParser.Select{
        projection: [
          %SqlParser.UnnamedExpr{
            expr: %SqlParser.Expr{
              type: :function,
              value: %SqlParser.Function{
                filter: %SqlParser.Expr{type: :binary_op}
              }
            }
          }
        ]
      }
    } = query
  end

  test "window function - simple OVER()" do
    assert {:ok, [_query]} = SqlParser.parse("SELECT ROW_NUMBER() OVER() FROM users")
  end

  test "window function - with PARTITION BY" do
    assert {:ok, [query]} = SqlParser.parse("SELECT ROW_NUMBER() OVER(PARTITION BY department) FROM users")

    assert %SqlParser.Query{
      body: %SqlParser.Select{
        projection: [
          %SqlParser.UnnamedExpr{
            expr: %SqlParser.Expr{
              type: :function,
              value: %SqlParser.Function{
                over: %SqlParser.WindowType{
                  type: :window_spec,
                  value: %SqlParser.WindowSpec{
                    partition_by: [%SqlParser.Expr{type: :identifier}]
                  }
                }
              }
            }
          }
        ]
      }
    } = query
  end

  test "window function - with ORDER BY" do
    assert {:ok, [_query]} = SqlParser.parse("SELECT ROW_NUMBER() OVER(ORDER BY salary DESC) FROM users")
  end

  test "window function - complete" do
    assert {:ok, [_query]} = SqlParser.parse(
      "SELECT RANK() OVER(PARTITION BY department ORDER BY salary DESC) FROM users"
    )
  end

  test "window function - with frame" do
    assert {:ok, [_query]} = SqlParser.parse(
      "SELECT SUM(amount) OVER(ORDER BY date ROWS BETWEEN 3 PRECEDING AND CURRENT ROW) FROM transactions"
    )
  end

  test "nested functions" do
    assert {:ok, [_query]} = SqlParser.parse("SELECT UPPER(CONCAT(first_name, last_name)) FROM users")
  end

  test "function in WHERE clause" do
    assert {:ok, [_query]} = SqlParser.parse("SELECT * FROM users WHERE LENGTH(name) > 5")
  end

  test "multiple aggregate functions" do
    assert {:ok, [_query]} = SqlParser.parse("SELECT COUNT(*), SUM(amount), AVG(price) FROM orders")
  end
end
```

### Phase 6: Verification and Testing

**Steps**:

1. **Compile the Rust code**:
   ```bash
   cd native/sqlparser_parse
   cargo build
   ```

2. **Run Elixir tests**:
   ```bash
   mix test
   ```

3. **Test specific function scenarios**:
   ```bash
   mix test --only describe:"function parsing"
   ```

4. **Manual verification in IEx**:
   ```elixir
   iex -S mix
   {:ok, result} = SqlParser.parse("SELECT COUNT(*) FROM users")
   IO.inspect(result, pretty: true, limit: :infinity)
   ```

## Success Criteria

- [ ] All existing tests continue to pass
- [ ] New function parsing tests pass (15+ test cases)
- [ ] Can parse scalar functions (UPPER, LOWER, CONCAT, etc.)
- [ ] Can parse aggregate functions (COUNT, SUM, AVG, etc.)
- [ ] Can parse window functions with PARTITION BY and ORDER BY
- [ ] Can handle DISTINCT modifier
- [ ] Can handle FILTER clause
- [ ] Can handle nested functions
- [ ] Round-trip tests work (parse → to_sql → parse)
- [ ] No compilation errors or warnings

## Troubleshooting Guide

### Common Issues

**Issue**: Compilation errors about missing imports
- **Solution**: Ensure all necessary imports are at the top of datatypes.rs
- Check that `use rustler::types::atom::Atom;` is present

**Issue**: Pattern matching errors in From implementations
- **Solution**: Verify atom equality checks use `==` not `=`
- Ensure all enum variants are handled

**Issue**: Tests fail with :nif_not_loaded
- **Solution**: Recompile the NIF:
  ```bash
  cd native/sqlparser_parse && cargo clean && cargo build
  mix clean
  mix compile
  ```

**Issue**: Struct field order mismatches
- **Solution**: Ensure Elixir struct field order matches Rust struct field order exactly

## Estimated Effort

- **Phase 1** (Elixir structs): 30 minutes
- **Phase 2** (Rust types): 1 hour
- **Phase 3** (Parsing direction): 2 hours
- **Phase 4** (Generation direction): 2 hours
- **Phase 5** (Tests): 1 hour
- **Phase 6** (Verification): 1 hour
- **Total**: ~7-8 hours

## Additional Notes for AI Implementation

1. **Type Safety**: Rust is strongly typed. Every field must have the correct type.
2. **Clone Trait**: Most structs need `#[derive(Clone)]` for the conversion implementations.
3. **Atom Comparison**: Always use `==` for atom comparisons in Rust, not pattern matching.
4. **Box Usage**: Recursive structures (like Expr containing Expr) must use `Box<>`.
5. **Option Handling**: Use `.map()` for Option types to transform values.
6. **Iterator Patterns**: Use `.iter().map().collect()` for vector transformations.
7. **Module Structure**: Keep all function-related code together for maintainability.

## References

- sqlparser-rs documentation: https://docs.rs/sqlparser/0.56.0/
- Rustler documentation: https://docs.rs/rustler/
- Source analysis: `/tmp/sqlparser-056/src/ast/mod.rs`
