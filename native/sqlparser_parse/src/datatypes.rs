use rustler::types::atom::Atom;
use rustler::{NifStruct, NifTaggedEnum, NifUntaggedEnum};
// use rustler::{map}
mod type_atoms {
    rustler::atoms! {
        value,
        unary_op,
        binary_op,
        nested,
        all_op,
        any_op,
        is_unknown,
        is_not_unknown,
        is_null,
        is_not_null,
        is_true,
        is_not_true,
        is_false,
        is_not_false,
        in_list,
        in_subquery,
        in_unnest,
        between,
        like,
        ilike,
        similar_to,
        identifier,
        compound_identifier,
        composite_access

    }
}

mod result_atoms {
    rustler::atoms! {
        not_implemented
    }
}

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
#[derive(NifStruct)]
// #[rustler(encode)]
#[module = "SqlParser.Document"]
pub struct Document {
    pub statements: Vec<Statement>,
}
// sqlparser::ast::Query
impl Document {
    pub fn new(ast: Vec<sqlparser::ast::Statement>) -> Self {
        Self {
            statements: ast
                .iter()
                .map(|s| match s {
                    sqlparser::ast::Statement::Query(query) => {
                        Statement::Query(Box::new(Query::new(*query.clone())))
                    }

                    _ => Statement::NotImplemented(result_atoms::not_implemented()),
                })
                .collect(),
        }
    }
}
impl From<Value> for sqlparser::ast::Value {
    fn from(value: Value) -> Self {
        // sqlparser::ast::Value {
        //     value: ident.value,
        //     quote_style: None,
        // }
        //Value::Number(number)
        match value {
            Value::Number(number) => sqlparser::ast::Value::Number(number.value, number.long),
            Value::Boolean(boolean) => sqlparser::ast::Value::Boolean(boolean.value),
            _ => sqlparser::ast::Value::Number("3".to_string(), false),
        }
    }
}
impl From<BinaryOperator> for sqlparser::ast::BinaryOperator {
    fn from(binary_operator: BinaryOperator) -> Self {
        match binary_operator {
            BinaryOperator::And => Self::And,
            BinaryOperator::BitwiseAnd => Self::BitwiseAnd,
            BinaryOperator::GtEq => Self::GtEq,
            BinaryOperator::Gt => Self::Gt,
            BinaryOperator::LtEq => Self::LtEq,
            BinaryOperator::Lt => Self::Lt,
            BinaryOperator::Plus => Self::Plus,
            BinaryOperator::Minus => Self::Minus,
            BinaryOperator::Multiply => Self::Multiply,
            BinaryOperator::Divide => Self::Divide,
            BinaryOperator::Modulo => Self::Modulo,
            BinaryOperator::StringConcat => Self::StringConcat,
            BinaryOperator::Eq => Self::Eq,
            _ => Self::Eq,
        }
    }
}
impl From<UnaryOperator> for sqlparser::ast::UnaryOperator {
    fn from(unary_operator: UnaryOperator) -> Self {
        match unary_operator {
            UnaryOperator::Plus => Self::Plus,
            UnaryOperator::Minus => Self::Minus,
            UnaryOperator::Not => Self::Not,
            UnaryOperator::PGBitwiseNot => Self::PGBitwiseNot,
            UnaryOperator::PGSquareRoot => Self::PGSquareRoot,
            UnaryOperator::PGCubeRoot => Self::PGCubeRoot,
            UnaryOperator::PGPostfixFactorial => Self::PGPostfixFactorial,
            UnaryOperator::PGPrefixFactorial => Self::PGPrefixFactorial,
            UnaryOperator::PGAbs => Self::PGAbs,
        }
    }
}

