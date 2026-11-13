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
| Basic | `abs`, `neg`, `sign`, `ceil`, `floor`, `round`, `trunc`, `fract`, `frac`, `round_even`, `round_away`, `rint`, `nearbyint`, `one_minus` |
| Powers & Roots | `square`, `cube`, `cbrt`, `sqrt`, `sqrt_abs`, `fourth_power`, `fifth_power`, `reciprocal`, `reciprocal_square`, `reciprocal_sqrt`, `inverse_square`, `inverse_cube`, `rsqrt` |
| Exponents & Logarithms | `exp`, `exp2`, `10^`, `2^`, `exp_negative`, `ln`, `ln1p`, `log2`, `log10`, `ln_abs`, `log2_abs`, `log10_abs`, `log1mexp`, `log1pmx`, `expm1` |
| Trigonometric | `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `acsc`, `asec`, `acot`, `csc`, `sec`, `cot`, `sin_pi`, `cos_pi`, `tan_pi` |
| Hyperbolic | `sinh`, `cosh`, `tanh`, `asinh`, `acosh`, `atanh`, `sech`, `csch`, `coth`, `acsch`, `asech`, `acoth`, `sinhc`, `coshm1` |
| Angle Conversion | `degrees`, `radians` |
| Factorials & Combinatorics | `factorial`, `double_factorial`, `primorial`, `gamma`, `log_gamma`, `derangement`, `subfactorial`, `partition_number`, `bell_number`, `catalan`, `triangular`, `harmonic_number`, `superfactorial`, `hyperfactorial` |
| Number Theory | `is_prime`, `next_prime`, `prime_count`, `nth_prime`, `totient`, `divisor_count`, `divisor_sum`, `prime_omega`, `omega`, `big_omega`, `mobius`, `radical`, `liouville`, `carmichael_lambda`, `aliquot_sum`, `is_squarefree`, `is_carmichael`, `is_perfect_number`, `is_perfect_square`, `is_composite`, `is_prime_power`, `smallest_prime_factor`, `largest_prime_factor`, `is_abundant`, `is_deficient`, `is_even`, `is_odd` |
| Digit & Integer Operations | `digit_sum`, `digit_product`, `digital_root`, `alternating_digit_sum`, `digit_count`, `reverse_digits` |
| Bitwise Operations | `popcount`, `bit_length`, `bit_not`, `count_ones`, `count_zeros`, `leading_zeros`, `trailing_zeros`, `reverse_bits`, `byte_swap` |
| Floating-Point & Classification | `is_finite`, `is_integer`, `is_nan`, `is_normal`, `is_subnormal`, `is_infinite`, `is_sign_positive`, `is_sign_negative`, `is_positive`, `is_negative`, `is_zero`, `next_up`, `next_down`, `ulp`, `ilogb` |
| Special Functions | `erf`, `erfc`, `erfi`, `erfcx`, `inverse_erf`, `digamma`, `trigamma`, `zeta`, `eta`, `lambert_w`, `sine_integral`, `cosine_integral`, `bessel_j0`, `bessel_y0`, `airy_ai`, `airy_bi`, `dawson`, `expint`, `bessel_i0`, `bessel_i1`, `bessel_k0`, `bessel_k1` |
| Probability & Statistics | `normal_pdf`, `normal_cdf`, `normal_quantile`, `logistic`, `softplus`, `logit`, `softsign`, `heaviside` |
| Trigonometric Functions | `gudermannian`, `inverse_gudermannian`, `versin`, `vercosine`, `haversine`, `havercosine`, `chord`, `exsec`, `excsc` |
| Integer Sequences | `fibonacci`, `lucas`, `pell`, `tribonacci`, `bernoulli`, `euler_number` |
| Other | `sinc`, `exprel`, `percent` |

### Binary Operations

| Category | Operations |
|---|---|
| Basic Arithmetic | `+`, `-`, `*`, `/`, `%`, `^` |
| Comparisons & Selection | `min`, `max`, `min_magnitude`, `max_magnitude`, `next_after`, `equal`, `not_equal`, `less_than`, `greater_than`, `less_equal`, `greater_equal` |
| Powers & Logarithms | `root`, `integer_root`, `log_base`, `signed_pow`, `polylog`, `logaddexp`, `logaddexp2`, `log_sub_exp`, `xlogy` |
| Number Theory | `gcd`, `lcm`, `smallest_common_multiple`, `is_coprime`, `is_divisible`, `modular_inverse`, `is_power_of`, `multiplicative_order`, `legendre`, `jacobi`, `kronecker`, `sigma_k`, `jordan_totient`, `div_rem_quotient`, `div_rem_remainder`, `extended_gcd_gcd`, `extended_gcd_x`, `extended_gcd_y` |
| Combinatorics | `nCr`, `nPr`, `stirling1`, `stirling2`, `eulerian`, `polygonal`, `binomial`, `choose`, `permute` |
| Means & Averages | `average`, `geometric_mean`, `harmonic_mean`, `agm`, `logarithmic_mean`, `contraharmonic_mean`, `heronian_mean`, `identric_mean`, `lehmer_mean_3`, `rms` |
| Mathematical Functions | `hypot`, `atan2`, `beta`, `hurwitz_zeta`, `incomplete_gamma`, `regularized_gamma_p`, `regularized_gamma_q` |
| Bitwise Operations | `bit_and`, `bit_or`, `bit_xor`, `hamming_distance`, `shift_left`, `shift_right`, `rotate_left`, `rotate_right`, `bit_test`, `set_bit`, `clear_bit` |
| Probability & Statistics | `exponential_pdf`, `exponential_cdf`, `poisson_pmf`, `poisson_cdf`, `student_t_pdf`, `student_t_cdf`, `chi_squared_pdf`, `chi_squared_cdf` |
| Additional Operations | `distance`, `copysign`, `floor_div`, `rem_euclid`, `fmod`, `remainder`, `quotient`, `sum_squares`, `difference_squares`, `product_minus_sum`, `parallel_sum`, `reciprocal_sum`, `smooth_minimum`, `round_to` |

### Ternary Operations

| Operation | Function |
|---|---|
| `clamp` | Restricts a value to a specified range |
| `lerp` | Linear interpolation |
| `smoothstep` | Smooth interpolation between two edges |
| `mod_pow` | Modular exponentiation |
| `binomial_cdf` | Binomial cumulative distribution function |
| `fma` | Fused multiply-add |
| `mul_add` | Multiply-add |
| `median3` | Median of three values |
| `hypot3` | Three-dimensional hypotenuse |
| `inv_lerp` | Inverse linear interpolation |
| `is_between` | Checks whether a value is between two bounds |
| `binomial_pmf` | Binomial probability mass function |
| `beta_inc` | Incomplete beta function |
| `regularized_beta` | Regularized incomplete beta function |
| `modular_add` | Modular addition |
| `modular_sub` | Modular subtraction |
| `modular_mul` | Modular multiplication |
| `multinomial` | Multinomial coefficient |
| `gamma_pdf` | Gamma probability density function |
| `gamma_cdf` | Gamma cumulative distribution function |
| `beta_pdf` | Beta probability density function |
| `beta_cdf` | Beta cumulative distribution function |
| `uniform_pdf` | Uniform probability density function |
| `uniform_cdf` | Uniform cumulative distribution function |

### Quaternary Operations

| Operation | Function |
|---|---|
| `crt` | Chinese Remainder Theorem |
| `bilinear` | Bilinear interpolation |
| `multilinear` | Multilinear interpolation |
| `cubic_interp` | Cubic interpolation |

### Quinary Operations

| Operation | Function |
|---|---|
| `remap` | Maps a value from one range to another |

### Complex Unary Operations

| Operation | Function |
|---|---|
| `conj` | Complex conjugate |
| `real` | Extracts the real component |
| `imag` | Extracts the imaginary component |
| `arg` | Complex argument |
| `norm` | Complex norm |
| `phase` | Complex phase |
| `complex_exp` | Complex exponential |
| `complex_ln` | Complex natural logarithm |
| `complex_sqrt` | Complex square root |
| `complex_sin` | Complex sine |
| `complex_cos` | Complex cosine |
| `complex_tan` | Complex tangent |

### Complex Binary Operations

| Operation | Function |
|---|---|
| `complex_add` | Complex addition |
| `complex_sub` | Complex subtraction |
| `complex_mul` | Complex multiplication |
| `complex_div` | Complex division |
| `complex_pow` | Complex exponentiation |
| `complex_log` | Complex logarithm |
