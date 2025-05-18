fn main() {
    use S::I;

    // 全体的に効率がひどく悪いが一旦無視する
    let t: Tape = (vec![I, I, I], I, vec![]);
    let (q, delta) = add_one();
    let r = exec(&delta, &q, &t);
    println!("t = {:?}, r = {:?}", t, r);
}

// 1.1.3
//
// チューリング機械の定義
// Post らによって改良された定義
//
// M = (Q, Σ, q_0, δ)
//
// Q は有限の状態集合
// Σ は有限のシンボル集合. B と表記する空白記号を含む
// q_0 ∈ Q は初期状態
// δ は状態遷移関数. 現在の状態 q と現在のヘッド位置 s の組を引数に取り、
//     次の状態 q' と現在のヘッド位置に書き出すシンボル s' と書き出した後のヘッドの移動方向を指定する3つ組を返す
//   δ: Q × Σ → Q × Σ × D
//

// 1.2.1
//
// <program> ::= (<Q>, <entryList>)              // 初期状態 q_0 と δ
// <entry>   ::= ((<Q>, <S>) => (<Q>, <S>, <D>)) // δ の要素
// <Q>       ::= <string>                        // 状態 Q . M | H
// <S>       ::= B | <string>                    // 集合 Σ . B | I | O
// <D>       ::= L | R                           // ヘッダーの移動方向

// TM

/// 方向
enum D {
    L,
    R,
}

/// シンボル
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum S {
    B,
    I,
    O,
}

/// 状態
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Q {
    H,
    M,
}

/// 状態遷移関数δ (<entryList>)
type Delta = Vec<((Q, S), (Q, S, D))>;

/// プログラム (<program>)
type Program = (Q, Delta);

/// テープ (<T>)
type Tape = (Vec<S>, S, Vec<S>);

fn add_one() -> Program {
    use D::L;
    use Q::{H, M};
    use S::{B, I, O};

    (
        M,
        vec![
            ((M, I), (M, O, L)),
            ((M, O), (H, I, L)),
            ((M, B), (H, I, L)),
        ],
    )
}

// Eval

fn hd(ss: &Vec<S>) -> S {
    // List は先頭を操作するが、 Vec で実装しているため末尾を操作する
    ss.last().copied().unwrap_or(S::B)
}

fn tl(ss: &Vec<S>) -> Vec<S> {
    let mut ss = ss.clone();
    ss.pop();
    ss
}

fn cons(s: S, ss: &Vec<S>) -> Vec<S> {
    match (s, ss.is_empty()) {
        (S::B, true) => return vec![],
        (s, _) => {
            let mut ss = ss.clone();
            ss.push(s);
            ss
        }
    }
}

fn move_l(tape: &Tape) -> Tape {
    let (left, head, right) = tape;
    (tl(&left), hd(&left), cons(*head, &right))
}

fn move_r(tape: &Tape) -> Tape {
    let (left, head, right) = tape;
    (cons(*head, &left), hd(&right), tl(&right))
}

fn mov(d: &D, tape: &Tape) -> Tape {
    match d {
        D::L => move_l(tape),
        D::R => move_r(tape),
    }
}

fn exec(delta: &Delta, q: &Q, (left, head, right): &Tape) -> Tape {
    match delta.iter().find(|(it, _)| it == &(*q, *head)) {
        None => (left.clone(), *head, right.clone()),
        Some((_, (q_, s, d))) => exec(delta, q_, &mov(d, &(left.clone(), *s, right.clone()))),
    }
}
