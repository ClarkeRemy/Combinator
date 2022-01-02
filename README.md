## Duo-Combinators
### Atop
```
      /x:A
M1-D2<
      \y:B 

M1.AT(D2) (x,y)
M1: Fn(C)->D
D2: Fn(A,B)->C
```

### Appose
```
   /M2-x:A
D1<
   \M2-y:A

D1.AP(M2) (x,y)
D1: Fn(B,B)->C
M2: Fn(A)->B
```

### Compose
```
M1-M2-y:B

M1.CP(M2) (y)
M1: Fn(D)->E
M2: Fn(B)->C
```

### Hook
```
   /x:A
D1<
   \M2-y:B

D1.HK(M2)(x,y)
D1: Fn(A,C)->D
M2: Fn(B)->C
```

### MonoHook
```
   (y)
   / \
D1<   \
   \M2-y:B

D1.MH(M2)(y)
D1: Fn(B,C)->D
M2: Fn(B)->C
```
### RevHook
```
    (y)
   /   \ /M-x:A
D1<     X
   \   / \-y:B
 (M2(x))

D1.RH(M2)(x,y)
D1: Fn(B,C)->D
M2: Fn(A)->C
```
## Mono-Combinator
### Relflex
```
   (y)
   / \
D1<   >y:B
   \ /
   (y)

D1.RF()(y)
D1 : (B,B)->C
```

### Flip
```
   (y)
   / \ /x:A
D1<   X
   \ / \y:B
   (x)

D1.FL()(x,y)
D1 : Fn(B,A)->C
```