# unification
１階単一化問題をナイーブに解く

```
$ cargo run
{ f(g(y),z) = f(x,g(y)) }
[g(y)/x][g(y)/z]
```
`[g(y)/x][g(y)/z]`は`z`に`g(y)`を代入し，`x`に`g(y)`を代入するという意味．等式制約に適用すると`f(g(y),g(y))=f(g(y),g(y))`となり，制約が満たされていることが分かる．