impl From<Expr> for sqlparser::ast::Expr {
    fn from(expr: Expr) -> Self {
        match expr.value {
            ExprEnum::Identifier(ident) => {
                sqlparser::ast::Expr::Identifier(sqlparser::ast::Ident {
                    value: ident.value,
                    quote_style: None,
                    span: sqlparser::tokenizer::Span::empty(),
                })
            }
            ExprEnum::CompoundIdentifier(idents) => sqlparser::ast::Expr::CompoundIdentifier(
                idents
                    .iter()
                    .map(|i| sqlparser::ast::Ident {
                        value: i.value.clone(),
                        quote_style: None,
                        span: sqlparser::tokenizer::Span::empty(),
                    })
                    .collect(),
            ),
            ExprEnum::Value(value) => {
                sqlparser::ast::Expr::Value(sqlparser::ast::ValueWithSpan {
                    value: sqlparser::ast::Value::from(value),
                    span: sqlparser::tokenizer::Span::empty(),
                })
            }
            ExprEnum::BinaryOp(op) => sqlparser::ast::Expr::BinaryOp {
                left: Box::new(sqlparser::ast::Expr::from(*op.left.clone())),
                op: sqlparser::ast::BinaryOperator::from(op.op),
                right: Box::new(sqlparser::ast::Expr::from(*op.right.clone())),
            },
            ExprEnum::IsFalse(expr) => {
                sqlparser::ast::Expr::IsFalse(Box::new(sqlparser::ast::Expr::from(*expr.clone())))
            }
            ExprEnum::IsNotFalse(expr) => sqlparser::ast::Expr::IsNotFalse(Box::new(
                sqlparser::ast::Expr::from(*expr.clone()),
            )),
            ExprEnum::IsTrue(expr) => {
                sqlparser::ast::Expr::IsTrue(Box::new(sqlparser::ast::Expr::from(*expr.clone())))
            }
            ExprEnum::IsNotTrue(expr) => {
                sqlparser::ast::Expr::IsNotTrue(Box::new(sqlparser::ast::Expr::from(*expr.clone())))
            }
            ExprEnum::IsNull(expr) => {
                sqlparser::ast::Expr::IsNull(Box::new(sqlparser::ast::Expr::from(*expr.clone())))
            }
            ExprEnum::IsNotNull(expr) => {
                sqlparser::ast::Expr::IsNotNull(Box::new(sqlparser::ast::Expr::from(*expr.clone())))
            }
            ExprEnum::IsUnknown(expr) => {
                sqlparser::ast::Expr::IsUnknown(Box::new(sqlparser::ast::Expr::from(*expr.clone())))
            }
            ExprEnum::IsNotUnknown(expr) => sqlparser::ast::Expr::IsNotUnknown(Box::new(
                sqlparser::ast::Expr::from(*expr.clone()),
            )),
            ExprEnum::UnaryOp(op) => sqlparser::ast::Expr::UnaryOp {
                expr: Box::new(sqlparser::ast::Expr::from(*op.expr.clone())),
                op: sqlparser::ast::UnaryOperator::from(op.op),
            },
            ExprEnum::Function(func) => {
                sqlparser::ast::Expr::Function(sqlparser::ast::Function::from(func))
            }
            ExprEnum::SimilarTo(..)
            | ExprEnum::Nested(..)
            | ExprEnum::NotImplemented(..)
            | ExprEnum::Between(..)
            | ExprEnum::Like(..)
            | ExprEnum::ILike(..)
            | ExprEnum::InUnnest(..)
            | ExprEnum::InSubquery(..)
            | ExprEnum::InList(..)
            | ExprEnum::AllOp(..)
            | ExprEnum::AnyOp(..)
            | ExprEnum::CompositeAccess(..) => {
                sqlparser::ast::Expr::Identifier(sqlparser::ast::Ident {
                    value: "abd".to_string(),
                    quote_style: None,
                    span: sqlparser::tokenizer::Span::empty(),
                })
            }
        }
    }
}
impl From<SelectItem> for sqlparser::ast::SelectItem {
    fn from(select_item: SelectItem) -> Self {
        match select_item {
            SelectItem::UnnamedExpr(expr) => {
                sqlparser::ast::SelectItem::UnnamedExpr(sqlparser::ast::Expr::from(expr))
            }
            SelectItem::Wildcard(_) => {
                sqlparser::ast::SelectItem::Wildcard(sqlparser::ast::WildcardAdditionalOptions::default())
            }
            _ => sqlparser::ast::SelectItem::UnnamedExpr(sqlparser::ast::Expr::Identifier(
                sqlparser::ast::Ident {
                    value: "abd".to_string(),
                    quote_style: None,
                    span: sqlparser::tokenizer::Span::empty(),
                },
            )),
        }
    }
}
impl From<Ident> for sqlparser::ast::Ident {
    fn from(ident: Ident) -> Self {
        sqlparser::ast::Ident {
            value: ident.value,
            quote_style: None,
            span: sqlparser::tokenizer::Span::empty(),
        }
    }
}
impl From<ObjectName> for sqlparser::ast::ObjectName {
    fn from(object_name: ObjectName) -> Self {
        let idents: Vec<sqlparser::ast::Ident> = object_name
            .names
            .iter()
            .map(|l| sqlparser::ast::Ident::from(l.clone()))
            .collect();
        sqlparser::ast::ObjectName::from(idents)
    }
}
impl From<TableFactor> for sqlparser::ast::TableFactor {
    fn from(select_item: TableFactor) -> Self {
        let name = sqlparser::ast::ObjectName([].to_vec());
        match select_item {
            TableFactor::Table(table) => sqlparser::ast::TableFactor::Table {
                name: sqlparser::ast::ObjectName::from(table.name),
                alias: None,
                args: None,
                with_hints: [].to_vec(),
                version: None,
                with_ordinality: false,
                partitions: vec![],
                json_path: None,
                sample: None,
                index_hints: vec![],
            },
            _ => sqlparser::ast::TableFactor::Table {
                name,
                alias: None,
                args: None,
                with_hints: [].to_vec(),
                version: None,
                with_ordinality: false,
                partitions: vec![],
                json_path: None,
                sample: None,
                index_hints: vec![],
            },
        }
    }
}
impl From<JoinOperator> for sqlparser::ast::JoinOperator {
    fn from(_: JoinOperator) -> Self {
        sqlparser::ast::JoinOperator::CrossJoin
    }
}
impl From<Join> for sqlparser::ast::Join {
    fn from(join: Join) -> Self {
        sqlparser::ast::Join {
            relation: sqlparser::ast::TableFactor::from(join.relation),
            join_operator: sqlparser::ast::JoinOperator::from(join.join_operator),
            global: false,
        }
    }
}
impl From<TableWithJoins> for sqlparser::ast::TableWithJoins {
    fn from(table_with_joins: TableWithJoins) -> Self {
        sqlparser::ast::TableWithJoins {
            relation: sqlparser::ast::TableFactor::from(table_with_joins.relation),
            joins: table_with_joins
                .joins
                .iter()
                .map(|j| sqlparser::ast::Join::from(j.clone()))
                .collect(),
        }
    }
}
impl From<OrderByExpr> for sqlparser::ast::OrderByExpr {
    fn from(order_by_expr: OrderByExpr) -> Self {
        sqlparser::ast::OrderByExpr {
            expr: sqlparser::ast::Expr::from(order_by_expr.expr),
            options: sqlparser::ast::OrderByOptions {
                asc: order_by_expr.asc,
                nulls_first: order_by_expr.nulls_first,
            },
            with_fill: None,
        }
    }
}
impl From<SetExpr> for sqlparser::ast::SetExpr {
    fn from(setexpr: SetExpr) -> Self {
        match setexpr {
            SetExpr::Select(select) => {
                sqlparser::ast::SetExpr::Select(Box::new(sqlparser::ast::Select {
                    distinct: if select.distinct { Some(sqlparser::ast::Distinct::Distinct) } else { None },
                    top: None,
                    projection: select
                        .projection
                        .iter()
                        .map(|l| sqlparser::ast::SelectItem::from(l.clone()))
                        .collect(),
                    into: None,
                    from: select
                        .from
                        .iter()
                        .map(|l| sqlparser::ast::TableWithJoins::from(l.clone()))
                        .collect(),
                    lateral_views: [].to_vec(),
                    selection: select.selection.map(sqlparser::ast::Expr::from),
                    group_by: sqlparser::ast::GroupByExpr::Expressions(
                        select
                            .group_by
                            .iter()
                            .map(|l| sqlparser::ast::Expr::from(l.clone()))
                            .collect(),
                        vec![],
                    ),
                    cluster_by: [].to_vec(),
                    distribute_by: [].to_vec(),
                    sort_by: select
                        .sort_by
                        .iter()
                        .map(|l| sqlparser::ast::Expr::from(l.clone()))
                        .collect(),
                    having: select.having.map(sqlparser::ast::Expr::from),
                    qualify: None,
                    connect_by: None,
                    named_window: vec![],
                    window_before_qualify: false,
                    value_table_mode: None,
                    prewhere: None,
                    flavor: sqlparser::ast::SelectFlavor::Standard,
                    select_token: sqlparser::ast::helpers::attached_token::AttachedToken::empty(),
                    top_before_distinct: false,
                }))
            }
            _ => sqlparser::ast::SetExpr::Select(Box::new(sqlparser::ast::Select {
                distinct: None,
                top: None,
                projection: [].to_vec(),
                into: None,
                from: [].to_vec(),
                lateral_views: [].to_vec(),
                selection: None,
                group_by: sqlparser::ast::GroupByExpr::Expressions(vec![], vec![]),
                cluster_by: [].to_vec(),
                distribute_by: [].to_vec(),
                sort_by: [].to_vec(),
                having: None,
                qualify: None,
                connect_by: None,
                named_window: vec![],
                window_before_qualify: false,
                value_table_mode: None,
                prewhere: None,
                flavor: sqlparser::ast::SelectFlavor::Standard,
                select_token: sqlparser::ast::helpers::attached_token::AttachedToken::empty(),
                top_before_distinct: false,
            })),
        }
    }
}
impl From<Statement> for sqlparser::ast::Statement {
    fn from(value: Statement) -> Self {
        match value {
            Statement::Query(query) => {
                sqlparser::ast::Statement::Query(Box::new(sqlparser::ast::Query {
                    body: Box::new(sqlparser::ast::SetExpr::from(query.body)),
                    limit_clause: query.limit.map(|expr| sqlparser::ast::LimitClause::LimitOffset {
                        limit: Some(sqlparser::ast::Expr::from(expr)),
                        offset: None,
                        limit_by: vec![],
                    }),
                    with: None,
                    order_by: if query.order_by.is_empty() {
                        None
                    } else {
                        Some(sqlparser::ast::OrderBy {
                            kind: sqlparser::ast::OrderByKind::Expressions(
                                query
                                    .order_by
                                    .iter()
                                    .map(|l| sqlparser::ast::OrderByExpr::from(l.clone()))
                                    .collect()
                            ),
                            interpolate: None,
                        })
                    },
                    locks: [].to_vec(),
                    fetch: None,
                    for_clause: None,
                    settings: None,
                    format_clause: None,
                }))
            }
            _ => sqlparser::ast::Statement::Query(Box::new(sqlparser::ast::Query {
                body: Box::new(sqlparser::ast::SetExpr::Select(Box::new(
                    sqlparser::ast::Select {
                        distinct: None,
                        top: None,
                        projection: [].to_vec(),
                        into: None,
                        from: [].to_vec(),
                        lateral_views: [].to_vec(),
                        selection: None,
                        group_by: sqlparser::ast::GroupByExpr::Expressions(vec![], vec![]),
                        cluster_by: [].to_vec(),
                        distribute_by: [].to_vec(),
                        sort_by: [].to_vec(),
                        having: None,
                        qualify: None,
                        connect_by: None,
                        named_window: vec![],
                        window_before_qualify: false,
                        value_table_mode: None,
                        prewhere: None,
                        flavor: sqlparser::ast::SelectFlavor::Standard,
                        select_token: sqlparser::ast::helpers::attached_token::AttachedToken::empty(),
                        top_before_distinct: false,
                    },
                ))),

                limit_clause: None,
                with: None,
                order_by: None,
                locks: [].to_vec(),
                fetch: None,
                for_clause: None,
                settings: None,
                format_clause: None,
            })),
        }
    }
}
#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.Values"]
#[derive(Clone)]
pub struct Values {
    explicit_row: bool,
    rows: Vec<Vec<Expr>>,
}
#[derive(NifTaggedEnum, Clone)]
pub enum SetOperator {
    Union,
    Except,
    Intersect,
}
#[derive(NifTaggedEnum, Clone)]
pub enum SetQuantifier {
    All,
    Distinct,
    None,
}
#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.SetOperation"]
#[derive(Clone)]
pub struct SetOperation {
    op: SetOperator,
    set_quantifier: SetQuantifier,
    left: Box<SetExpr>,
    right: Box<SetExpr>,
}

