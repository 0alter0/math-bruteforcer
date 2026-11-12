# math-bruteforcer
Takes the digits of an input number and makes them equal to a target number

# Use
Whatever you may need it for, not meant for anything really specific. I made it in my free time over the course of a month and a half


# Info
The performance of the program is extremely optimized. A 12 digit to 12 digit calculation takes only 40 seconds (in earlier versions, prior to initial release, this would take up to 20 minutes).

These 12-12 digit calculations are also highly accurate (usually within 20,000).

The average 12-12 calculation has to search through about 5 **billion** equations just to get close.

Supports hundreds of possible math operations (listed further below).

# Usage
1. Download the .exe (or build it yourself via ``rustc -O main.rs -o math.exe``).
2. Open command prompt and cd to the directory
3. Simply type in math
4. Enter your input number
5. Enter your target number
6. Enter max operations
7. Let it run

# Operators
The search engine supports unary, binary, ternary, and quaternary mathematical operations.

### Unary Operations

| Category | Operations |
|---|---|
| Basic | `abs`, `neg`, `sign`, `ceil`, `floor`, `round`, `trunc`, `fract` |
| Powers & Roots | `square`, `cube`, `cbrt`, `sqrt`, `fourth_power`, `fifth_power`, `reciprocal`, `reciprocal_square`, `reciprocal_sqrt` |
| Exponents & Logarithms | `exp`, `exp2`, `10^`, `2^`, `negative_exp`, `ln`, `ln1p`, `log2`, `log10`, `log_abs`, `log2_abs`, `log10_abs`, `log1mexp`, `expm1` |
| Trigonometric | `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `csc`, `sec`, `cot`, `sin_pi`, `cos_pi`, `tan_pi` |
| Hyperbolic | `sinh`, `cosh`, `tanh`, `asinh`, `acosh`, `atanh`, `sech`, `csch`, `coth`, `sinhc`, `coshm1` |
| Angle Conversion | `degrees`, `radians` |
| Factorials & Combinatorics | `factorial`, `double_factorial`, `primorial`, `gamma`, `log_gamma`, `derangement`, `partition_number`, `bell_number`, `catalan`, `triangular` |
| Number Theory | `is_prime`, `next_prime`, `prime_count`, `nth_prime`, `totient`, `divisor_count`, `divisor_sum`, `prime_omega`, `mobius`, `radical`, `liouville`, `carmichael_lambda`, `aliquot_sum`, `is_squarefree`, `is_carmichael`, `is_perfect_number`, `is_perfect_square`, `is_even`, `is_odd` |
| Digit & Integer Operations | `digit_sum`, `digit_product`, `digital_root`, `alternating_digit_sum`, `digit_count`, `reverse_digits`, `popcount`, `bit_length` |
| Special Functions | `erf`, `erfc`, `erfi`, `erfcx`, `inverse_erf`, `digamma`, `trigamma`, `zeta`, `eta`, `lambert_w`, `sine_integral`, `cosine_integral`, `bessel_j0`, `bessel_y0` |
| Probability & Statistics | `normal_pdf`, `normal_cdf`, `normal_quantile`, `logistic`, `softplus`, `logit`, `softsign`, `heaviside` |
| Trigonometric Functions | `gudermannian`, `inverse_gudermannian`, `versin`, `vercosine`, `haversine`, `havercosine`, `chord`, `exsec`, `excsc` |
| Integer Sequences | `fibonacci`, `lucas`, `pell`, `tribonacci`, `bernoulli`, `euler_number` |
| Other | `sinc`, `exprel`, `percent`, `is_finite`, `is_integer`, `is_nan` |

### Binary Operations

| Category | Operations |
|---|---|
| Basic Arithmetic | `+`, `-`, `*`, `/`, `%`, `^` |
| Comparisons & Selection | `min`, `max`, `min_magnitude`, `max_magnitude`, `next_after` |
| Powers & Logarithms | `root`, `log_base`, `polylog` |
| Number Theory | `gcd`, `lcm`, `is_coprime`, `is_divisible`, `modular_inverse`, `is_power_of`, `multiplicative_order`, `legendre`, `jacobi`, `kronecker` |
| Combinatorics | `ncr`, `npr`, `stirling1`, `stirling2`, `eulerian`, `polygonal` |
| Means & Averages | `average`, `geometric_mean`, `harmonic_mean`, `arithmetic_geometric_mean`, `logarithmic_mean`, `contraharmonic_mean`, `heronian_mean`, `identric_mean`, `lehmer_mean_3`, `rms` |
| Mathematical Functions | `hypot`, `atan2`, `beta`, `log_add_exp`, `x_log_y`, `sigma_k`, `jordan_totient` |
| Bitwise Operations | `bit_and`, `bit_or`, `bit_xor`, `hamming_distance` |
| Additional Operations | `distance`, `copy_sign`, `floor_div`, `euclidean_mod`, `sum_squares`, `difference_squares`, `product_minus_sum`, `parallel_sum`, `reciprocal_sum`, `smooth_minimum`, `round_to` |

### Ternary Operations

| Operation | Function |
|---|---|
| `clamp` | Restricts a value to a specified range |
| `lerp` | Linear interpolation |
| `smoothstep` | Smooth interpolation between two edges |
| `mod_pow` | Modular exponentiation |
| `binomial_cdf` | Binomial cumulative distribution function |

### Quaternary Operations

| Operation | Function |
|---|---|
| `crt` | Chinese Remainder Theorem |
