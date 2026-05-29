## Algorithm HyperLogLog(input: multiset M)

**HyperLogLog is a cardinality estimation algorithm designed to answer: "How many distinct elements are in a very large dataset?"**


```
assume m = 2^b
initialize m registers M[1..m] to 0
  
  for v in M do
    x := hash(v)
    j := 1 + [first b bits of x]      // register index
    w := [remaining bits of x]        // trailing bits
    M[j] := max(M[j], ρ(w))           // track max leading zeros
  
  // Compute harmonic mean-based estimate
  Z := Σ(2^(-M[j])) for j=1 to m
  E := α_m * m^2 / Z
  
  // Apply corrections for edge cases
  if E ≤ 2.5m and V > 0:             // small cardinality
    E := m * log(m/V)                // use empty register count
  else if E > (1/30)*2^32:            // large cardinality
    E := -2^32 * log(1 - E/2^32)     // handle hash collisions
  
  return E
```

[Link to paper](https://algo.inria.fr/flajolet/Publications/FlFuGaMe07.pdf)