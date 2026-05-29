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

---
**Test Results**

| Elements | Actual Distinct | Estimated | Error |
|----------|-----------------|-----------|-------|
| 100      | 100             | 100.31    | 0.31% |
| 1,000    | 1,000           | 999.90    | 0.01% |
| 10,000   | 10,000          | 9,972.65  | 0.27% |
| 100,000  | 100,000         | 101,268.10| 1.27% |

---

**Poisson Analysis**

#### Step 1: Express the Indicator Z as a Sum

Under the Poisson model, the expected value of indicator Z is:

$$E_{P(\lambda)}(Z) = \sum_{k_1, \ldots, k_m \geq 1} \left(\prod_{j=1}^{m} \mathbb{P}(\text{register } j \text{ has } k_j \text{ elements})\right) \cdot g\left(\frac{\lambda}{m2^{k_j}}\right)$$

Where:
- λ = Poisson rate (representing cardinality)
- m = number of registers
- g() = observable behavior from register values
- k_j = counts in each register

This sum is difficult to analyze directly.

#### Step 2: Convert to Integral Form

Using the identity:
$$\frac{1}{a^2} = \int_0^\infty t e^{-at} dt$$

The authors transform the discrete sum into an integral:

$$E_{P(\lambda)}(Z) = H\left(\frac{\lambda}{m}\right)$$

where 

$$H(x) := x \int_0^{\infty} G(x, xu)^m du$$

**Why this helps**: Integrals are much easier to analyze than discrete sums using integral calculus and complex analysis.

#### Step 3: Apply Mellin Transform Analysis

The authors use **Mellin transforms**, a technique from complex analysis:

$$M[f](s) = \int_0^\infty f(u) u^{s-1} du$$

**Key steps**:
1. Break G(x, xu) into regions where it behaves predictably
2. Use saddle-point methods to extract dominant asymptotic behavior
3. Bound error terms rigorously
4. Apply Mellin inversion

**Main result from Lemlin 1**:
$$H(x) = x \int_0^{\infty} (f(u))^m du + o(1)$$

where f(u) is a specific function they analyze in detail.

### The Final Result

After this complex analysis, the paper proves:

$$E_{P(\lambda)}(Z) = \frac{n}{\alpha_m \cdot m} + \text{lower order terms}$$

Where:
$$\alpha_m = \frac{1}{m \cdot J_0(m)} \approx \frac{1}{2\log 2} = 0.7213...$$

**Interpretation**: By dividing the raw indicator Z by the constant αₘ·m, you get an unbiased estimate of true cardinality n.