#[derive(Clone, NifUntaggedEnum)]
// #[rustler(encode)]
pub enum SetExpr {
    Select(Select),
    Query(Box<Query>),
    Values(Values),
    SetOperation(SetOperation),
    // Insert(Statement),
    NotImplemented(Atom),
}

#[derive(NifStruct)]
#[module = "SqlParser.Wildcard"]
#[derive(Clone)]
pub struct Wildcard {}
impl Wildcard {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Wildcard {
    fn default() -> Self {
        Wildcard::new()
    }
}

#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.ExprWithAlias"]
#[derive(Clone)]
pub struct ExprWithAlias {
    expr: Expr,
    alias: Ident,
}
#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.JoinConstraint"]
#[derive(Clone)]
pub struct JoinConstraint {
    constraint: JoinConstraintEnum,
    kind: Atom,
}

mod join_constraints_atoms {
    rustler::atoms! {
        on,
        using,
        natural,
        none
    }
}

#[derive(NifUntaggedEnum, Clone)]
//#[rustler(encode)]
pub enum JoinConstraintEnum {
    On(Expr),
    Using(Vec<Ident>),
    Natural(Atom),
    None(Atom),
}
impl From<sqlparser::ast::JoinConstraint> for JoinConstraint {
    fn from(join_constraint: sqlparser::ast::JoinConstraint) -> Self {
        match join_constraint {
            sqlparser::ast::JoinConstraint::On(expr) => JoinConstraint {
                constraint: JoinConstraintEnum::On(Expr::new(expr)),
                kind: join_constraints_atoms::on(),
            },
            sqlparser::ast::JoinConstraint::Using(ident) => JoinConstraint {
                constraint: JoinConstraintEnum::Using(
                    ident.iter().flat_map(|name| {
                        name.0.iter().filter_map(|part| part.as_ident()).map(|i| Ident::from(i.clone())).collect::<Vec<_>>()
                    }).collect(),
                ),
                kind: join_constraints_atoms::using(),
            },
            sqlparser::ast::JoinConstraint::Natural => JoinConstraint {
                constraint: JoinConstraintEnum::Natural(join_constraints_atoms::natural()),
                kind: join_constraints_atoms::natural(),
            },
            sqlparser::ast::JoinConstraint::None => JoinConstraint {
                constraint: JoinConstraintEnum::None(join_constraints_atoms::none()),
                kind: join_constraints_atoms::none(),
            },
        }
    }
}

#[derive(NifStruct, Clone)]
//#[rustler(encode)]
#[module = "SqlParser.JoinOperator"]
pub struct JoinOperator {
    operator: JoinOperatorEnum,
    kind: Atom,
}

mod join_operator_atoms {
    rustler::atoms! {
        inner,
        left_outer,
        right_outer,
        full_outer,
        cross_join,
        left_semi,
        right_semi,
        left_anti,
        right_anti,
        cross_apply,
        outer_apply
    }
}
#[derive(NifUntaggedEnum, Clone)]
//#[rustler(encode)]
pub enum JoinOperatorEnum {
    Inner(JoinConstraint),
    LeftOuter(JoinConstraint),
    RightOuter(JoinConstraint),
    FullOuter(JoinConstraint),
    CrossJoin(JoinConstraint),
    LeftSemi(JoinConstraint),
    RightSemi(JoinConstraint),
    LeftAnti(JoinConstraint),
    RightAnti(JoinConstraint),
    // CrossApply,
    // OuterApply,
}

impl From<sqlparser::ast::JoinOperator> for JoinOperator {
    fn from(join_operator: sqlparser::ast::JoinOperator) -> Self {
        match join_operator {
            sqlparser::ast::JoinOperator::Inner(constraint) => JoinOperator {
                kind: join_operator_atoms::inner(),
                operator: JoinOperatorEnum::Inner(JoinConstraint::from(constraint)),
            },
            sqlparser::ast::JoinOperator::LeftOuter(constraint) => JoinOperator {
                kind: join_operator_atoms::left_outer(),
                operator: JoinOperatorEnum::LeftOuter(JoinConstraint::from(constraint)),
            },
            sqlparser::ast::JoinOperator::RightOuter(constraint) => JoinOperator {
                kind: join_operator_atoms::right_outer(),
                operator: JoinOperatorEnum::RightOuter(JoinConstraint::from(constraint)),
            },
            sqlparser::ast::JoinOperator::FullOuter(constraint) => JoinOperator {
                kind: join_operator_atoms::full_outer(),
                operator: JoinOperatorEnum::FullOuter(JoinConstraint::from(constraint)),
            },
            sqlparser::ast::JoinOperator::CrossJoin => JoinOperator {
                kind: join_operator_atoms::cross_join(),
                operator: JoinOperatorEnum::CrossJoin(JoinConstraint {
                    constraint: JoinConstraintEnum::None(join_constraints_atoms::none()),
                    kind: join_constraints_atoms::none(),
                }),
            },
            sqlparser::ast::JoinOperator::LeftSemi(constraint) => JoinOperator {
                kind: join_operator_atoms::left_semi(),
                operator: JoinOperatorEnum::LeftSemi(JoinConstraint::from(constraint)),
            },
            sqlparser::ast::JoinOperator::RightSemi(constraint) => JoinOperator {
                kind: join_operator_atoms::right_semi(),
                operator: JoinOperatorEnum::RightSemi(JoinConstraint::from(constraint)),
            },
            sqlparser::ast::JoinOperator::LeftAnti(constraint) => JoinOperator {
                kind: join_operator_atoms::left_anti(),
                operator: JoinOperatorEnum::LeftAnti(JoinConstraint::from(constraint)),
            },
            sqlparser::ast::JoinOperator::RightAnti(constraint) => JoinOperator {
                kind: join_operator_atoms::right_anti(),
                operator: JoinOperatorEnum::RightAnti(JoinConstraint::from(constraint)),
            },
            sqlparser::ast::JoinOperator::Join(constraint) => JoinOperator {
                kind: join_operator_atoms::inner(),
                operator: JoinOperatorEnum::Inner(JoinConstraint::from(constraint)),
            },
            _ => panic!("Unsupported join operator: {:?}", join_operator),
            // sqlparser::ast::JoinOperator::CrossApply => JoinOperator{ operator: JoinOperatorEnum::CrossApply, },
            // sqlparser::ast::JoinOperator::OuterApply => JoinOperator{ operator: JoinOperatorEnum::OuterApply },
        }
    }
}

#[derive(NifUntaggedEnum, Clone)]
//#[rustler(encode)]
pub enum SelectItem {
    Wildcard(Wildcard),
    UnnamedExpr(Expr),
    NotImplemented(Atom),
    ExprWithAlias(ExprWithAlias), // QualifiedWildcard(ObjectName, WildcardAdditionalOptions),
}
#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.Join"]
#[derive(Clone)]
pub struct Join {
    pub relation: TableFactor,
    pub join_operator: JoinOperator,
}

#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.TableWithJoins"]
#[derive(Clone)]
pub struct TableWithJoins {
    pub relation: TableFactor,
    pub joins: Vec<Join>,
}
impl TableWithJoins {
    pub fn new(ast: &sqlparser::ast::TableWithJoins) -> Self {
        let relation = TableFactor::from(ast.relation.clone());
        Self {
            relation,
            joins: ast
                .joins
                .iter()
                .map(|j| Join {
                    join_operator: JoinOperator::from(j.join_operator.clone()),
                    relation: TableFactor::from(j.relation.clone()),
                })
                .collect(),
        }
    }
}
#[derive(NifStruct)]
#[module = "SqlParser.Ident"]
#[derive(Clone)]
pub struct Ident {
    pub value: String,
    pub quote_style: Option<String>,
}

impl From<sqlparser::ast::Ident> for Ident {
    fn from(ident: sqlparser::ast::Ident) -> Self {
        Self {
            value: ident.to_string(),
            quote_style: ident.quote_style.map(|s| s.to_string()),
        }
    }
}

#[derive(NifStruct)]
#[module = "SqlParser.ObjectName"]
#[derive(Clone)]
pub struct ObjectName {
    names: Vec<Ident>,
}

#[derive(NifUntaggedEnum, Clone)]
pub enum TableFactor {
    Table(Table),
    NotImplemented(Atom),
}

impl From<sqlparser::ast::TableFactor> for TableFactor {
    fn from(table_factor: sqlparser::ast::TableFactor) -> Self {
        match table_factor {
            sqlparser::ast::TableFactor::Table { name, .. } => TableFactor::Table(Table {
                name: ObjectName {
                    names: name.0.iter().filter_map(|p| p.as_ident()).map(|i| Ident::from(i.clone())).collect(),
                },
            }),
            sqlparser::ast::TableFactor::NestedJoin { .. } => {
                TableFactor::NotImplemented(result_atoms::not_implemented())
            }
            sqlparser::ast::TableFactor::Derived { .. } => {
                TableFactor::NotImplemented(result_atoms::not_implemented())
            }
            sqlparser::ast::TableFactor::TableFunction { .. } => {
                TableFactor::NotImplemented(result_atoms::not_implemented())
            }
            sqlparser::ast::TableFactor::UNNEST { .. }
            | sqlparser::ast::TableFactor::Function { .. }
            | sqlparser::ast::TableFactor::JsonTable { .. }
            | sqlparser::ast::TableFactor::OpenJsonTable { .. }
            | sqlparser::ast::TableFactor::Unpivot { .. }
            | sqlparser::ast::TableFactor::Pivot { .. }
            | sqlparser::ast::TableFactor::MatchRecognize { .. }
            | sqlparser::ast::TableFactor::XmlTable { .. } => {
                TableFactor::NotImplemented(result_atoms::not_implemented())
            }
        }
    }
}

#[derive(NifStruct)]
#[module = "SqlParser.Table"]
#[derive(Clone)]
pub struct Table {
    name: ObjectName,
}

#[derive(NifTaggedEnum, Clone)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    StringConcat,
    Gt,
    Lt,
    GtEq,
    LtEq,
    Spaceship,
    Eq,
    NotEq,
    And,
    Or,
    Xor,
    BitwiseOr,
    BitwiseAnd,
    BitwiseXor,
    PGBitwiseXor,
    PGBitwiseShiftLeft,
    PGBitwiseShiftRight,
    PGRegexMatch,
    PGRegexIMatch,
    PGRegexNotMatch,
    PGRegexNotIMatch,
    NotImplemented,
}

