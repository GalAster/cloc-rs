(* ::Package:: *)

SetDirectory@NotebookDirectory[];


texLine[l_] := StringRiffle[l, " & "] <> " \\\\";
texMatrix[m_, kind_] := TemplateApply[
    "\\begin{`1`}
`2`
\\end{`1`}"
    ,
    {
        kind,
        StringRiffle[texLine /@ m, "\n"]
    }
];
asTex[solution_] := Block[
    {
        a = A /. solution,
        b = B /. solution
    },
    StringRiffle[{
        "$$",
        texMatrix[a, "vmatrix"],

        texMatrix[b, "vmatrix"],
        "=",
        texMatrix[a . b, "vmatrix"],
        "$$"
    }, "\n"]
];


A = {{a1, a2}, {a3, a4}};
B = {{b1, b2}, {b3, b4}};
sol = Solve[
    {
        A . B == 10 A + B,
        0 <= {a1, a2, a3, a4},
        0 <= {b1, b2, b3, b4} <= 9,
        a1 != 0 || a2 != 0 || a3 != 0 || a4 != 0
    },
    {a1, a2, a3, a4, b1, b2, b3, b4},
    Integers
];


d1 = Select[sol, Max[{a, b}] <= 9 /. #&];
d2 = Select[sol, 9 < Max[{a, b}] <= 99 /. #&];
d3 = Select[sol, 99 < Max[{a, b}] /. #&];
sol2 = SortBy[sol, Max[a1, a2, a3, a4] /. #&];
sol3 = SortBy[sol, Max[a . b] /. #&];


Export[
    "MatrixR2N2_1.csv",
    {a1, a2, a3, a4, b1, b2, b3, b4} /. sol2,
    "CSV",
    Alignment -> Right,
    "TableHeadings" -> {a1, a2, a3, a4, b1, b2, b3, b4}
]
