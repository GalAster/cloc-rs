# Vertical Multiplication Steps



#### Decimal Multiplication

```rust
BigInt::from(12), &BigInt::from(345), 10)
```

```js
     12
 ×  345
--------
 =   60
 +  48.
 + 36..
--------
 = 4140
```

#### Hexadecimal Multiplication

```rust
BigInt::from(12), &BigInt::from(345), 16)
```

```js
     c
 × 159
-------
 =  6c
 + 3c.
 + c..
-------
 = 102c
````

#### Binary Multiplication

```rust
BigInt::from(12), &BigInt::from(345), 2)
```

```js
           1100
 ×    101011001
----------------
 =         1100
 +           0.
 +          0..
 +      1100...
 +     1100....
 +       0.....
 +   1100......
 +     0.......
 + 1100........
----------------
 = 1000000101100
```