impl From<sqlparser::ast::BinaryOperator> for BinaryOperator {
    fn from(op: sqlparser::ast::BinaryOperator) -> Self {
        match op {
            sqlparser::ast::BinaryOperator::Plus => Self::Plus,
            sqlparser::ast::BinaryOperator::Minus => Self::Minus,
            sqlparser::ast::BinaryOperator::Multiply => Self::Multiply,
            sqlparser::ast::BinaryOperator::Divide => Self::Divide,
            sqlparser::ast::BinaryOperator::Modulo => Self::Modulo,
            sqlparser::ast::BinaryOperator::StringConcat => Self::StringConcat,
            sqlparser::ast::BinaryOperator::Gt => Self::Gt,
            sqlparser::ast::BinaryOperator::Lt => Self::Lt,
            sqlparser::ast::BinaryOperator::GtEq => Self::GtEq,
            sqlparser::ast::BinaryOperator::LtEq => Self::LtEq,
            sqlparser::ast::BinaryOperator::Spaceship => Self::Spaceship,
            sqlparser::ast::BinaryOperator::Eq => Self::Eq,
            sqlparser::ast::BinaryOperator::NotEq => Self::NotEq,
            sqlparser::ast::BinaryOperator::And => Self::And,
            sqlparser::ast::BinaryOperator::Or => Self::Or,
            sqlparser::ast::BinaryOperator::Xor => Self::Xor,
            sqlparser::ast::BinaryOperator::BitwiseOr => Self::BitwiseOr,
            sqlparser::ast::BinaryOperator::BitwiseAnd => Self::BitwiseAnd,
            sqlparser::ast::BinaryOperator::BitwiseXor => Self::BitwiseXor,
            sqlparser::ast::BinaryOperator::PGBitwiseXor => Self::PGBitwiseXor,
            sqlparser::ast::BinaryOperator::PGBitwiseShiftLeft => Self::PGBitwiseShiftLeft,
            sqlparser::ast::BinaryOperator::PGBitwiseShiftRight => Self::PGBitwiseShiftRight,
            sqlparser::ast::BinaryOperator::PGRegexMatch => Self::PGRegexMatch,
            sqlparser::ast::BinaryOperator::PGRegexIMatch => Self::PGRegexIMatch,
            sqlparser::ast::BinaryOperator::PGRegexNotMatch => Self::PGRegexNotMatch,
            sqlparser::ast::BinaryOperator::PGRegexNotIMatch => Self::PGRegexNotIMatch,
            _ => Self::NotImplemented,
        }
    }
}

