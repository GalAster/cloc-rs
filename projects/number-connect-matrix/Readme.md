# 矩阵乘法

### 二阶二元乘法

设矩阵 $A, B$ 为:

$$
A = \begin{bmatrix}
a_1 & a_2 \\
a_3 & a_4 \\
\end{bmatrix}\quad
B = \begin{bmatrix}
b_1 & b_2 \\
b_3 & b_4 \\
\end{bmatrix}
$$

则待解得方程为 $A \cdot B = 10 A + B$

其中 $B$ 的参数都是个位数, $A$ 可以是任意自然数, 但 $a_1, a_2, a_3, a_4$ 不能同时为 $0$.

如果 $A$ 是零矩阵那 $B$ 也只能是零矩阵, 方程就退化了无意义.



正好有 100 个解最高一位数, 352 个解最高两位数, 6 个解最高三位数, 共计 458 个解.

其中最小的值是 36, 最大的值是 1018.

$$
\begin{bmatrix}
2 & 2 \\
3 & 2 \\
\end{bmatrix}
\begin{bmatrix}
8 & 4 \\
6 & 8 \\
\end{bmatrix}
=
\begin{bmatrix}
28 & 24 \\
36 & 28 \\
\end{bmatrix}
$$

$$
\begin{bmatrix}
101 & 70 \\
30 & 21 \\
\end{bmatrix}
\begin{bmatrix}
8 & 7 \\
3 & 0 \\
\end{bmatrix}
=
\begin{bmatrix}
1018 & 707 \\
303 & 210 \\
\end{bmatrix}
$$

## 高阶二元乘法

在上述解中我们发现了一个特殊的解.

$$
\begin{bmatrix}
3 & 3 \\
3 & 3 \\
\end{bmatrix}
\begin{bmatrix}
6 & 6 \\
6 & 6 \\
\end{bmatrix}=
\begin{bmatrix}
36 & 36 \\
36 & 36 \\
\end{bmatrix}
$$

那么, 对于更高阶的矩阵, 这样的解还存在吗?

设 $r$ 阶矩阵 $A_r, B_r$ 为:

$$
A_r = \begin{bmatrix}
a      & \cdots & a      \\
\vdots & \ddots & \vdots \\
a      & \cdots & a      \\
\end{bmatrix}\quad
B_r = \begin{bmatrix}
b      & \cdots & b      \\
\vdots & \ddots & \vdots \\
b      & \cdots & b      \\
\end{bmatrix}
$$

易知:

$$
A_r \cdot B_r = \begin{bmatrix}
a b r  & \cdots & a b r  \\
\vdots & \ddots & \vdots \\
a b r  & \cdots & a b r  \\
\end{bmatrix}
$$

所以我们只要解方程 $n a b = 10 a + b$ 即可

我们可以解得如下五个解:

$$
\begin{array}{rrr}
n & a & b \\
\hline\\
2 & 3 & 5 \\
3 & 1 & 5 \\
3 & 2 & 4 \\
6 & 1 & 2 \\
11 & 1 & 1 \\
\end{array}
$$

所以这样的矩阵只在 $2, 3, 6, 11$ 阶时存在.

## 高阶多元乘法

更一般的, 对于 $n$ 个 $r$ 阶矩阵相乘的情形, 用 $x_i$ 表示矩阵元素, 则有:

$$r^{n - 1}\prod_{i=1}^{n}x_i = \sum_{i=1}^{n}10^{n-i}x_i$$