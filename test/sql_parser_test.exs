defmodule SqlParserTest do
  use ExUnit.Case
  doctest SqlParser

  # test "to_sql" do
  #   # WHERE b.a = d
  #   sql = "SELECT * FROM a JOIN b on a.id = b.a_id WHERE b.a = d"
  #   {:ok, statements} = SqlParser.parse(sql) |> IO.inspect
  #   assert {:ok, sql} == SqlParser.to_sql(%SqlParser.Document{statements: statements} , dialect: :postgres)
  # end
  test "recursion limit" do
    assert {:error, "sql parser error: recursion limit exceeded"} =
             SqlParser.parse("SELECT * FROM a WHERE b.a = c", recursion_limit: 1)
  end

  test "simple query" do
    assert {:ok, [query]} = SqlParser.parse("SELECT * FROM a WHERE b.a = c")

    assert %SqlParser.Query{
             body: %SqlParser.Select{
               projection: [%SqlParser.Wildcard{}],
               selection: _,
               from: [
                 %SqlParser.TableWithJoins{
                   relation: %SqlParser.Table{
                     name: %SqlParser.ObjectName{
                       names: [%SqlParser.Ident{quote_style: nil, value: "a"}]
                     }
                   }
                 }
               ]
             }
           } = query
  end

  test "update query" do
    assert {:ok, [:not_implemented]} ==
             SqlParser.parse("UPDATE foo SET bar = 1")
  end

  test "group query" do
    assert {:ok, [_query]} = SqlParser.parse("SELECT * FROM a group by e")
  end

  test "order by query" do
    assert {:ok,
            [
              %SqlParser.Query{
                body: %SqlParser.Select{
                  distinct: false,
                  from: [
                    %SqlParser.TableWithJoins{
                      joins: [],
                      relation: %SqlParser.Table{
                        name: %SqlParser.ObjectName{
                          names: [%SqlParser.Ident{quote_style: nil, value: "a"}]
                        }
                      }
                    }
                  ],
                  group_by: [],
                  having: nil,
                  projection: [%SqlParser.Wildcard{}],
                  selection: nil,
                  sort_by: []
                },
                order_by: [
                  %SqlParser.OrderByExpr{
                    expr: %SqlParser.Expr{
                      type: :identifier,
                      value: %SqlParser.Ident{quote_style: nil, value: "f"}
                    },
                    asc: nil,
                    nulls_first: nil
                  }
                ],
                limit: nil,
                offset: nil
              }
            ]} ==
             SqlParser.parse("SELECT * FROM a ORDER BY f")
  end

  @ops %{
    plus: "+",
    minus: "-",
    multiply: "*",
    divide: "/",
    modulo: "%",
    string_concat: "||",
    gt: ">",
    lt: "<",
    gt_eq: ">=",
    lt_eq: "<=",
    spaceship: "<=>",
    eq: "=",
    not_eq: "<>",
    and: "AND",
    or: "OR",
    xor: "XOR",
    bitwise_or: "|",
    bitwise_and: "&",
    bitwise_xor: "^"
    # pg_bitwise_xor: "#",
    # pg_bitwise_shift_left: "<<",
    # pg_bitwise_shift_right: ">>",
    # pg_regex_match: "~",
    # pg_regex_imatch: "~*",
    # pg_regex_not_match: "!~",
    # pg_regex_not_imatch: "!~*"
  }

  test "ops work" do
    for {name, op} <- @ops do
      assert {:ok, [%SqlParser.Query{} = doc]} =
               SqlParser.parse("SELECT * FROM a WHERE b.a #{op} d")

      assert %SqlParser.Query{
               body: %SqlParser.Select{
                 selection: %SqlParser.Expr{
                   type: :binary_op,
                   value: %SqlParser.BinaryOp{
                     left: _,
                     op: ^name,
                     right: _
                   }
                 }
               }
             } = doc
    end
  end

  test "join work" do
    assert {:ok, [%SqlParser.Query{} = doc]} =
             SqlParser.parse("SELECT * FROM a JOIN b on a.id = b.a_id WHERE b.a = d")

    assert %SqlParser.Query{
             body: %SqlParser.Select{
               from: [
                 %SqlParser.TableWithJoins{
                   relation: %SqlParser.Table{
                     name: %SqlParser.ObjectName{
                       names: [%SqlParser.Ident{quote_style: nil, value: "a"}]
                     }
                   },
                   joins: [
                     %SqlParser.Join{
                       join_operator: %SqlParser.JoinOperator{
                         operator: %{
                           __struct__: SqlParser.JoinConstraint,
                           constraint: %SqlParser.Expr{
                             type: :binary_op,
                             value: %SqlParser.BinaryOp{
                               left: %SqlParser.Expr{
                                 type: :compound_identifier,
                                 value: [
                                   %SqlParser.Ident{quote_style: nil, value: "a"},
                                   %SqlParser.Ident{quote_style: nil, value: "id"}
                                 ]
                               },
                               op: :eq,
                               right: %SqlParser.Expr{
                                 type: :compound_identifier,
                                 value: [
                                   %SqlParser.Ident{quote_style: nil, value: "b"},
                                   %SqlParser.Ident{quote_style: nil, value: "a_id"}
                                 ]
                               }
                             }
                           },
                           kind: :on
                         },
                         kind: :inner
                       },
                       relation: %SqlParser.Table{
                         name: %SqlParser.ObjectName{
                           names: [%SqlParser.Ident{quote_style: nil, value: "b"}]
                         }
                       }
                     }
                   ]
                 }
               ]
             }
           } = doc
  end

  @exprs %{
    %SqlParser.Expr{
      type: :binary_op,
      value: %SqlParser.BinaryOp{
        left: %SqlParser.Expr{
          type: :identifier,
          value: %SqlParser.Ident{quote_style: nil, value: "a"}
        },
        op: :eq,
        right: %SqlParser.Expr{
          type: :value,
          value: %SqlParser.Number{
            long: false,
            value: "18446744073709551616.18446744073709551616"
          }
        }
      }
    } => "a = 18446744073709551616.18446744073709551616",
    %SqlParser.Expr{
      type: :is_null,
      value: %SqlParser.Expr{
        type: :identifier,
        value: %SqlParser.Ident{quote_style: nil, value: "c"}
      }
    } => "c IS NULL"
  }

  test "expr work" do
    for {expected_selection, expr} <- @exprs do
      assert {:ok, [query]} = SqlParser.parse("SELECT c as b from a WHERE #{expr}")

      assert %SqlParser.Query{
               body: %SqlParser.Select{
                 selection: selection
               }
             } = query

      assert expected_selection == selection
    end
  end

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
end