#[derive(NifTaggedEnum, Clone)]
pub enum UnaryOperator {
    Plus,
    Minus,
    Not,
    PGBitwiseNot,       //=> "~",
    PGSquareRoot,       //=> "|/",
    PGCubeRoot,         //=> "||/",
    PGPostfixFactorial, //=> "!",
    PGPrefixFactorial,  //=> "!!",
    PGAbs,              //=> "@",
}
impl From<sqlparser::ast::UnaryOperator> for UnaryOperator {
    fn from(op: sqlparser::ast::UnaryOperator) -> Self {
        match op {
            sqlparser::ast::UnaryOperator::Plus => Self::Plus,
            sqlparser::ast::UnaryOperator::Minus => Self::Minus,
            sqlparser::ast::UnaryOperator::Not => Self::Not,
            sqlparser::ast::UnaryOperator::PGBitwiseNot => Self::PGBitwiseNot,
            sqlparser::ast::UnaryOperator::PGSquareRoot => Self::PGSquareRoot,
            sqlparser::ast::UnaryOperator::PGCubeRoot => Self::PGCubeRoot,
            sqlparser::ast::UnaryOperator::PGPostfixFactorial => Self::PGPostfixFactorial,
            sqlparser::ast::UnaryOperator::PGPrefixFactorial => Self::PGPrefixFactorial,
            sqlparser::ast::UnaryOperator::PGAbs => Self::PGAbs,
            _ => Self::Not, // Default to Not for unhandled cases
        }
    }
}

#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.UnaryOp"]
#[derive(Clone)]
pub struct UnaryOp {
    op: UnaryOperator,
    expr: Box<Expr>,
}
#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.CompositeAccess"]
#[derive(Clone)]
pub struct CompositeAccess {
    expr: Box<Expr>,
    key: Ident,
}

#[derive(NifStruct)]
//#[rustler(encode)]
#[derive(Clone)]
#[module = "SqlParser.BinaryOp"]
pub struct BinaryOp {
    left: Box<Expr>,
    op: BinaryOperator,
    right: Box<Expr>,
}
#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.Number"]
#[derive(Clone)]
pub struct Number {
    value: String,
    long: bool,
}
#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.Boolean"]
#[derive(Clone)]
pub struct Boolean {
    value: bool,
}

#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.Null"]
#[derive(Clone)]
pub struct Null {}

