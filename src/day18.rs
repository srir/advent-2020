use aoc_runner_derive::aoc;

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Add,
    Mul
}

struct ParseResult {
    result: usize,
    bytes_parsed: usize
}

fn parse_expr(slice: &[u8]) -> ParseResult {
    let mut result = 0;
    let mut op = Op::Add;

    let mut idx = 0;

    while idx < slice.len() {
        let c = slice[idx] as char;

        match c {
            '+' => op = Op::Add,
            '*' => op = Op::Mul,
            ')' => return ParseResult { result, bytes_parsed: idx + 1 },
            '0'..='9' => {
                let new_val = c.to_digit(10).unwrap() as usize;
                if op == Op::Add {
                    result += new_val;
                } else {
                    result *= new_val;
                }
            },
            '(' => {
                let inner = parse_expr(&slice[idx+1..]);

                if op == Op::Add {
                    result += inner.result;
                } else {
                    result *= inner.result;
                }
                idx += inner.bytes_parsed;
            }
            _ => ()
        }

        idx += 1;
    }

    ParseResult { result, bytes_parsed: idx }
}

#[aoc(day18, part1)]
fn sum_of_exprs(input: &str) -> usize {
    input.lines().map(|line| {
        parse_expr(line.as_bytes()).result
    }).sum()
}

fn parse_expr_2(slice: &[u8]) -> usize {
    let mut result = 0;

    let mut idx = 0;

    while idx < slice.len() {
        let c = slice[idx] as char;

        match c {
            '+' => (),
            '*' => {
                result *= parse_expr_2(&slice[idx+1..]);
                break;
            },
            ')' => return result,
            '0'..='9' => {
                result += c.to_digit(10).unwrap() as usize;
            },
            '(' => {
                let closing_idx = slice.iter().enumerate().skip(idx+1)
                    .try_fold(1, |open_count, (j, c)| {
                        match (open_count, c) {
                            (_, b'(') => Ok(open_count + 1),
                            (1, b')') => Err(j),
                            (_, b')') => Ok(open_count - 1),
                            _ => Ok(open_count)
                        }
                    }).expect_err("no closing paren");

                result += parse_expr_2(&slice[idx+1..closing_idx]);
                idx = closing_idx;
            }
            _ => ()
        }

        idx += 1;
    }

    result
}

#[aoc(day18, part2)]
fn sum_of_exprs_2(input: &str) -> usize {
    input.lines().map(|line| {
        parse_expr_2(line.as_bytes())
    }).sum()
}
