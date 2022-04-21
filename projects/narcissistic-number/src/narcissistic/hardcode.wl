const[base_, list_] := TemplateApply[
    "pub const BASE`1`: [u128; `2`] =[`3`];",
    {
        base,
        Length@list,
        StringRiffle[list, ","]
    }
];

base6 = Import["http://oeis.org/A010348/b010348.txt", "Table"][[1 ;; 30, -1]];
base7 = Import["http://oeis.org/A010350/b010350.txt", "Table"][[1 ;; 59, -1]];
base8 = Import["http://oeis.org/A010354/b010354.txt", "Table"][[1 ;; 62, -1]];
base9 = Import["http://oeis.org/A010353/b010353.txt", "Table"][[1 ;; 58, -1]];
base10 = Import["http://oeis.org/A005188/b005188.txt", "Table"][[1 ;; 88, -1]];