#[derive(NifUntaggedEnum)]
//#[rustler(encode)]
#[derive(Clone)]
pub enum Value {
    Number(Number),
    SingleQuotedString(String),
    // DollarQuotedString(DollarQuotedString),
    EscapedStringLiteral(String),
    NationalStringLiteral(String),
    HexStringLiteral(String),
    DoubleQuotedString(String),
    Boolean(Boolean),
    Null(Null),
    Placeholder(String),
    UnQuotedString(String),
    NotImplemented(Atom),
}
impl From<sqlparser::ast::Value> for Value {
    fn from(value: sqlparser::ast::Value) -> Self {
        match value {
            sqlparser::ast::Value::Number(num, long) => Self::Number(Number { value: num, long }),
            sqlparser::ast::Value::SingleQuotedString(string) => Self::SingleQuotedString(string),
            sqlparser::ast::Value::DollarQuotedString(_dollar_quoted_string) => {
                Self::NotImplemented(result_atoms::not_implemented())
            }
            sqlparser::ast::Value::EscapedStringLiteral(string) => {
                Self::EscapedStringLiteral(string)
            }
            sqlparser::ast::Value::NationalStringLiteral(string) => {
                Self::NationalStringLiteral(string)
            }
            sqlparser::ast::Value::HexStringLiteral(string) => Self::HexStringLiteral(string),
            sqlparser::ast::Value::DoubleQuotedString(string) => Self::DoubleQuotedString(string),
            sqlparser::ast::Value::Boolean(boolean) => Self::Boolean(Boolean { value: boolean }),
            sqlparser::ast::Value::Null => Self::Null(Null {}),
            sqlparser::ast::Value::Placeholder(placeholder) => Self::Placeholder(placeholder),
            _ => Self::NotImplemented(result_atoms::not_implemented()),
        }
    }
}

#[derive(NifStruct)]
//#[rustler(encode)]
#[derive(Clone)]
#[module = "SqlParser.InList"]
pub struct InList {
    expr: Box<Expr>,
    list: Vec<Expr>,
    negated: bool,
}
#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.InSubquery"]
#[derive(Clone)]
pub struct InSubquery {
    expr: Box<Expr>,
    subquery: Box<Query>,
    negated: bool,
}
#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.InUnnest"]
#[derive(Clone)]
pub struct InUnnest {
    expr: Box<Expr>,
    array_expr: Box<Expr>,
    negated: bool,
}

#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.Between"]
#[derive(Clone)]
pub struct Between {
    expr: Box<Expr>,
    negated: bool,
    low: Box<Expr>,
    high: Box<Expr>,
}
#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.SimilarTo"]
#[derive(Clone)]
pub struct SimilarTo {
    negated: bool,
    expr: Box<Expr>,
    pattern: Box<Expr>,
    escape_char: Option<String>,
}
#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.Like"]
#[derive(Clone)]
pub struct Like {
    negated: bool,
    expr: Box<Expr>,
    pattern: Box<Expr>,
    escape_char: Option<String>,
}
#[derive(NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.ILike"]
#[derive(Clone)]
pub struct ILike {
    negated: bool,
    expr: Box<Expr>,
    pattern: Box<Expr>,
    escape_char: Option<String>,
}
#[derive(NifUntaggedEnum)]
//#[rustler(encode)]
#[derive(Clone)]
pub enum ExprEnum {
    Identifier(Ident),
    CompoundIdentifier(Vec<Ident>),
    // jsonaccess
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
    // IsDistinctFrom(Box<Expr>, Box<Expr>),
    // IsNotDistinctFrom(Box<Expr>, Box<Expr>),
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
    Function(Function),
    NotImplemented(Atom),
}
#[derive(NifStruct, Clone)]
//#[rustler(encode)]
#[module = "SqlParser.Expr"]
pub struct Expr {
    r#type: Atom,
    value: ExprEnum,
}

