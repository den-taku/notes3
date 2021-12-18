from mip import Model, maximize, BINARY, xsum, OptimizationStatus

# ユーザが自分で決める物理，魔法，速度の下限値
pin = 25
min = 40
sin = 60

# ユーザのプレーヤーのレベル
lebel = 35

# 全装備のデータ
l = 10
m = 10
n = 10
# 物理
pa = [3, 4, 2, 5, 6, 3, 5, 6, 3, 2]
pb = [5, 7, 1, 10, 3, 4, 5, 6, 2, 2]
pc = [12, 45, 54, 43, 43, 5, 23, 34, 42, 43]
# 魔法
ma = [4, 6, 3, 8, 9, 1, 4, 3, 6, 7]
mb = [2, 4, 10, 4, 4, 4, 4, 1, 7, 8]
mc = [100, 9, 12, 43, 32, 5, 33, 24, 12, 93]
# 速度
sa = [12, 34, 32, 43, 54, 23, 32, 12, 18, 73]
sb = [32, 32, 12, 56, 64, 33, 43, 54, 64, 12]
sc = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
# 防御力
da = [43, 43, 23, 75, 52, 12, 32, 54, 32, 21]
db = [32, 32, 32, 54, 65, 54, 42, 75, 32, 42]
dc = [3, 3, 3, 3, 3, 3, 3, 3, 3, 3]
# ランク
ra = [12, 13, 11, 19, 11, 9, 6, 10, 6, 12]
rb = [12, 18, 18, 15, 20, 9, 10, 12, 14, 18]
rc = [20, 12, 14, 17, 19, 18, 14, 13, 14, 12]

# まずモデルを定義していきます
solver = Model("最強装備シミュレータ(仮)")

# 変数を定義します．今回はa, b, cです．
# solverのadd_varメソッドを呼び出すと引数に指定した名前の変数がモデルの中に作成されます．
# その変数へのアドレスが返ってくるので，それをa，b，cという配列に保存しておきます．
# 変数は0か1しかとらないのでBINARYという種類の変数として定義しています．
# INTEGERとして定義して，後で制約で0以上1以下と追加しても問題としては同じです．
a = [solver.add_var('a{}'.format(i), var_type=BINARY) for i in range(l)]
b = [solver.add_var('b{}'.format(i), var_type=BINARY) for i in range(m)]
c = [solver.add_var('c{}'.format(i), var_type=BINARY) for i in range(n)]

# モデルに制約を追加していきます．add_constrメソッドを呼び出します．
# 総和はfor文で計算しても良いのですが，
# Python-MIPが提供するxsum関数で計算した方が高速なのでそちらを利用します．

# 頭，体，武器は一つずつしか装備できません
solver.add_constr(xsum(a[i] for i in range(l)) <= 1)
solver.add_constr(xsum(b[i] for i in range(m)) <= 1)
solver.add_constr(xsum(c[i] for i in range(n)) <= 1)

# 物理，魔法，速度，防御力，ランクの合計を表す変数を用意します
P = solver.add_var('P')
solver.add_constr(xsum(a[i] * pa[i] for i in range(l)) + xsum(b[i] * pb[i]
                  for i in range(m)) + xsum(c[i] * pc[i] for i in range(n)) == P)
M = solver.add_var('M')
solver.add_constr(xsum(a[i] * ma[i] for i in range(l)) + xsum(b[i] * mb[i]
                  for i in range(m)) + xsum(c[i] * mc[i] for i in range(n)) == M)
S = solver.add_var('S')
solver.add_constr(xsum(a[i] * sa[i] for i in range(l)) + xsum(b[i] * sb[i]
                  for i in range(m)) + xsum(c[i] * sc[i] for i in range(n)) == S)
D = solver.add_var('D')
solver.add_constr(xsum(a[i] * da[i] for i in range(l)) + xsum(b[i] * db[i]
                  for i in range(m)) + xsum(c[i] * dc[i] for i in range(n)) == D)
R = solver.add_var('R')
solver.add_constr(xsum(a[i] * ra[i] for i in range(l)) + xsum(b[i] * rb[i]
                  for i in range(m)) + xsum(c[i] * rc[i] for i in range(n)) == R)

# ランクの総和はレベル以下です
solver.add_constr(R <= lebel)

# 物理，魔法，速度の下限値を設定します
solver.add_constr(pin <= P)
solver.add_constr(min <= M)
solver.add_constr(sin <= S)

# 最後に目的関数を設定します
# 今回はDの最大化なので以下のように書きます
solver.objective = maximize(D)

# 定式化したモデルを特にはこのメソッドを呼び出します．
# 返り値は結果のステータスです
# 計算の過程がログとして出力されます
status = solver.optimize()

# 最適解がもとまったなら何番目の装備を使用したのかを出力しましょう
if status == OptimizationStatus.OPTIMAL:
    head = 0
    for i in range(l):
        if a[i].x >= 0.99:
            head = i
            print("頭装備は{}番目のものを使用".format(head))
            break
    body = 0
    for i in range(m):
        if b[i].x >= 0.99:
            body = i
            print("体装備は{}番目のものを使用".format(body))
            break
    hand = 0
    for i in range(m):
        if c[i].x >= 0.99:
            hand = i
            print("武器は{}番目のものを使用".format(hand))
            break
    print("物理:{}，魔法:{}，速度:{}，防御力:{}，ランク:{}".format(P.x, M.x, S.x, D.x, R.x))
else:
    print("そのような装備の組み合わせはありません")
