# 矩阵乘法

### 二阶二元乘法

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


$a_1, a_2, a_3, a_4$ 不能同时为 $0$, 不然就恒成立了.

正好有 100 个解最高一位数, 352 个解最高两位数, 6 个解最高三位数, 共计 458 个解.

其中最小的值是 36, 最大的值是 1018.

$$
\begin{bmatrix}
2 & 2 \\
3 & 2 \\
\end{bmatrix}
\times
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
\times
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