impl Expr {
    pub fn new(ast: sqlparser::ast::Expr) -> Self {
        match ast {
            sqlparser::ast::Expr::Identifier(ident) => Expr {
                r#type: type_atoms::identifier(),
                value: ExprEnum::Identifier(Ident::from(ident)),
            },
            sqlparser::ast::Expr::CompoundIdentifier(idents) => Expr {
                r#type: type_atoms::compound_identifier(),
                value: ExprEnum::CompoundIdentifier(
                    idents.iter().map(|p| Ident::from(p.clone())).collect(),
                ),
            },
            sqlparser::ast::Expr::IsFalse(expr) => Expr {
                r#type: type_atoms::is_false(),
                value: ExprEnum::IsFalse(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::IsNotFalse(expr) => Expr {
                r#type: type_atoms::is_not_false(),
                value: ExprEnum::IsNotFalse(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::IsTrue(expr) => Expr {
                r#type: type_atoms::is_true(),
                value: ExprEnum::IsTrue(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::IsNotTrue(expr) => Expr {
                r#type: type_atoms::is_not_true(),
                value: ExprEnum::IsNotTrue(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::IsNull(expr) => Expr {
                r#type: type_atoms::is_null(),
                value: ExprEnum::IsNull(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::IsNotNull(expr) => Expr {
                r#type: type_atoms::is_not_null(),
                value: ExprEnum::IsNotNull(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::IsUnknown(expr) => Expr {
                r#type: type_atoms::is_unknown(),
                value: ExprEnum::IsUnknown(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::IsNotUnknown(expr) => Expr {
                r#type: type_atoms::is_not_unknown(),
                value: ExprEnum::IsNotUnknown(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::InList {
                expr,
                list,
                negated,
            } => Expr {
                r#type: type_atoms::in_list(),
                value: ExprEnum::InList(InList {
                    expr: Box::new(Expr::new(*expr)),
                    list: list.iter().map(|p| Expr::new(p.clone())).collect(),
                    negated,
                }),
            },
            sqlparser::ast::Expr::InSubquery {
                expr,
                subquery,
                negated,
            } => Expr {
                r#type: type_atoms::in_subquery(),
                value: ExprEnum::InSubquery(InSubquery {
                    expr: Box::new(Expr::new(*expr)),
                    subquery: Box::new(Query::new(sqlparser::ast::Query {
                        body: subquery,
                        with: None,
                        order_by: None,
                        limit_clause: None,
                        fetch: None,
                        locks: vec![],
                        for_clause: None,
                        settings: None,
                        format_clause: None,
                    })),
                    negated,
                }),
            },
            sqlparser::ast::Expr::InUnnest {
                expr,
                array_expr,
                negated,
            } => Expr {
                r#type: type_atoms::in_subquery(),
                value: ExprEnum::InUnnest(InUnnest {
                    expr: Box::new(Expr::new(*expr)),
                    array_expr: Box::new(Expr::new(*array_expr)),
                    negated,
                }),
            },
            sqlparser::ast::Expr::Between {
                expr,
                negated,
                low,
                high,
            } => Expr {
                r#type: type_atoms::in_subquery(),
                value: ExprEnum::Between(Between {
                    expr: Box::new(Expr::new(*expr)),
                    negated,
                    low: Box::new(Expr::new(*low)),
                    high: Box::new(Expr::new(*high)),
                }),
            },
            sqlparser::ast::Expr::BinaryOp { left, op, right } => Expr {
                r#type: type_atoms::binary_op(),
                value: ExprEnum::BinaryOp(BinaryOp {
                    left: Box::new(Expr::new(*left)),
                    op: op.into(),
                    right: Box::new(Expr::new(*right)),
                }),
            },
            sqlparser::ast::Expr::Like {
                negated,
                expr,
                pattern,
                escape_char,
                any: _,
            } => Expr {
                r#type: type_atoms::like(),
                value: ExprEnum::Like(Like {
                    expr: Box::new(Expr::new(*expr)),
                    negated,
                    pattern: Box::new(Expr::new(*pattern)),
                    escape_char: escape_char.map(|c| c.to_string()),
                }),
            },
            sqlparser::ast::Expr::ILike {
                negated,
                expr,
                pattern,
                escape_char,
                any: _,
            } => Expr {
                r#type: type_atoms::ilike(),
                value: ExprEnum::ILike(ILike {
                    expr: Box::new(Expr::new(*expr)),
                    negated,
                    pattern: Box::new(Expr::new(*pattern)),
                    escape_char: escape_char.map(|c| c.to_string()),
                }),
            },
            sqlparser::ast::Expr::SimilarTo {
                negated,
                expr,
                pattern,
                escape_char,
            } => Expr {
                r#type: type_atoms::similar_to(),
                value: ExprEnum::SimilarTo(SimilarTo {
                    expr: Box::new(Expr::new(*expr)),
                    negated,
                    pattern: Box::new(Expr::new(*pattern)),
                    escape_char: escape_char.map(|c| c.to_string()),
                }),
            },
            sqlparser::ast::Expr::AnyOp { .. } => Expr {
                r#type: result_atoms::not_implemented(),
                value: ExprEnum::NotImplemented(result_atoms::not_implemented()),
            },
            sqlparser::ast::Expr::AllOp { .. } => Expr {
                r#type: result_atoms::not_implemented(),
                value: ExprEnum::NotImplemented(result_atoms::not_implemented()),
            },
            sqlparser::ast::Expr::Nested(expr) => Expr {
                r#type: type_atoms::nested(),
                value: ExprEnum::Nested(Box::new(Expr::new(*expr))),
            },
            sqlparser::ast::Expr::UnaryOp { op, expr } => Expr {
                r#type: type_atoms::unary_op(),
                value: ExprEnum::UnaryOp(UnaryOp {
                    op: op.into(),
                    expr: Box::new(Expr::new(*expr)),
                }),
            },
            sqlparser::ast::Expr::Value(value) => Expr {
                r#type: type_atoms::value(),
                value: ExprEnum::Value(Value::from(value.value)),
            },
            sqlparser::ast::Expr::Function(func) => Expr {
                r#type: function_atoms::function(),
                value: ExprEnum::Function(Function::from(func)),
            },
            sqlparser::ast::Expr::Cast { .. }
            | sqlparser::ast::Expr::JsonAccess { .. }
            | sqlparser::ast::Expr::IsDistinctFrom(_, _)
            | sqlparser::ast::Expr::AtTimeZone { .. }
            | sqlparser::ast::Expr::Extract { .. }
            | sqlparser::ast::Expr::Ceil { .. }
            | sqlparser::ast::Expr::Floor { .. }
            | sqlparser::ast::Expr::Position { .. }
            | sqlparser::ast::Expr::Substring { .. }
            | sqlparser::ast::Expr::Trim { .. }
            | sqlparser::ast::Expr::Overlay { .. }
            | sqlparser::ast::Expr::Collate { .. }
            | sqlparser::ast::Expr::TypedString { .. }
            | sqlparser::ast::Expr::Case { .. }
            | sqlparser::ast::Expr::Exists { .. }
            | sqlparser::ast::Expr::Subquery { .. }
            | sqlparser::ast::Expr::GroupingSets(_)
            | sqlparser::ast::Expr::Cube(_)
            | sqlparser::ast::Expr::Rollup(_)
            | sqlparser::ast::Expr::Tuple(_)
            | sqlparser::ast::Expr::Array(_)
            | sqlparser::ast::Expr::Interval { .. }
            | sqlparser::ast::Expr::MatchAgainst { .. }
            | sqlparser::ast::Expr::IsNotDistinctFrom(_, _) => Expr {
                r#type: result_atoms::not_implemented(),
                value: ExprEnum::NotImplemented(result_atoms::not_implemented()),
            },
            _ => Expr {
                r#type: result_atoms::not_implemented(),
                value: ExprEnum::NotImplemented(result_atoms::not_implemented()),
            },
        }
    }
}
#[derive(NifStruct)]
// #[rustler(encode)]
#[module = "SqlParser.Select"]
#[derive(Clone)]
pub struct Select {
    pub distinct: bool,
    // pub top: Option<Top>,
    pub projection: Vec<SelectItem>,
    // pub into: Option<SelectInto>,
    pub from: Vec<TableWithJoins>,
    // pub lateral_views: Vec<LateralView>,
    pub selection: Option<Expr>,
    pub group_by: Vec<Expr>,
    // pub cluster_by: Vec<Expr>,
    // pub distribute_by: Vec<Expr>,
    pub sort_by: Vec<Expr>,
    pub having: Option<Expr>,
    // pub qualify: Option<Expr>,
}
impl Select {
    pub fn new(ast: sqlparser::ast::Select) -> Self {
        Self {
            distinct: ast.distinct.is_some(),
            projection: ast
                .projection
                .iter()
                .map(|p| match p {
                    sqlparser::ast::SelectItem::Wildcard(_wildcard) => {
                        SelectItem::Wildcard(Wildcard::new())
                    }
                    sqlparser::ast::SelectItem::UnnamedExpr(expr) => {
                        SelectItem::UnnamedExpr(Expr::new(expr.clone()))
                    }
                    sqlparser::ast::SelectItem::ExprWithAlias { expr, alias } => {
                        SelectItem::ExprWithAlias(ExprWithAlias {
                            expr: Expr::new(expr.clone()),
                            alias: Ident::from(alias.clone()),
                        })
                    }
                    sqlparser::ast::SelectItem::QualifiedWildcard(_, _) => {
                        SelectItem::NotImplemented(result_atoms::not_implemented())
                    }
                })
                .collect(),
            from: ast.from.iter().map(TableWithJoins::new).collect(),
            selection: ast.selection.map(Expr::new),
            group_by: match &ast.group_by {
                sqlparser::ast::GroupByExpr::Expressions(exprs, _) => {
                    exprs.iter().map(|expr| Expr::new(expr.clone())).collect()
                }
                _ => vec![],
            },
            sort_by: ast
                .sort_by
                .iter()
                .map(|expr| Expr::new(expr.clone()))
                .collect(),
            having: ast.having.map(Expr::new),
        }
    }
}

#[derive(Clone, NifStruct)]
//#[rustler(encode)]
#[module = "SqlParser.Offset"]
pub struct Offset {
    pub value: Expr,
    pub rows: OffsetRows,
}

#[derive(NifStruct)]
//#[rustler(encode)]
#[derive(Clone)]
#[module = "SqlParser.OrderByExpr"]
pub struct OrderByExpr {
    pub expr: Expr,
    pub asc: Option<bool>,
    pub nulls_first: Option<bool>,
}

#[derive(Clone, NifStruct)]
// #[rustler(encode)]
#[module = "SqlParser.Query"]
pub struct Query {
    // pub with: Option<With>,
    pub body: SetExpr,
    pub order_by: Vec<OrderByExpr>,
    pub limit: Option<Expr>,
    pub offset: Option<Offset>,
    // pub fetch: Option<Fetch>,
    // pub lock: Option<LockType>,
}
#[derive(NifTaggedEnum, Clone)]
pub enum OffsetRows {
    None,
    Row,
    Rows,
}

impl From<sqlparser::ast::SetExpr> for SetExpr {
    fn from(set_expr: sqlparser::ast::SetExpr) -> Self {
        match set_expr {
            sqlparser::ast::SetExpr::Select(select) => SetExpr::Select(Select::new(*select)),
            sqlparser::ast::SetExpr::Query(query) => SetExpr::Query(Box::new(Query::new(*query))),
            sqlparser::ast::SetExpr::Values(values) => SetExpr::Values(Values {
                rows: values
                    .rows
                    .iter()
                    .map(|row| row.iter().map(|expr| Expr::new(expr.clone())).collect())
                    .collect(),
                explicit_row: values.explicit_row,
            }),
            sqlparser::ast::SetExpr::SetOperation {
                op,
                set_quantifier,
                left,
                right,
            } => SetExpr::SetOperation(SetOperation {
                op: match op {
                    sqlparser::ast::SetOperator::Union => SetOperator::Union,
                    sqlparser::ast::SetOperator::Except => SetOperator::Except,
                    sqlparser::ast::SetOperator::Intersect => SetOperator::Intersect,
                    sqlparser::ast::SetOperator::Minus => SetOperator::Except, // Use Except as fallback
                },
                set_quantifier: match set_quantifier {
                    sqlparser::ast::SetQuantifier::All => SetQuantifier::All,
                    sqlparser::ast::SetQuantifier::Distinct => SetQuantifier::Distinct,
                    sqlparser::ast::SetQuantifier::None => SetQuantifier::None,
                    _ => SetQuantifier::None, // Default to None for new variants
                },
                left: Box::new((*left).into()),
                right: Box::new((*right).into()),
            }),
            sqlparser::ast::SetExpr::Insert(_) 
            | sqlparser::ast::SetExpr::Table(_)
            | sqlparser::ast::SetExpr::Update(_)
            | sqlparser::ast::SetExpr::Delete(_) => {
                SetExpr::NotImplemented(result_atoms::not_implemented())
            }
        }
    }
}

impl Query {
    pub fn new(ast: sqlparser::ast::Query) -> Self {
        Self {
            body: (*ast.body).into(),
            order_by: match &ast.order_by {
                Some(order_by) => match &order_by.kind {
                    sqlparser::ast::OrderByKind::Expressions(exprs) => {
                        exprs.iter().map(|order_by_expr| OrderByExpr {
                            expr: Expr::new(order_by_expr.expr.clone()),
                            asc: order_by_expr.options.asc,
                            nulls_first: order_by_expr.options.nulls_first,
                        })
                        .collect()
                    }
                    _ => vec![],
                },
                None => vec![],
            },
            limit: match &ast.limit_clause {
                Some(sqlparser::ast::LimitClause::LimitOffset { limit, .. }) => limit.clone().map(Expr::new),
                _ => None,
            },
            offset: match &ast.limit_clause {
                Some(sqlparser::ast::LimitClause::LimitOffset { offset: Some(offset), .. }) => Some(Offset {
                    value: Expr::new(offset.value.clone()),
                    rows: match offset.rows {
                        sqlparser::ast::OffsetRows::None => OffsetRows::None,
                        sqlparser::ast::OffsetRows::Row => OffsetRows::Row,
                        sqlparser::ast::OffsetRows::Rows => OffsetRows::Rows,
                    },
                }),
                _ => None,
            },
        }
    }
}

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

// =============================================================================
// Function From Implementations (sqlparser → Elixir)
// =============================================================================

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

// =============================================================================
// Function From Implementations (Elixir → sqlparser)
// =============================================================================

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
            within_group: vec![],
        }
    }
}

impl From<FunctionArguments> for sqlparser::ast::FunctionArguments {
    fn from(args: FunctionArguments) -> Self {
        match args.value {
            FunctionArgumentsValue::None(_) => sqlparser::ast::FunctionArguments::None,
            FunctionArgumentsValue::Subquery(query) => {
                // Convert Query to sqlparser::ast::Query manually
                let q = *query;
                sqlparser::ast::FunctionArguments::Subquery(Box::new(sqlparser::ast::Query {
                    body: Box::new(sqlparser::ast::SetExpr::from(q.body)),
                    limit_clause: q.limit.map(|expr| sqlparser::ast::LimitClause::LimitOffset {
                        limit: Some(sqlparser::ast::Expr::from(expr)),
                        offset: None,
                        limit_by: vec![],
                    }),
                    with: None,
                    order_by: if q.order_by.is_empty() {
                        None
                    } else {
                        Some(sqlparser::ast::OrderBy {
                            kind: sqlparser::ast::OrderByKind::Expressions(
                                q.order_by.iter().map(|l| sqlparser::ast::OrderByExpr::from(l.clone())).collect()
                            ),
                            interpolate: None,
                        })
                    },
                    locks: vec![],
                    fetch: None,
                    for_clause: None,
                    settings: None,
                    format_clause: None,
                }))
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

#[derive(Clone, NifUntaggedEnum)]
// #[rustler(encode)]
pub enum Statement {
    Query(Box<Query>),
    NotImplemented(Atom),
}
