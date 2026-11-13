use std::cell::RefCell;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::{self, Write};
use std::sync::{Arc, OnceLock};
use std::sync::atomic::{AtomicUsize, Ordering};

const MAX_DIGITS: usize = 80;
const MAX_EXACT_DIGITS: usize = 12;
const MAX_DEPTH: usize = 50;
const DEFAULT_CANDIDATES_PER_CELL: usize = 24;
const MAX_CANDIDATES_PER_CELL: usize = 2000;
const MAX_ABS_VALUE: f64 = 1.0e12;
const MAX_INTEGER_ARGUMENT: f64 = 1.0e12;
const EXACT_TOLERANCE: f64 = 1.0e-9;

#[derive(Clone, Copy)]
enum UnaryOp {
    Cbrt,
    Ceil,
    Floor,
    Round,
    Trunc,
    Fract,
    Abs,
    Neg,
    Square,
    Cube,
    Reciprocal,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Sinh,
    Cosh,
    Tanh,
    Asinh,
    Acosh,
    Atanh,
    Degrees,
    Radians,
    Percent,
    Exp,
    TenPow,
    TwoPow,
    Ln,
    Log2,
    Log10,
    Ln1p,
    Expm1,
    Sqrt,
    Factorial,
    Signum,
    Heaviside,
    Exp2,
    DoubleFactorial,
    Primorial,
    DigitSum,
    DigitalRoot,
    Sinc,
    Gamma,
    Logistic,
    Softplus,
    Csc,
    Sec,
    Cot,
    Sech,
    Csch,
    Coth,
    Logit,
    Softsign,
    Erf,
    Erfc,
    NormalCdf,
    Fibonacci,
    Triangular,
    Catalan,
    IsPrime,
    Totient,
    DivisorCount,
    PrimeOmega,
    Mobius,
    Popcount,
    BitLength,
    DigitProduct,
    AlternatingDigitSum,
    LogAbs,
    Log2Abs,
    Log10Abs,
    SqrtAbs,
    FourthPower,
    FifthPower,
    ReciprocalSquare,
    ReciprocalSqrt,
    NegativeExp,
    NormalPdf,
    Gudermannian,
    InverseGudermannian,
    Versin,
    Vercosine,
    Haversine,
    Havercosine,
    Chord,
    Exsec,
    Excsc,
    LogGamma,
    HarmonicNumber,
    Derangement,
    PartitionNumber,
    BellNumber,
    IsEven,
    IsOdd,
    IsPerfectSquare,
    IsPerfectNumber,
    ReverseDigits,
    DigitCount,
    Radical,
    DivisorSum,
    Liouville,
    CarmichaelLambda,
    Sinhc,
    Exprel,
    Coshm1,
    Log1mexp,
    SinPi,
    CosPi,
    TanPi,
    LambertW,
    Digamma,
    Trigamma,
    Zeta,
    Eta,
    Erfi,
    Erfcx,
    SineIntegral,
    CosineIntegral,
    BesselJ0,
    BesselY0,
    InverseErf,
    NormalQuantile,
    PrimeCount,
    NthPrime,
    NextPrime,
    IsSquarefree,
    AliquotSum,
    IsCarmichael,
    Lucas,
    Pell,
    Tribonacci,
    Bernoulli,
    EulerNumber,
    IsFinite,
    IsInteger,
    IsNan,
    RoundEven,
    RoundAway,
    Rint,
    NearbyInt,
    InverseSquare,
    InverseCube,
    Acsc,
    Asec,
    Acot,
    Acsch,
    Asech,
    Acoth,
    NextUp,
    NextDown,
    Ulp,
    Ilogb,
    IsNormal,
    IsSubnormal,
    IsInfinite,
    IsSignPositive,
    IsSignNegative,
    IsComposite,
    IsPrimePower,
    SmallestPrimeFactor,
    LargestPrimeFactor,
    Omega,
    BigOmega,
    IsAbundant,
    IsDeficient,
    Subfactorial,
    Superfactorial,
    Hyperfactorial,
    BitNot,
    CountOnes,
    CountZeros,
    LeadingZeros,
    TrailingZeros,
    ReverseBits,
    ByteSwap,
    AiryAi,
    AiryBi,
    Dawson,
    Expint,
    BesselI0,
    BesselI1,
    BesselK0,
    BesselK1,
    OneMinus,
    Frac,
    Rsqrt,
    IsPositive,
    IsNegative,
    IsZero,
    Log1pmx,
}

#[derive(Clone, Copy)]
enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Min,
    Max,
    Hypot,
    Atan2,
    LogBase,
    Root,
    Gcd,
    Lcm,
    NCr,
    NPr,
    Average,
    GeometricMean,
    HarmonicMean,
    Rms,
    Distance,
    CopySign,
    FloorDiv,
    EuclideanMod,
    LogAddExp,
    XLogY,
    Beta,
    LogarithmicMean,
    ArithmeticGeometricMean,
    SumSquares,
    DifferenceSquares,
    IsCoprime,
    Polygonal,
    BitAnd,
    BitOr,
    BitXor,
    IsDivisible,
    ContraharmonicMean,
    HeronianMean,
    ParallelSum,
    ReciprocalSum,
    IdentricMean,
    LehmerMean3,
    MaxMagnitude,
    MinMagnitude,
    NextAfter,
    HammingDistance,
    ProductMinusSum,
    SmoothMinimum,
    ModularInverse,
    IsPowerOf,
    Polylog,
    SigmaK,
    JordanTotient,
    MultiplicativeOrder,
    StirlingFirst,
    StirlingSecond,
    Eulerian,
    LegendreSymbol,
    JacobiSymbol,
    KroneckerSymbol,
    RoundTo,
    Fmod,
    Remainder,
    Quotient,
    IntegerRoot,
    SignedPow,
    SmallestCommonMultiple,
    Binomial,
    Choose,
    Permute,
    LogAddExp2,
    LogSubExp,
    HurwitzZeta,
    IncompleteGamma,
    RegularizedGammaP,
    RegularizedGammaQ,
    ShiftLeft,
    ShiftRight,
    RotateLeft,
    RotateRight,
    BitTest,
    SetBit,
    ClearBit,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    ExponentialPdf,
    ExponentialCdf,
    PoissonPmf,
    PoissonCdf,
    StudentTPdf,
    StudentTCdf,
    ChiSquaredPdf,
    ChiSquaredCdf,
    DivRemQuotient,
    DivRemRemainder,
    ExtendedGcdGcd,
    ExtendedGcdX,
    ExtendedGcdY,
}

type CachedUnaryResults = Arc<[(f64, UnaryOp)]>;
type CachedBinaryResults = Arc<[(f64, BinaryOp, bool)]>;

const THREAD_CACHE_MAX_ENTRIES: usize = 4096;

thread_local! {
    static ADDED_UNARY_RESULT_CACHE: RefCell<HashMap<u64, CachedUnaryResults>> =
        RefCell::new(HashMap::new());
    static ADDED_BINARY_RESULT_CACHE: RefCell<HashMap<(u64, u64), CachedBinaryResults>> =
        RefCell::new(HashMap::new());
}

#[derive(Clone, Copy)]
enum TernaryOp {
    Clamp,
    Lerp,
    Smoothstep,
    ModularPower,
    BinomialCdf,
    Fma,
    MulAdd,
    Median3,
    Hypot3,
    InverseLerp,
    IsBetween,
    BinomialPmf,
    BetaInc,
    RegularizedBeta,
    ModularAdd,
    ModularSub,
    ModularMul,
    Multinomial,
    GammaPdf,
    GammaCdf,
    BetaPdf,
    BetaCdf,
    UniformPdf,
    UniformCdf,
}

#[derive(Clone, Copy)]
enum QuaternaryOp {
    ChineseRemainder,
    Bilinear,
    Multilinear,
    CubicInterp,
}

#[derive(Clone, Copy)]
enum QuinaryOp {
    Remap,
}

#[derive(Clone, Copy)]
enum ComplexUnaryOp {
    Conjugate,
    Real,
    Imaginary,
    Argument,
    Norm,
    Phase,
    Exp,
    Ln,
    Sqrt,
    Sin,
    Cos,
    Tan,
}

#[derive(Clone, Copy)]
enum ComplexBinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Logarithm,
}

#[derive(Clone, Copy)]
enum ExprNode {
    Number {
        packed_digits: u32,
        len: u8,
        decimal_places: u8,
    },
    NumberRange {
        start: u8,
        len: u8,
        decimal_places: u8,
    },
    Unary(UnaryOp, usize),
    Binary(BinaryOp, usize, usize),
    Ternary(TernaryOp, usize, usize, usize),
    Quaternary(QuaternaryOp, usize, usize, usize, usize),
    Quinary(QuinaryOp, usize, usize, usize, usize, usize),
    ComplexUnary(ComplexUnaryOp, usize),
    ComplexBinary(ComplexBinaryOp, usize, usize),
}

#[derive(Clone, Copy)]
struct Candidate {
    value: f64,
    imag: f64,
    node_id: usize,
    score: f64,
    rank_score: f64,
}

struct Cell {
    candidates: Vec<Candidate>,
    seen: HashSet<(u64, u64)>,
    worst_heap: BinaryHeap<(u64, usize, u64)>,
    versions: Vec<u64>,
    bucket_indices: HashMap<i32, usize>,
    limit: usize,
    diverse: bool,
}

impl Cell {
    fn new(limit: usize) -> Self {
        Self::with_mode(limit, false)
    }

    fn new_diverse(limit: usize) -> Self {
        Self::with_mode(limit, true)
    }

    fn with_mode(limit: usize, diverse: bool) -> Self {
        Self {
            candidates: Vec::new(),
            seen: HashSet::new(),
            worst_heap: BinaryHeap::new(),
            versions: Vec::new(),
            bucket_indices: HashMap::new(),
            limit,
            diverse,
        }
    }

    fn worst(&mut self) -> Option<(f64, usize)> {
        loop {
            let &(score_bits, index, version) = self.worst_heap.peek()?;
            if self.versions.get(index).copied() == Some(version)
                && self.candidates.get(index).map(|candidate| candidate.rank_score.to_bits()) == Some(score_bits)
            {
                return Some((f64::from_bits(score_bits), index));
            }
            self.worst_heap.pop();
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
    #[inline]
    fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    #[inline]
    fn magnitude(self) -> f64 {
        self.re.hypot(self.im)
    }

    #[inline]
    fn is_finite(self) -> bool {
        self.re.is_finite() && self.im.is_finite()
    }

    #[inline]
    fn exp(self) -> Self {
        let scale = self.re.exp();
        Self::new(scale * self.im.cos(), scale * self.im.sin())
    }

    #[inline]
    fn ln(self) -> Option<Self> {
        let magnitude = self.magnitude();
        if magnitude == 0.0 || !magnitude.is_finite() {
            None
        } else {
            Some(Self::new(magnitude.ln(), self.im.atan2(self.re)))
        }
    }

    #[inline]
    fn sqrt(self) -> Self {
        if self.im == 0.0 {
            if self.re >= 0.0 {
                return Self::new(self.re.sqrt(), 0.0);
            }
            return Self::new(0.0, (-self.re).sqrt());
        }
        let magnitude = self.magnitude();
        let re = ((magnitude + self.re) * 0.5).max(0.0).sqrt();
        let im = ((magnitude - self.re) * 0.5).max(0.0).sqrt().copysign(self.im);
        Self::new(re, im)
    }

    #[inline]
    fn sin(self) -> Self {
        Self::new(self.re.sin() * self.im.cosh(), self.re.cos() * self.im.sinh())
    }

    #[inline]
    fn cos(self) -> Self {
        Self::new(self.re.cos() * self.im.cosh(), -self.re.sin() * self.im.sinh())
    }

    #[inline]
    fn tan(self) -> Option<Self> {
        self.sin().div(self.cos())
    }

    #[inline]
    fn div(self, rhs: Self) -> Option<Self> {
        if rhs.re == 0.0 && rhs.im == 0.0 {
            return None;
        }
        let scale = rhs.re.abs().max(rhs.im.abs());
        if scale == 0.0 || !scale.is_finite() {
            return None;
        }
        let c = rhs.re / scale;
        let d = rhs.im / scale;
        let denominator = c * c + d * d;
        let a = self.re / scale;
        let b = self.im / scale;
        Some(Self::new((a * c + b * d) / denominator, (b * c - a * d) / denominator))
    }

    #[inline]
    fn pow(self, exponent: Self) -> Option<Self> {
        if self.re == 0.0 && self.im == 0.0 {
            if exponent.im == 0.0 && exponent.re > 0.0 {
                return Some(Self::new(0.0, 0.0));
            }
            return None;
        }
        let logarithm = self.ln()?;
        Some(Complex::new(
            exponent.re * logarithm.re - exponent.im * logarithm.im,
            exponent.re * logarithm.im + exponent.im * logarithm.re,
        ).exp())
    }
}

#[inline]
fn complex_key(re: f64, im: f64) -> (u64, u64) {
    let re = if re == 0.0 { 0.0 } else { re };
    let im = if im == 0.0 { 0.0 } else { im };
    (re.to_bits(), im.to_bits())
}

#[inline]
fn complex_display(re: f64, im: f64) -> String {
    if im == 0.0 {
        return format!("{re:.15}");
    }
    if re == 0.0 {
        return format!("{im:.15}i");
    }
    if im < 0.0 {
        format!("{re:.15} - {:.15}i", im.abs())
    } else {
        format!("{re:.15} + {im:.15}i")
    }
}

#[inline]
fn long_beam_rank(value: f64, target: f64) -> f64 {
    ((value.abs() + 1.0).ln() - (target.abs() + 1.0).ln()).abs()
}

#[inline]
fn long_beam_bucket(value: f64, imag: f64) -> i32 {
    if value == 0.0 && imag == 0.0 {
        return i32::MIN;
    }
    let magnitude = value.hypot(imag);
    let exponent = magnitude.log10().floor() as i32;
    if imag == 0.0 {
        exponent.saturating_mul(4) + if value.is_sign_negative() { 2 } else { 0 }
    } else {
        let quadrant = if value >= 0.0 && imag >= 0.0 { 0 }
            else if value < 0.0 && imag >= 0.0 { 1 }
            else if value < 0.0 && imag < 0.0 { 2 }
            else { 3 };
        exponent.saturating_mul(4) + quadrant
    }
}

fn insert_into_cell(
    cell: &mut Cell,
    nodes: &mut Vec<ExprNode>,
    target: f64,
    value: f64,
    node: ExprNode,
) -> Option<f64> {
    insert_into_cell_with_base(cell, nodes, target, value, node, 0)
}

fn insert_into_cell_with_base(
    cell: &mut Cell,
    nodes: &mut Vec<ExprNode>,
    target: f64,
    value: f64,
    node: ExprNode,
    node_base: usize,
) -> Option<f64> {
    insert_complex_into_cell_with_base(cell, nodes, target, value, 0.0, node, node_base)
}

fn insert_complex_into_cell(
    cell: &mut Cell,
    nodes: &mut Vec<ExprNode>,
    target: f64,
    value: Complex,
    node: ExprNode,
) -> Option<f64> {
    insert_complex_into_cell_with_base(cell, nodes, target, value.re, value.im, node, 0)
}

fn insert_complex_into_cell_with_base(
    cell: &mut Cell,
    nodes: &mut Vec<ExprNode>,
    target: f64,
    value: f64,
    imag: f64,
    node: ExprNode,
    node_base: usize,
) -> Option<f64> {
    if !value.is_finite() || !imag.is_finite() || value.hypot(imag) > MAX_ABS_VALUE {
        return None;
    }

    let value = if value == 0.0 { 0.0 } else { value };
    let imag = if imag == 0.0 { 0.0 } else { imag };
    let key = complex_key(value, imag);
    let score = (value - target).hypot(imag);
    let rank_score = if cell.diverse {
        let magnitude = value.hypot(imag);
        ((magnitude + 1.0).ln() - (target.abs() + 1.0).ln()).abs()
    } else {
        score
    };

    if !cell.diverse && cell.limit <= 16 {
        if cell.candidates.iter().any(|candidate| complex_key(candidate.value, candidate.imag) == key) {
            return None;
        }

        if cell.candidates.len() < cell.limit {
            let node_id = node_base + nodes.len();
            nodes.push(node);
            cell.candidates.push(Candidate { value, imag, node_id, score, rank_score });
            return Some(score);
        }

        let (index, worst) = cell.candidates
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.rank_score.total_cmp(&b.rank_score))?;
        if rank_score >= worst.rank_score {
            return None;
        }

        let local_node_id = cell.candidates[index].node_id.checked_sub(node_base)?;
        nodes[local_node_id] = node;
        let node_id = node_base + local_node_id;
        cell.candidates[index] = Candidate { value, imag, node_id, score, rank_score };
        return Some(score);
    }

    if cell.candidates.is_empty() {
        let reserve = cell.limit.min(32);
        cell.candidates.reserve(reserve);
        cell.seen.reserve(reserve.saturating_mul(2));
        cell.worst_heap.reserve(reserve.saturating_mul(2));
        cell.versions.reserve(reserve);
        if cell.diverse {
            cell.bucket_indices.reserve(reserve);
        }
    }

    if cell.seen.contains(&key) {
        return None;
    }

    if cell.diverse {
        let bucket = long_beam_bucket(value, imag);
        if let Some(index) = cell.bucket_indices.get(&bucket).copied() {
            if rank_score >= cell.candidates[index].rank_score {
                return None;
            }
            let old_key = complex_key(cell.candidates[index].value, cell.candidates[index].imag);
            let local_node_id = cell.candidates[index].node_id.checked_sub(node_base)?;
            nodes[local_node_id] = node;
            let node_id = node_base + local_node_id;
            cell.seen.remove(&old_key);
            cell.seen.insert(key);
            cell.candidates[index] = Candidate { value, imag, node_id, score, rank_score };
            cell.versions[index] = cell.versions[index].wrapping_add(1);
            cell.worst_heap.push((rank_score.to_bits(), index, cell.versions[index]));
            return Some(score);
        }
    }

    if cell.candidates.len() < cell.limit {
        let node_id = node_base + nodes.len();
        nodes.push(node);
        let index = cell.candidates.len();
        cell.seen.insert(key);
        cell.candidates.push(Candidate { value, imag, node_id, score, rank_score });
        cell.versions.push(0);
        if cell.diverse {
            cell.bucket_indices.insert(long_beam_bucket(value, imag), index);
        }
        cell.worst_heap.push((rank_score.to_bits(), index, 0));
        return Some(score);
    }

    let (worst_score, index) = cell.worst()?;
    if rank_score >= worst_score {
        return None;
    }

    let old_key = complex_key(cell.candidates[index].value, cell.candidates[index].imag);
    let local_node_id = cell.candidates[index].node_id.checked_sub(node_base)?;
    nodes[local_node_id] = node;
    let node_id = node_base + local_node_id;

    cell.seen.remove(&old_key);
    cell.seen.insert(key);
    if cell.diverse {
        cell.bucket_indices.remove(&long_beam_bucket(cell.candidates[index].value, cell.candidates[index].imag));
        cell.bucket_indices.insert(long_beam_bucket(value, imag), index);
    }
    cell.candidates[index] = Candidate { value, imag, node_id, score, rank_score };
    cell.versions[index] = cell.versions[index].wrapping_add(1);
    cell.worst_heap.push((rank_score.to_bits(), index, cell.versions[index]));
    Some(score)
}

struct Search {
    dp: Vec<Vec<Cell>>,
    nodes: Vec<ExprNode>,
    target: f64,
    tolerance: f64,
    full_mask: usize,
    found_solution: bool,
}

impl Search {
    fn new(
        size: usize,
        _depth: usize,
        target: f64,
        tolerance: f64,
        full_mask: usize,
        candidate_limit: usize,
    ) -> Self {
        let dp = (0..size).map(|_| Vec::new()).collect();
        let node_capacity = size.saturating_mul(candidate_limit.min(8));
        Self {
            dp,
            nodes: Vec::with_capacity(node_capacity),
            target,
            tolerance,
            full_mask,
            found_solution: false,
        }
    }

}

#[inline]
fn factorial(value: f64) -> Option<f64> {
    if value < 0.0 || value > 12.0 || value.fract() != 0.0 {
        return None;
    }

    let mut result = 1.0;
    for i in 2..=(value as u32) {
        result *= i as f64;
    }
    Some(result)
}

#[inline]
fn integer_argument(value: f64) -> Option<i64> {
    if value.is_finite()
        && value.fract() == 0.0
        && value.abs() <= MAX_INTEGER_ARGUMENT
    {
        Some(value as i64)
    } else {
        None
    }
}

#[inline]
fn gcd_i64(a: i64, b: i64) -> i64 {
    let mut x = a.abs();
    let mut y = b.abs();
    while y != 0 {
        let remainder = x % y;
        x = y;
        y = remainder;
    }
    x
}

#[inline]
fn gcd_value(a: f64, b: f64) -> Option<f64> {
    Some(gcd_i64(integer_argument(a)?, integer_argument(b)?) as f64)
}

#[inline]
fn lcm_value(a: f64, b: f64) -> Option<f64> {
    let a = integer_argument(a)?;
    let b = integer_argument(b)?;

    if a == 0 || b == 0 {
        return Some(0.0);
    }

    let gcd = gcd_i64(a, b);
    let result = (a / gcd).checked_mul(b)?;
    if (result as f64).abs() <= MAX_ABS_VALUE {
        Some((result as f64).abs())
    } else {
        None
    }
}

#[inline]
fn permutations(n: f64, r: f64) -> Option<f64> {
    let n = integer_argument(n)?;
    let r = integer_argument(r)?;
    if n < 0 || n > 20 || r < 0 || r > n {
        return None;
    }

    let mut result = 1.0;
    for i in 0..r {
        result *= (n - i) as f64;
    }
    Some(result)
}

#[inline]
fn combinations(n: f64, r: f64) -> Option<f64> {
    let n = integer_argument(n)?;
    let r = integer_argument(r)?;
    if n < 0 || n > 20 || r < 0 || r > n {
        return None;
    }

    let k = r.min(n - r);
    let mut result = 1.0;
    for i in 1..=k {
        result = result * (n - k + i) as f64 / i as f64;
    }
    Some(result)
}

#[inline]
fn double_factorial(value: f64) -> Option<f64> {
    if value.fract() != 0.0 || value < -1.0 || value > 100.0 {
        return None;
    }
    if value == -1.0 || value == 0.0 {
        return Some(1.0);
    }

    let mut n = value as i64;
    let mut result = 1.0;
    while n > 1 {
        result *= n as f64;
        n -= 2;
    }
    Some(result)
}

#[inline]
fn primorial(value: f64) -> Option<f64> {
    if value.fract() != 0.0 || !(0.0..=100.0).contains(&value) {
        return None;
    }

    let limit = value as u32;
    let mut result = 1.0;
    for candidate in 2..=limit {
        let mut prime = true;
        let mut divisor = 2;
        while divisor * divisor <= candidate {
            if candidate % divisor == 0 {
                prime = false;
                break;
            }
            divisor += 1;
        }
        if prime {
            result *= candidate as f64;
            if result > MAX_ABS_VALUE {
                return None;
            }
        }
    }
    Some(result)
}

#[inline]
fn digit_sum(value: f64) -> Option<f64> {
    let mut n = integer_argument(value)?.abs();
    let mut sum = 0_i64;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    Some(sum as f64)
}

#[inline]
fn digital_root(value: f64) -> Option<f64> {
    let n = integer_argument(value)?.abs();
    if n == 0 {
        Some(0.0)
    } else {
        Some((1 + (n - 1) % 9) as f64)
    }
}

#[inline]
fn gamma_function(value: f64) -> Option<f64> {
    const COEFFICIENTS: [f64; 9] = [
        0.99999999999980993,
        676.5203681218851,
        -1259.1392167224028,
        771.32342877765313,
        -176.61502916214059,
        12.507343278686905,
        -0.13857109526572012,
        9.9843695780195716e-6,
        1.5056327351493116e-7,
    ];

    if !value.is_finite()
        || value < -50.0
        || value > 100.0
        || (value <= 0.0 && value.fract() == 0.0)
    {
        return None;
    }

    let result = if value < 0.5 {
        let reflected = gamma_function(1.0 - value)?;
        std::f64::consts::PI / ((std::f64::consts::PI * value).sin() * reflected)
    } else {
        let z = value - 1.0;
        let mut x = COEFFICIENTS[0];
        for (i, coefficient) in COEFFICIENTS.iter().enumerate().skip(1) {
            x += coefficient / (z + i as f64);
        }
        let t = z + 7.5;
        (2.0 * std::f64::consts::PI).sqrt()
            * t.powf(z + 0.5)
            * (-t).exp()
            * x
    };

    result.is_finite().then_some(result)
}

#[inline]
fn polevl(x: f64, coefficients: &[f64]) -> f64 {
    coefficients[1..]
        .iter()
        .fold(coefficients[0], |result, coefficient| result * x + coefficient)
}

#[inline]
fn p1evl(x: f64, coefficients: &[f64]) -> f64 {
    coefficients[1..]
        .iter()
        .fold(x + coefficients[0], |result, coefficient| result * x + coefficient)
}

#[inline]
fn erfc_positive(x: f64) -> f64 {
    const P: [f64; 9] = [
        2.46196981473530512524e-10,
        5.64189564831068821977e-1,
        7.46321056442269912687,
        4.86371970985681366614e1,
        1.96520832956077098242e2,
        5.26445194995477358631e2,
        9.34528527171957607540e2,
        1.02755188689515710272e3,
        5.57535335369399327526e2,
    ];
    const Q: [f64; 8] = [
        1.32281951154744992508e1,
        8.67072140885989742329e1,
        3.54937778887819891062e2,
        9.75708501743205489753e2,
        1.82390916687909736289e3,
        2.24633760818710981792e3,
        1.65666309194161350182e3,
        5.57535340817727675546e2,
    ];
    const R: [f64; 6] = [
        5.64189583547755073984e-1,
        1.27536670759978104416,
        5.01905042251180477414,
        6.16021097993053585195,
        7.40974269950448939160,
        2.97886665372100240670,
    ];
    const S: [f64; 6] = [
        2.26052863220117276590,
        9.39603524938001434673,
        1.20489539808096656605e1,
        1.70814450747565897222e1,
        9.60896809063285878198,
        3.36907645100081516050,
    ];
    if x < 1.0 {
        return 1.0 - erf_approx(x);
    }
    let ratio = if x < 8.0 {
        polevl(x, &P) / p1evl(x, &Q)
    } else {
        polevl(x, &R) / p1evl(x, &S)
    };
    (-x * x).exp() * ratio
}

#[inline]
fn erf_approx(value: f64) -> f64 {
    const T: [f64; 5] = [
        9.60497373987051638749,
        9.00260197203842689217e1,
        2.23200534594684319226e3,
        7.00332514112805075473e3,
        5.55923013010394962768e4,
    ];
    const U: [f64; 5] = [
        3.35617141647503099647e1,
        5.21357949780152679795e2,
        4.59432382970980127987e3,
        2.26290000613890934246e4,
        4.92673942608635921086e4,
    ];
    if value == 0.0 {
        return 0.0;
    }
    let x = value.abs();
    let result = if x <= 1.0 {
        let z = x * x;
        x * polevl(z, &T) / p1evl(z, &U)
    } else {
        1.0 - erfc_positive(x)
    };
    if value < 0.0 { -result } else { result }
}

#[inline]
fn erfc_approx(value: f64) -> f64 {
    if value < 0.0 {
        2.0 - erfc_positive(-value)
    } else {
        erfc_positive(value)
    }
}

#[inline]
fn fibonacci_value(value: f64) -> Option<f64> {
    if value.fract() != 0.0 || !(-78.0..=78.0).contains(&value) {
        return None;
    }
    let n = value as i32;
    let mut a = 0.0_f64;
    let mut b = 1.0_f64;
    for _ in 0..n.unsigned_abs() {
        let next = a + b;
        a = b;
        b = next;
    }
    if n < 0 && n.unsigned_abs() % 2 == 0 { Some(-a) } else { Some(a) }
}

#[inline]
fn triangular_value(value: f64) -> Option<f64> {
    if value.fract() != 0.0 || !(0.0..=1_000_000.0).contains(&value) {
        return None;
    }
    let n = value;
    Some(n * (n + 1.0) / 2.0)
}

#[inline]
fn catalan_value(value: f64) -> Option<f64> {
    if value.fract() != 0.0 || !(0.0..=23.0).contains(&value) {
        return None;
    }
    let n = value as u32;
    let mut result = 1.0_f64;
    for k in 0..n {
        result *= 2.0 * (2 * k + 1) as f64 / (k + 2) as f64;
    }
    Some(result)
}

#[derive(Clone, Copy)]
struct IntegerFacts {
    is_prime: f64,
    totient: f64,
    divisor_count: f64,
    prime_omega: f64,
    mobius: f64,
    is_perfect: f64,
    radical: f64,
    divisor_sum: f64,
    liouville: f64,
    carmichael_lambda: f64,
}

#[inline]
fn integer_facts(value: f64) -> Option<IntegerFacts> {
    if value.fract() != 0.0 || value < 0.0 || value > 10_000.0 {
        return None;
    }
    let original = value as u64;
    if original == 0 {
        return Some(IntegerFacts {
            is_prime: 0.0,
            totient: f64::NAN,
            divisor_count: f64::NAN,
            prime_omega: f64::NAN,
            mobius: f64::NAN,
            is_perfect: 0.0,
            radical: 0.0,
            divisor_sum: 0.0,
            liouville: 0.0,
            carmichael_lambda: f64::NAN,
        });
    }

    let mut remaining = original;
    let mut phi = original;
    let mut divisors = 1_u64;
    let mut omega = 0_u64;
    let mut mu = 1_i64;
    let mut prime = true;
    let mut radical = 1_u64;
    let mut sigma = 1_u64;
    let mut lambda = 1_u64;
    let mut factor = 2_u64;

    while factor <= remaining / factor {
        if remaining % factor == 0 {
            prime = false;
            phi -= phi / factor;
            radical *= factor;
            let mut exponent = 0_u64;
            let mut prime_power = 1_u64;
            let mut sigma_term = 1_u64;
            while remaining % factor == 0 {
                remaining /= factor;
                exponent += 1;
                prime_power *= factor;
                sigma_term += prime_power;
            }
            divisors *= exponent + 1;
            omega += exponent;
            sigma *= sigma_term;
            if exponent > 1 {
                mu = 0;
            } else if mu != 0 {
                mu = -mu;
            }
            let prime_lambda = if factor == 2 && exponent >= 3 {
                prime_power / 4
            } else {
                ((factor - 1) * prime_power) / factor
            };
            let common = gcd_i64(lambda as i64, prime_lambda as i64) as u64;
            lambda = (lambda / common) * prime_lambda;
        }
        factor = if factor == 2 { 3 } else { factor + 2 };
    }

    if remaining > 1 {
        prime = original == remaining;
        phi -= phi / remaining;
        divisors *= 2;
        omega += 1;
        radical *= remaining;
        sigma *= remaining + 1;
        let common = gcd_i64(lambda as i64, (remaining - 1) as i64) as u64;
        lambda = (lambda / common) * (remaining - 1);
        if mu != 0 {
            mu = -mu;
        }
    }

    Some(IntegerFacts {
        is_prime: if original >= 2 && prime { 1.0 } else { 0.0 },
        totient: phi as f64,
        divisor_count: divisors as f64,
        prime_omega: omega as f64,
        mobius: mu as f64,
        is_perfect: if sigma - original == original { 1.0 } else { 0.0 },
        radical: radical as f64,
        divisor_sum: sigma as f64,
        liouville: if omega % 2 == 0 { 1.0 } else { -1.0 },
        carmichael_lambda: lambda as f64,
    })
}

#[inline]
fn digit_product(value: f64) -> Option<f64> {
    let mut n = integer_argument(value)?.unsigned_abs();
    let mut product = 1_u64;
    if n == 0 {
        return Some(0.0);
    }
    while n > 0 {
        product = product.saturating_mul(n % 10);
        n /= 10;
        if product as f64 > MAX_ABS_VALUE {
            return None;
        }
    }
    Some(product as f64)
}

#[inline]
fn alternating_digit_sum(value: f64) -> Option<f64> {
    let mut n = integer_argument(value)?.unsigned_abs();
    let mut sum = 0_i64;
    let mut sign = 1_i64;
    while n > 0 {
        sum += sign * (n % 10) as i64;
        sign = -sign;
        n /= 10;
    }
    Some(sum as f64)
}

#[inline]
fn harmonic_number(value: f64) -> Option<f64> {
    if value.fract() != 0.0 || !(0.0..=100.0).contains(&value) {
        return None;
    }
    let mut result = 0.0;
    for n in 1..=value as u32 {
        result += 1.0 / n as f64;
    }
    Some(result)
}

#[inline]
fn derangement_value(value: f64) -> Option<f64> {
    if value.fract() != 0.0 || !(0.0..=18.0).contains(&value) {
        return None;
    }
    let n = value as usize;
    if n == 0 {
        return Some(1.0);
    }
    let mut previous_previous = 1.0;
    let mut previous = 0.0;
    for k in 2..=n {
        let current = (k - 1) as f64 * (previous + previous_previous);
        previous_previous = previous;
        previous = current;
    }
    Some(previous)
}

#[inline]
fn partition_number(value: f64) -> Option<f64> {
    if value.fract() != 0.0 || !(0.0..=50.0).contains(&value) {
        return None;
    }
    let n = value as usize;
    let mut p = [0.0_f64; 51];
    p[0] = 1.0;
    for i in 1..=n {
        let mut total = 0.0;
        let mut k = 1usize;
        loop {
            let g1 = k * (3 * k - 1) / 2;
            if g1 > i {
                break;
            }
            let sign = if k % 2 == 1 { 1.0 } else { -1.0 };
            total += sign * p[i - g1];
            let g2 = k * (3 * k + 1) / 2;
            if g2 <= i {
                total += sign * p[i - g2];
            }
            k += 1;
        }
        p[i] = total;
    }
    Some(p[n])
}

#[inline]
fn bell_number(value: f64) -> Option<f64> {
    if value.fract() != 0.0 || !(0.0..=15.0).contains(&value) {
        return None;
    }
    let n = value as usize;
    let mut previous = [0.0_f64; 16];
    previous[0] = 1.0;
    for row_index in 1..=n {
        let mut current = [0.0_f64; 16];
        current[0] = previous[row_index - 1];
        for column in 1..=row_index {
            current[column] = current[column - 1] + previous[column - 1];
        }
        previous = current;
    }
    Some(previous[0])
}

#[inline]
fn reverse_digits(value: f64) -> Option<f64> {
    let original = integer_argument(value)?;
    let sign = if original < 0 { -1.0 } else { 1.0 };
    let mut n = original.abs() as u64;
    let mut reversed = 0_u64;
    while n > 0 {
        reversed = reversed.saturating_mul(10).saturating_add(n % 10);
        n /= 10;
        if reversed as f64 > MAX_ABS_VALUE {
            return None;
        }
    }
    Some(sign * reversed as f64)
}

#[inline]
fn digit_count(value: f64) -> Option<f64> {
    let mut n = integer_argument(value)?.unsigned_abs();
    let mut count = 1_u32;
    while n >= 10 {
        n /= 10;
        count += 1;
    }
    Some(count as f64)
}

#[inline]
fn is_perfect_square(value: f64) -> Option<f64> {
    let n = integer_argument(value)?;
    if n < 0 {
        return Some(0.0);
    }
    let root = (n as f64).sqrt().round() as i64;
    Some(if root * root == n { 1.0 } else { 0.0 })
}

#[inline]
fn lambert_w0(value: f64) -> Option<f64> {
    let branch_point = -1.0 / std::f64::consts::E;
    if !value.is_finite() || value < branch_point || value > MAX_ABS_VALUE {
        return None;
    }
    if value == 0.0 {
        return Some(0.0);
    }
    if (value - branch_point).abs() <= 1.0e-15 {
        return Some(-1.0);
    }
    let mut w = if value < -0.2 {
        -1.0 + (2.0 * (std::f64::consts::E * value + 1.0)).sqrt()
    } else if value < 1.0 {
        value
    } else {
        value.ln() - value.ln().ln().max(0.0)
    };
    for _ in 0..10 {
        let ew = w.exp();
        let f = w * ew - value;
        let denominator = ew * (w + 1.0) - (w + 2.0) * f / (2.0 * w + 2.0);
        if denominator == 0.0 || !denominator.is_finite() {
            break;
        }
        let delta = f / denominator;
        w -= delta;
        if delta.abs() <= 1.0e-14 * w.abs().max(1.0) {
            break;
        }
    }
    w.is_finite().then_some(w)
}

#[inline]
fn next_after_value(a: f64, b: f64) -> f64 {
    if a == b {
        return b;
    }
    if a == 0.0 {
        return f64::from_bits(1).copysign(b);
    }
    let bits = a.to_bits();
    let moves_up_in_bits = (b > a) == (a > 0.0);
    f64::from_bits(if moves_up_in_bits { bits + 1 } else { bits - 1 })
}

#[inline]
fn extended_gcd_values(a: f64, b: f64) -> Option<(f64, f64, f64)> {
    let a = integer_argument(a)? as i128;
    let b = integer_argument(b)? as i128;
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1_i128, 0_i128);
    let (mut old_t, mut t) = (0_i128, 1_i128);
    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
        (old_t, t) = (t, old_t - q * t);
    }
    if old_r < 0 { old_r = -old_r; old_s = -old_s; old_t = -old_t; }
    Some((old_r as f64, old_s as f64, old_t as f64))
}

fn modular_inverse_value(a: f64, modulus: f64) -> Option<f64> {
    let a = integer_argument(a)? as i128;
    let modulus = integer_argument(modulus)? as i128;
    if modulus <= 1 || modulus > 1_000_000 {
        return None;
    }
    let mut old_r = a.rem_euclid(modulus);
    let mut r = modulus;
    let mut old_s = 1_i128;
    let mut s = 0_i128;
    while r != 0 {
        let quotient = old_r / r;
        (old_r, r) = (r, old_r - quotient * r);
        (old_s, s) = (s, old_s - quotient * s);
    }
    if old_r != 1 {
        return None;
    }
    Some(old_s.rem_euclid(modulus) as f64)
}

#[inline]
fn is_power_of_value(value: f64, base: f64) -> Option<f64> {
    let mut n = integer_argument(value)?;
    let base = integer_argument(base)?;
    if n < 1 || base < 2 {
        return Some(0.0);
    }
    while n % base == 0 {
        n /= base;
    }
    Some(if n == 1 { 1.0 } else { 0.0 })
}

#[inline]
fn logistic(value: f64) -> f64 {
    if value >= 0.0 {
        1.0 / (1.0 + (-value).exp())
    } else {
        let e = value.exp();
        e / (1.0 + e)
    }
}

#[inline]
fn gamma_cached(value: f64, cache: &mut HashMap<u64, Option<f64>>) -> Option<f64> {
    let key = value.to_bits();
    if cache.len() >= 32_768 && !cache.contains_key(&key) {
        cache.clear();
    }
    *cache.entry(key).or_insert_with(|| gamma_function(value))
}

fn beta_value(a: f64, b: f64) -> Option<f64> {
    if a <= 0.0 || b <= 0.0 || a > 50.0 || b > 50.0 || a + b > 100.0 {
        return None;
    }
    let numerator = gamma_function(a)? * gamma_function(b)?;
    let denominator = gamma_function(a + b)?;
    let result = numerator / denominator;
    result.is_finite().then_some(result)
}

fn beta_value_cached(
    a: f64,
    b: f64,
    cache: &mut HashMap<u64, Option<f64>>,
) -> Option<f64> {
    if a <= 0.0 || b <= 0.0 || a > 50.0 || b > 50.0 || a + b > 100.0 {
        return None;
    }
    let numerator = gamma_cached(a, cache)? * gamma_cached(b, cache)?;
    let denominator = gamma_cached(a + b, cache)?;
    let result = numerator / denominator;
    result.is_finite().then_some(result)
}

#[inline]
fn logarithmic_mean(a: f64, b: f64) -> Option<f64> {
    if a <= 0.0 || b <= 0.0 {
        return None;
    }
    if a == b || (a - b).abs() <= f64::EPSILON * a.max(b) {
        return Some((a + b) / 2.0);
    }
    let result = (a - b) / (a.ln() - b.ln());
    result.is_finite().then_some(result)
}

#[inline]
fn arithmetic_geometric_mean(a: f64, b: f64) -> Option<f64> {
    if a < 0.0 || b < 0.0 {
        return None;
    }
    if a == 0.0 || b == 0.0 {
        return Some(0.0);
    }
    if a == b {
        return Some(a);
    }
    let mut arithmetic = (a + b) / 2.0;
    let mut geometric = (a * b).sqrt();
    for _ in 0..32 {
        if (arithmetic - geometric).abs() <= 1.0e-14 * arithmetic.max(1.0) {
            break;
        }
        let next_arithmetic = (arithmetic + geometric) / 2.0;
        let next_geometric = (arithmetic * geometric).sqrt();
        arithmetic = next_arithmetic;
        geometric = next_geometric;
    }
    arithmetic.is_finite().then_some((arithmetic + geometric) / 2.0)
}

#[inline]
fn polygonal_value(n: f64, sides: f64) -> Option<f64> {
    let n = integer_argument(n)?;
    let sides = integer_argument(sides)?;
    if n < 0 || n > 1_000_000 || !(3..=1_000).contains(&sides) {
        return None;
    }
    let n = n as f64;
    let sides = sides as f64;
    Some(((sides - 2.0) * n * n - (sides - 4.0) * n) / 2.0)
}

#[inline]
fn nth_root(value: f64, degree: f64) -> Option<f64> {
    if degree.fract() != 0.0 || degree == 0.0 || degree.abs() > 12.0 {
        return None;
    }

    let n = degree.abs();
    if value < 0.0 && (n as i32) % 2 == 0 {
        return None;
    }

    let root = if value < 0.0 {
        -(-value).powf(1.0 / n)
    } else {
        value.powf(1.0 / n)
    };

    if degree < 0.0 {
        if root == 0.0 {
            None
        } else {
            Some(1.0 / root)
        }
    } else {
        Some(root)
    }
}

const EULER_MASCHERONI: f64 = 0.5772156649015329;

#[inline]
fn is_prime_u64(n: u64) -> bool {
    if n < 2 { return false; }

    const SMALL_PRIMES: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for prime in SMALL_PRIMES {
        if n == prime { return true; }
        if n % prime == 0 { return false; }
    }

    let mut d = n - 1;
    let s = d.trailing_zeros();
    d >>= s;

    const BASES: [u64; 7] = [2, 325, 9_375, 28_178, 450_775, 9_780_504, 1_795_265_022];
    'witness: for base in BASES {
        let a = base % n;
        if a == 0 { continue; }
        let mut x = match modular_pow_u64(a, d, n) {
            Some(value) => value,
            None => return false,
        };
        if x == 1 || x == n - 1 { continue; }

        for _ in 1..s {
            x = ((x as u128 * x as u128) % n as u128) as u64;
            if x == n - 1 { continue 'witness; }
        }
        return false;
    }
    true
}

fn prime_table() -> &'static Vec<u32> {
    static TABLE: OnceLock<Vec<u32>> = OnceLock::new();
    TABLE.get_or_init(|| {
        const LIMIT: usize = 2_000_000;
        let mut composite = vec![false; LIMIT + 1];
        let mut primes = Vec::with_capacity(150_000);
        for p in 2..=LIMIT {
            if !composite[p] {
                primes.push(p as u32);
                if p <= LIMIT / p {
                    let mut multiple = p * p;
                    while multiple <= LIMIT {
                        composite[multiple] = true;
                        multiple += p;
                    }
                }
            }
        }
        primes
    })
}

fn prime_count_value(value: f64) -> Option<f64> {
    let n = integer_argument(value)?;
    if !(0..=2_000_000).contains(&n) { return None; }
    let primes = prime_table();
    Some(primes.partition_point(|&p| p as i64 <= n) as f64)
}

fn nth_prime_value(value: f64) -> Option<f64> {
    let n = integer_argument(value)?;
    if !(1..=100_000).contains(&n) { return None; }
    Some(prime_table()[(n - 1) as usize] as f64)
}

fn next_prime_value(value: f64) -> Option<f64> {
    let n = integer_argument(value)?;
    if n >= 1_000_000 { return None; }
    if n < 2_000_000 {
        let primes = prime_table();
        let index = primes.partition_point(|&prime| prime as i64 <= n);
        if let Some(&prime) = primes.get(index) {
            return Some(prime as f64);
        }
    }

    let mut candidate = (n + 1).max(2) as u64;
    if candidate > 2 && candidate % 2 == 0 { candidate += 1; }
    while candidate <= 1_000_000_007 {
        if is_prime_u64(candidate) { return Some(candidate as f64); }
        candidate += if candidate == 2 { 1 } else { 2 };
    }
    None
}

fn is_squarefree_value(value: f64) -> Option<f64> {
    let n = integer_argument(value)?;
    if n <= 0 { return Some(0.0); }
    if n > 1_000_000 { return None; }
    let n = n as u64;
    for &prime in prime_table().iter() {
        let p = prime as u64;
        if p > n / p { break; }
        let square = p * p;
        if n % square == 0 { return Some(0.0); }
    }
    Some(1.0)
}

fn modular_pow_u64(mut base: u64, mut exponent: u64, modulus: u64) -> Option<u64> {
    if modulus == 0 { return None; }
    base %= modulus;
    let mut result = 1_u64 % modulus;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = ((result as u128 * base as u128) % modulus as u128) as u64;
        }
        base = ((base as u128 * base as u128) % modulus as u128) as u64;
        exponent >>= 1;
    }
    Some(result)
}

fn multiplicative_order_value(a: f64, modulus: f64) -> Option<f64> {
    let a = integer_argument(a)?;
    let m = integer_argument(modulus)?;
    if m < 2 || m > 4_096 || gcd_i64(a, m) != 1 { return None; }
    let m = m as u64;
    let base = a.rem_euclid(m as i64) as u64;
    let mut value = 1_u64;
    for k in 1..=m {
        value = ((value as u128 * base as u128) % m as u128) as u64;
        if value == 1 { return Some(k as f64); }
    }
    None
}

fn sigma_k_value(n: f64, k: f64) -> Option<f64> {
    let n = integer_argument(n)?;
    let k = integer_argument(k)?;
    if n < 1 || n > 100_000 || !(0..=10).contains(&k) { return None; }
    let mut sum = 0.0;
    let mut d = 1_i64;
    while d <= n / d {
        if n % d == 0 {
            sum += (d as f64).powi(k as i32);
            let paired = n / d;
            if paired != d { sum += (paired as f64).powi(k as i32); }
        }
        d += 1;
    }
    sum.is_finite().then_some(sum)
}

fn jordan_totient_value(n: f64, k: f64) -> Option<f64> {
    let n = integer_argument(n)?;
    let k = integer_argument(k)?;
    if n < 1 || n > 1_000_000 || !(1..=10).contains(&k) { return None; }
    let mut remaining = n as u64;
    let mut result = (n as f64).powi(k as i32);
    let mut p = 2_u64;
    while p <= remaining / p {
        if remaining % p == 0 {
            result *= 1.0 - 1.0 / (p as f64).powi(k as i32);
            while remaining % p == 0 { remaining /= p; }
        }
        p += if p == 2 { 1 } else { 2 };
    }
    if remaining > 1 {
        result *= 1.0 - 1.0 / (remaining as f64).powi(k as i32);
    }
    result.is_finite().then_some(result)
}

fn aliquot_sum_value(value: f64) -> Option<f64> {
    let n = integer_argument(value)?;
    if n < 1 { return None; }
    Some(sigma_k_value(n as f64, 1.0)? - n as f64)
}

fn is_carmichael_value(value: f64) -> Option<f64> {
    let n = integer_argument(value)?;
    if n < 3 { return Some(0.0); }
    if n > 1_000_000 { return None; }
    let mut remaining = n as u64;
    let mut p = 2_u64;
    let mut factors = 0_u32;
    while p <= remaining / p {
        if remaining % p == 0 {
            factors += 1;
            if remaining % (p * p) == 0 { return Some(0.0); }
            if (n - 1) % (p as i64 - 1) != 0 { return Some(0.0); }
            while remaining % p == 0 { remaining /= p; }
        }
        p += if p == 2 { 1 } else { 2 };
    }
    if remaining > 1 {
        factors += 1;
        if (n - 1) % (remaining as i64 - 1) != 0 { return Some(0.0); }
    }
    Some(if factors >= 2 { 1.0 } else { 0.0 })
}

fn lucas_value(value: f64) -> Option<f64> {
    let n = integer_argument(value)?;
    if !(0..=78).contains(&n) { return None; }
    let (mut a, mut b) = (2.0, 1.0);
    for _ in 0..n { (a, b) = (b, a + b); }
    Some(a)
}

fn pell_value(value: f64) -> Option<f64> {
    let n = integer_argument(value)?;
    if !(0..=78).contains(&n) { return None; }
    let (mut a, mut b) = (0.0, 1.0);
    for _ in 0..n { (a, b) = (b, 2.0 * b + a); }
    Some(a)
}

fn tribonacci_value(value: f64) -> Option<f64> {
    let n = integer_argument(value)?;
    if !(0..=70).contains(&n) { return None; }
    let (mut a, mut b, mut c) = (0.0, 0.0, 1.0);
    for _ in 0..n { (a, b, c) = (b, c, a + b + c); }
    Some(a)
}

fn bernoulli_value(value: f64) -> Option<f64> {
    let n = integer_argument(value)?;
    if !(0..=30).contains(&n) { return None; }
    if n == 1 { return Some(-0.5); }
    let n = n as usize;
    let mut a = vec![0.0_f64; n + 1];
    for m in 0..=n {
        a[m] = 1.0 / (m + 1) as f64;
        for j in (1..=m).rev() { a[j - 1] = j as f64 * (a[j - 1] - a[j]); }
    }
    Some(a[0])
}

fn euler_number_value(value: f64) -> Option<f64> {
    let n = integer_argument(value)?;
    if !(0..=28).contains(&n) { return None; }
    if n % 2 == 1 { return Some(0.0); }
    let n = n as usize;
    let mut e = vec![0.0_f64; n + 1];
    e[0] = 1.0;
    for m in (2..=n).step_by(2) {
        let mut sum = 0.0;
        for k in (0..m).step_by(2) {
            let mut binom = 1.0;
            for j in 0..k { binom *= (m - j) as f64 / (j + 1) as f64; }
            sum += binom * e[k];
        }
        e[m] = -sum;
    }
    Some(e[n])
}

fn stirling_first_value(n: f64, k: f64) -> Option<f64> {
    let n = integer_argument(n)?;
    let k = integer_argument(k)?;
    if n < 0 || n > 25 || k < 0 || k > n { return Some(0.0); }
    let mut row = vec![0.0_f64; k as usize + 1];
    row[0] = 1.0;
    for i in 1..=n as usize {
        let upper = (i).min(k as usize);
        for j in (1..=upper).rev() {
            row[j] = row[j - 1] - (i as f64 - 1.0) * row[j];
        }
        row[0] = 0.0;
    }
    Some(row[k as usize])
}

fn stirling_second_value(n: f64, k: f64) -> Option<f64> {
    let n = integer_argument(n)?;
    let k = integer_argument(k)?;
    if n < 0 || n > 25 || k < 0 || k > n { return Some(0.0); }
    let mut row = vec![0.0_f64; k as usize + 1];
    row[0] = 1.0;
    for i in 1..=n as usize {
        let upper = i.min(k as usize);
        for j in (1..=upper).rev() {
            row[j] = j as f64 * row[j] + row[j - 1];
        }
        row[0] = 0.0;
    }
    Some(row[k as usize])
}

fn eulerian_value(n: f64, k: f64) -> Option<f64> {
    let n = integer_argument(n)?;
    let k = integer_argument(k)?;
    if n < 0 || n > 25 || k < 0 || k >= n.max(1) { return Some(0.0); }
    let mut row = vec![0.0_f64; k as usize + 1];
    row[0] = 1.0;
    for i in 1..=n as usize {
        let upper = (i - 1).min(k as usize);
        for j in (0..=upper).rev() {
            let left = if j > 0 { (i - j) as f64 * row[j - 1] } else { 0.0 };
            let right = (j + 1) as f64 * row[j];
            row[j] = left + right;
        }
    }
    Some(row[k as usize])
}

fn digamma_positive(mut x: f64) -> Option<f64> {
    if !x.is_finite() || x <= 0.0 || x > 1.0e8 { return None; }
    let mut result = 0.0;
    while x < 8.0 { result -= 1.0 / x; x += 1.0; }
    let inv = 1.0 / x;
    let inv2 = inv * inv;
    result += x.ln() - 0.5 * inv
        - inv2 * (1.0 / 12.0 - inv2 * (1.0 / 120.0 - inv2 * (1.0 / 252.0 - inv2 * (1.0 / 240.0 - inv2 * 5.0 / 660.0))));
    result.is_finite().then_some(result)
}

fn trigamma_positive(mut x: f64) -> Option<f64> {
    if !x.is_finite() || x <= 0.0 || x > 1.0e8 { return None; }
    let mut result = 0.0;
    while x < 8.0 { result += 1.0 / (x * x); x += 1.0; }
    let inv = 1.0 / x;
    let inv2 = inv * inv;
    result += inv + 0.5 * inv2 + inv * inv2 * (1.0 / 6.0 - inv2 * (1.0 / 30.0 - inv2 * (1.0 / 42.0 - inv2 * (1.0 / 30.0 - inv2 * 5.0 / 66.0))));
    result.is_finite().then_some(result)
}

fn zeta_value(s: f64) -> Option<f64> {
    if !s.is_finite() || s < 0.0 || (s - 1.0).abs() < 1.0e-12 || s > 100.0 { return None; }
    if s == 0.0 { return Some(-0.5); }
    const N: usize = 32;
    let mut sum = 0.0;
    for n in 1..N { sum += (n as f64).powf(-s); }
    let nf = N as f64;
    sum += nf.powf(1.0 - s) / (s - 1.0) + 0.5 * nf.powf(-s);
    let coeffs = [
        (1.0 / 12.0, 1usize),
        (-1.0 / 720.0, 3),
        (1.0 / 30240.0, 5),
        (-1.0 / 1209600.0, 7),
        (1.0 / 47900160.0, 9),
        (-691.0 / 1307674368000.0, 11),
    ];
    for (coefficient, rising_len) in coeffs {
        let mut rising = 1.0;
        for j in 0..rising_len { rising *= s + j as f64; }
        sum += coefficient * rising * nf.powf(-s - rising_len as f64);
    }
    sum.is_finite().then_some(sum)
}

fn eta_value(s: f64) -> Option<f64> {
    if !s.is_finite() || s < 0.0 || s > 100.0 { return None; }
    if s == 0.0 { return Some(0.5); }
    if (s - 1.0).abs() < 1.0e-12 { return Some(std::f64::consts::LN_2); }
    let zeta = zeta_value(s)?;
    let result = (1.0 - 2.0_f64.powf(1.0 - s)) * zeta;
    result.is_finite().then_some(result)
}

fn polylog_value(s: f64, x: f64) -> Option<f64> {
    if !s.is_finite() || !x.is_finite() || s.abs() > 20.0 || x.abs() > 1.0 { return None; }
    if x == 0.0 { return Some(0.0); }
    if x == 1.0 { return zeta_value(s); }
    if x == -1.0 { return eta_value(s).map(|v| -v); }
    if x.abs() > 0.9 { return None; }
    let mut sum = 0.0;
    let mut power = x;
    for n in 1..=512 {
        let term = power / (n as f64).powf(s);
        sum += term;
        if term.abs() < 1.0e-14 * sum.abs().max(1.0) { break; }
        power *= x;
    }
    sum.is_finite().then_some(sum)
}

fn erfi_value(x: f64) -> Option<f64> {
    if !x.is_finite() || x.abs() > 5.0 { return None; }
    let xx = x * x;
    let mut term = x;
    let mut sum = term;
    for n in 1..=100 {
        term *= xx / n as f64;
        let add = term / (2.0 * n as f64 + 1.0);
        sum += add;
        if add.abs() < 1.0e-15 * sum.abs().max(1.0) { break; }
    }
    (2.0 / std::f64::consts::PI.sqrt() * sum).is_finite().then_some(2.0 / std::f64::consts::PI.sqrt() * sum)
}

fn erfcx_value(x: f64) -> Option<f64> {
    if !x.is_finite() || x < -5.0 || x > 26.0 { return None; }
    let result = if x >= 0.0 { (x * x).exp() * erfc_approx(x) } else { 2.0 * (x * x).exp() - erfcx_value(-x)? };
    result.is_finite().then_some(result)
}

fn sine_integral_value(x: f64) -> Option<f64> {
    if !x.is_finite() || x.abs() > 20.0 { return None; }
    let mut sum = 0.0;
    let mut factorial = 1.0;
    let mut power = x;
    for k in 0..80 {
        if k > 0 { factorial *= (2 * k) as f64 * (2 * k + 1) as f64; power *= -x * x; }
        let term = power / (factorial * (2 * k + 1) as f64);
        sum += term;
        if term.abs() < 1.0e-15 * sum.abs().max(1.0) { break; }
    }
    sum.is_finite().then_some(sum)
}

fn cosine_integral_value(x: f64) -> Option<f64> {
    if !x.is_finite() || x == 0.0 || x.abs() > 20.0 { return None; }
    let ax = x.abs();
    let mut sum = EULER_MASCHERONI + ax.ln();
    let mut factorial = 1.0;
    let mut power = 1.0;
    for k in 1..80 {
        factorial *= (2 * k - 1) as f64 * (2 * k) as f64;
        power *= -ax * ax;
        let term = power / (factorial * (2 * k) as f64);
        sum += term;
        if term.abs() < 1.0e-15 * sum.abs().max(1.0) { break; }
    }
    sum.is_finite().then_some(sum)
}

fn bessel_j0_value(x: f64) -> Option<f64> {
    if !x.is_finite() || x.abs() > 30.0 { return None; }
    let q = x * x / 4.0;
    let mut term = 1.0;
    let mut sum = 1.0;
    for k in 1..=100 {
        term *= -q / (k as f64 * k as f64);
        sum += term;
        if term.abs() < 1.0e-15 * sum.abs().max(1.0) { break; }
    }
    sum.is_finite().then_some(sum)
}

fn bessel_y0_value(x: f64) -> Option<f64> {
    if !x.is_finite() || x <= 0.0 || x > 30.0 { return None; }
    let q = x * x / 4.0;
    let mut term = 1.0;
    let mut j0 = 1.0;
    let mut series = 0.0;
    let mut harmonic = 0.0;
    for k in 1..=100 {
        term *= -q / (k as f64 * k as f64);
        j0 += term;
        harmonic += 1.0 / k as f64;
        series += -harmonic * term;
        if term.abs() < 1.0e-15 * j0.abs().max(1.0) { break; }
    }
    let result = 2.0 / std::f64::consts::PI * ((x / 2.0).ln() + EULER_MASCHERONI) * j0
        + 2.0 / std::f64::consts::PI * series;
    result.is_finite().then_some(result)
}

fn inverse_erf_value(x: f64) -> Option<f64> {
    if !x.is_finite() || x <= -1.0 || x >= 1.0 { return None; }
    if x == 0.0 { return Some(0.0); }
    let a = 0.147;
    let ln = (1.0 - x * x).ln();
    let first = 2.0 / (std::f64::consts::PI * a) + ln / 2.0;
    let mut y = x.signum() * ((first * first - ln / a).sqrt() - first).sqrt();
    for _ in 0..5 {
        let error = erf_approx(y) - x;
        let derivative = 2.0 / std::f64::consts::PI.sqrt() * (-y * y).exp();
        y -= error / derivative;
    }
    y.is_finite().then_some(y)
}

fn normal_quantile_value(p: f64) -> Option<f64> {
    if !p.is_finite() || p <= 0.0 || p >= 1.0 { return None; }
    const A: [f64; 6] = [-3.969683028665376e1, 2.209460984245205e2, -2.759285104469687e2, 1.383577518672690e2, -3.066479806614716e1, 2.506628277459239];
    const B: [f64; 5] = [-5.447609879822406e1, 1.615858368580409e2, -1.556989798598866e2, 6.680131188771972e1, -1.328068155288572e1];
    const C: [f64; 6] = [-7.784894002430293e-3, -3.223964580411365e-1, -2.400758277161838, -2.549732539343734, 4.374664141464968, 2.938163982698783];
    const D: [f64; 4] = [7.784695709041462e-3, 3.224671290700398e-1, 2.445134137142996, 3.754408661907416];
    let plow = 0.02425;
    let mut x = if p < plow {
        let q = (-2.0 * p.ln()).sqrt();
        (((((C[0]*q+C[1])*q+C[2])*q+C[3])*q+C[4])*q+C[5]) / ((((D[0]*q+D[1])*q+D[2])*q+D[3])*q+1.0)
    } else if p > 1.0 - plow {
        let q = (-2.0 * (1.0-p).ln()).sqrt();
        -(((((C[0]*q+C[1])*q+C[2])*q+C[3])*q+C[4])*q+C[5]) / ((((D[0]*q+D[1])*q+D[2])*q+D[3])*q+1.0)
    } else {
        let q = p - 0.5;
        let r = q*q;
        (((((A[0]*r+A[1])*r+A[2])*r+A[3])*r+A[4])*r+A[5])*q / (((((B[0]*r+B[1])*r+B[2])*r+B[3])*r+B[4])*r+1.0)
    };
    for _ in 0..2 {
        let cdf = 0.5 * (1.0 + erf_approx(x / std::f64::consts::SQRT_2));
        let pdf = (-0.5*x*x).exp() / (2.0*std::f64::consts::PI).sqrt();
        x -= (cdf - p) / pdf;
    }
    x.is_finite().then_some(x)
}

fn jacobi_symbol_value(mut a: i64, mut n: i64) -> Option<f64> {
    if n <= 0 || n % 2 == 0 { return None; }
    a = a.rem_euclid(n);
    let mut result = 1_i64;
    while a != 0 {
        while a % 2 == 0 {
            a /= 2;
            let r = n % 8;
            if r == 3 || r == 5 { result = -result; }
        }
        std::mem::swap(&mut a, &mut n);
        if a % 4 == 3 && n % 4 == 3 { result = -result; }
        a %= n;
    }
    Some(if n == 1 { result as f64 } else { 0.0 })
}

fn kronecker_symbol_value(a: i64, n: i64) -> Option<f64> {
    if n == 0 { return Some(if a.abs() == 1 { 1.0 } else { 0.0 }); }
    if n < 0 {
        let sign = if a < 0 { -1.0 } else { 1.0 };
        return Some(sign * kronecker_symbol_value(a, -n)?);
    }
    let mut twos = 0;
    let mut odd = n;
    while odd % 2 == 0 { twos += 1; odd /= 2; }
    let mut result = 1.0;
    if twos > 0 {
        if a % 2 == 0 { return Some(0.0); }
        let r = a.rem_euclid(8);
        let symbol = if r == 1 || r == 7 { 1.0 } else { -1.0 };
        if twos % 2 == 1 { result *= symbol; }
    }
    if odd > 1 { result *= jacobi_symbol_value(a, odd)?; }
    Some(result)
}

fn legendre_symbol_value(a: i64, p: i64) -> Option<f64> {
    if p < 2 || !is_prime_u64(p as u64) { return None; }
    jacobi_symbol_value(a, p)
}

fn clamp_value(x: f64, lo: f64, hi: f64) -> Option<f64> {
    if !lo.is_finite() || !hi.is_finite() || lo > hi { return None; }
    Some(x.clamp(lo, hi))
}

fn lerp_value(a: f64, b: f64, t: f64) -> f64 { a + (b - a) * t }

fn remap_value(x: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> Option<f64> {
    if !in_min.is_finite() || !in_max.is_finite() || in_min == in_max { return None; }
    Some(out_min + (x - in_min) * (out_max - out_min) / (in_max - in_min))
}

fn smoothstep_value(edge0: f64, edge1: f64, x: f64) -> Option<f64> {
    if edge0 == edge1 { return None; }
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    Some(t * t * (3.0 - 2.0 * t))
}

fn modular_power_value(a: f64, b: f64, m: f64) -> Option<f64> {
    let a = integer_argument(a)?;
    let b = integer_argument(b)?;
    let m = integer_argument(m)?;
    if b < 0 || m <= 0 || m > 1_000_000_000_000 { return None; }
    Some(modular_pow_u64(a.rem_euclid(m) as u64, b as u64, m as u64)? as f64)
}

fn binomial_pmf_value(n: f64, k: f64, p: f64) -> Option<f64> {
    let n = integer_argument(n)?;
    let k = integer_argument(k)?;
    if n < 0 || n > 500 || k < 0 || k > n || !p.is_finite() || !(0.0..=1.0).contains(&p) { return None; }
    if p == 0.0 { return Some(if k == 0 { 1.0 } else { 0.0 }); }
    if p == 1.0 { return Some(if k == n { 1.0 } else { 0.0 }); }
    let combination = combinations(n as f64, k as f64)?;
    Some(combination * p.powi(k as i32) * (1.0 - p).powi((n - k) as i32))
}

fn binomial_cdf_value(n: f64, k: f64, p: f64) -> Option<f64> {
    let n = integer_argument(n)?;
    let k = integer_argument(k)?;
    if n < 0 || n > 500 || !p.is_finite() || !(0.0..=1.0).contains(&p) { return None; }
    if k < 0 { return Some(0.0); }
    if k >= n { return Some(1.0); }
    if p == 0.0 { return Some(1.0); }
    if p == 1.0 { return Some(0.0); }
    let mut probability = (1.0 - p).powi(n as i32);
    let mut sum = probability;
    for i in 0..k {
        probability *= (n - i) as f64 / (i + 1) as f64 * p / (1.0 - p).max(f64::MIN_POSITIVE);
        sum += probability;
    }
    sum.is_finite().then_some(sum.clamp(0.0, 1.0))
}

fn chinese_remainder_value(a: f64, m: f64, b: f64, n: f64) -> Option<f64> {
    let a = integer_argument(a)? as i128;
    let m = integer_argument(m)? as i128;
    let b = integer_argument(b)? as i128;
    let n = integer_argument(n)? as i128;
    if m <= 0 || n <= 0 || m > 1_000_000_000 || n > 1_000_000_000 { return None; }
    fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
        if b == 0 { (a, 1, 0) } else {
            let (g, x, y) = egcd(b, a.rem_euclid(b));
            (g, y, x - (a.div_euclid(b)) * y)
        }
    }
    let (g, inv, _) = egcd(m, n);
    let diff = b - a;
    if diff % g != 0 { return None; }
    let n_reduced = n / g;
    let factor = ((diff / g) * inv).rem_euclid(n_reduced);
    let lcm = (m / g) * n;
    let result = (a + m * factor).rem_euclid(lcm);
    (result.abs() <= MAX_ABS_VALUE as i128).then_some(result as f64)
}

fn round_ties_even(value: f64) -> f64 {
    if !value.is_finite() || value.abs() >= 4.5e15 { return value; }
    let lower = value.floor();
    let fraction = value - lower;
    if fraction < 0.5 { lower }
    else if fraction > 0.5 { lower + 1.0 }
    else if (lower as i64) & 1 == 0 { lower }
    else { lower + 1.0 }
}

fn ilogb_value(value: f64) -> Option<f64> {
    if value == 0.0 || !value.is_finite() { return None; }
    let bits = value.abs().to_bits();
    let exponent = ((bits >> 52) & 0x7ff) as i32;
    if exponent != 0 {
        Some((exponent - 1023) as f64)
    } else {
        let mantissa = bits & ((1_u64 << 52) - 1);
        Some((63 - mantissa.leading_zeros() as i32 - 1074) as f64)
    }
}

fn ulp_value(value: f64) -> Option<f64> {
    if !value.is_finite() { return None; }
    let x = value.abs();
    let next = next_after_value(x, f64::INFINITY);
    let result = next - x;
    let result = if result.is_finite() { result } else { x - next_after_value(x, f64::NEG_INFINITY) };
    result.is_finite().then_some(result)
}

fn prime_factor_summary(value: f64) -> Option<(u64, u64, u64, u64)> {
    let n = integer_argument(value)?;
    if n < 2 || n > 10_000_000 { return None; }
    let mut remaining = n as u64;
    let mut smallest = 0_u64;
    let mut largest = 0_u64;
    let mut distinct = 0_u64;
    let mut total = 0_u64;
    let mut p = 2_u64;
    while p <= remaining / p {
        if remaining % p == 0 {
            if smallest == 0 { smallest = p; }
            largest = p;
            distinct += 1;
            while remaining % p == 0 {
                remaining /= p;
                total += 1;
            }
        }
        p += if p == 2 { 1 } else { 2 };
    }
    if remaining > 1 {
        if smallest == 0 { smallest = remaining; }
        largest = remaining;
        distinct += 1;
        total += 1;
    }
    Some((smallest, largest, distinct, total))
}

fn is_abundant_or_deficient(value: f64) -> Option<(f64, f64)> {
    let n = integer_argument(value)?;
    if n < 1 || n > 100_000 { return None; }
    let proper = aliquot_sum_value(value)?;
    Some((if proper > n as f64 { 1.0 } else { 0.0 }, if proper < n as f64 { 1.0 } else { 0.0 }))
}

fn superfactorial_value(value: f64) -> Option<f64> {
    let n = integer_argument(value)?;
    if !(0..=10).contains(&n) { return None; }
    let mut result = 1.0;
    let mut factorial_value = 1.0;
    for k in 1..=n {
        factorial_value *= k as f64;
        result *= factorial_value;
        if !result.is_finite() || result > MAX_ABS_VALUE { return None; }
    }
    Some(result)
}

fn hyperfactorial_value(value: f64) -> Option<f64> {
    let n = integer_argument(value)?;
    if !(0..=10).contains(&n) { return None; }
    let mut result = 1.0;
    for k in 1..=n {
        result *= (k as f64).powi(k as i32);
        if !result.is_finite() || result > MAX_ABS_VALUE { return None; }
    }
    Some(result)
}

fn airy_series(x: f64, y0: f64, y1: f64) -> Option<f64> {
    if !x.is_finite() || x.abs() > 7.0 { return None; }
    let mut coefficients = [0.0_f64; 64];
    coefficients[0] = y0;
    coefficients[1] = y1;
    for n in 1..62 {
        coefficients[n + 2] = coefficients[n - 1] / ((n + 2) * (n + 1)) as f64;
    }
    let result = coefficients.iter().rev().fold(0.0, |acc, coefficient| acc * x + *coefficient);
    result.is_finite().then_some(result)
}

fn airy_ai_value(x: f64) -> Option<f64> {
    let ai0 = 1.0 / (3.0_f64.powf(2.0 / 3.0) * gamma_function(2.0 / 3.0)?);
    let ai1 = -1.0 / (3.0_f64.powf(1.0 / 3.0) * gamma_function(1.0 / 3.0)?);
    airy_series(x, ai0, ai1)
}

fn airy_bi_value(x: f64) -> Option<f64> {
    let bi0 = 1.0 / (3.0_f64.powf(1.0 / 6.0) * gamma_function(2.0 / 3.0)?);
    let bi1 = 3.0_f64.powf(1.0 / 6.0) / gamma_function(1.0 / 3.0)?;
    airy_series(x, bi0, bi1)
}

fn dawson_value(x: f64) -> Option<f64> {
    if !x.is_finite() || x.abs() > 8.0 { return None; }
    if x == 0.0 { return Some(0.0); }
    let sign = x.signum();
    let end = x.abs();
    let steps = 128usize;
    let h = end / steps as f64;
    let mut sum = (-end * end).exp() + 1.0;
    for i in 1..steps {
        let t = i as f64 * h;
        let y = (t * t - end * end).exp();
        sum += if i % 2 == 0 { 2.0 * y } else { 4.0 * y };
    }
    Some(sign * sum * h / 3.0)
}

fn expint_ei_value(x: f64) -> Option<f64> {
    if !x.is_finite() || x == 0.0 || x.abs() > 20.0 { return None; }
    let mut term = 1.0;
    let mut sum = 0.0;
    for k in 1..=300 {
        term *= x / k as f64;
        let add = term / k as f64;
        sum += add;
        if add.abs() < 1.0e-15 * sum.abs().max(1.0) { break; }
    }
    let result = EULER_MASCHERONI + x.abs().ln() + sum;
    result.is_finite().then_some(result)
}

fn bessel_i0_value(x: f64) -> Option<f64> {
    if !x.is_finite() || x.abs() > 20.0 { return None; }
    let q = x * x / 4.0;
    let mut term = 1.0;
    let mut sum = 1.0;
    for k in 1..=200 {
        term *= q / (k as f64 * k as f64);
        sum += term;
        if term.abs() < 1.0e-15 * sum.abs().max(1.0) { break; }
    }
    sum.is_finite().then_some(sum)
}

fn bessel_i1_value(x: f64) -> Option<f64> {
    if !x.is_finite() || x.abs() > 20.0 { return None; }
    let q = x * x / 4.0;
    let mut term = x / 2.0;
    let mut sum = term;
    for k in 1..=200 {
        term *= q / (k as f64 * (k + 1) as f64);
        sum += term;
        if term.abs() < 1.0e-15 * sum.abs().max(1.0) { break; }
    }
    sum.is_finite().then_some(sum)
}

fn bessel_k_value(x: f64, order: usize) -> Option<f64> {
    if !x.is_finite() || x <= 0.0 || x > 30.0 { return None; }
    let end = (8.0 + (1.0 / x).ln().max(0.0)).min(18.0);
    let steps = 128usize;
    let h = end / steps as f64;
    let integrand = |t: f64| (-x * t.cosh()).exp() * (if order == 0 { 1.0 } else { t.cosh() });
    let mut sum = integrand(0.0) + integrand(end);
    for i in 1..steps {
        let y = integrand(i as f64 * h);
        sum += if i % 2 == 0 { 2.0 * y } else { 4.0 * y };
    }
    let result = sum * h / 3.0;
    result.is_finite().then_some(result)
}

fn log1pmx_value(x: f64) -> Option<f64> {
    if !x.is_finite() || x <= -1.0 { return None; }
    let result = if x.abs() < 1.0e-4 {
        let mut term = x * x;
        let mut sum = -term / 2.0;
        for k in 3..=30 {
            term *= x;
            let add = if k % 2 == 0 { -term / k as f64 } else { term / k as f64 };
            sum += add;
        }
        sum
    } else { x.ln_1p() - x };
    result.is_finite().then_some(result)
}

fn hurwitz_zeta_value(s: f64, q: f64) -> Option<f64> {
    if !s.is_finite() || !q.is_finite() || s <= 1.0 || q <= 0.0 || s > 100.0 || q > 1.0e6 { return None; }
    const N: usize = 40;
    let mut sum = 0.0;
    for k in 0..N { sum += (q + k as f64).powf(-s); }
    let x = q + N as f64;
    sum += x.powf(1.0 - s) / (s - 1.0) + 0.5 * x.powf(-s);
    sum += s / 12.0 * x.powf(-s - 1.0);
    sum -= s * (s + 1.0) * (s + 2.0) / 720.0 * x.powf(-s - 3.0);
    sum += s * (s + 1.0) * (s + 2.0) * (s + 3.0) * (s + 4.0) / 30240.0 * x.powf(-s - 5.0);
    sum.is_finite().then_some(sum)
}

fn gamma_pq(a: f64, x: f64) -> Option<(f64, f64)> {
    if !a.is_finite() || !x.is_finite() || a <= 0.0 || a > 100.0 || x < 0.0 || x > 1.0e6 { return None; }
    if x == 0.0 { return Some((0.0, 1.0)); }
    let log_factor = -x + a * x.ln() - gamma_function(a)?.ln();
    if x < a + 1.0 {
        let mut ap = a;
        let mut delta = 1.0 / a;
        let mut sum = delta;
        for _ in 0..500 {
            ap += 1.0;
            delta *= x / ap;
            sum += delta;
            if delta.abs() < sum.abs() * 1.0e-15 { break; }
        }
        let p = (sum * log_factor.exp()).clamp(0.0, 1.0);
        Some((p, (1.0 - p).clamp(0.0, 1.0)))
    } else {
        let tiny = 1.0e-300;
        let mut b = x + 1.0 - a;
        let mut c = 1.0 / tiny;
        let mut d = 1.0 / b;
        let mut h = d;
        for i in 1..=500 {
            let fi = i as f64;
            let an = -fi * (fi - a);
            b += 2.0;
            d = an * d + b;
            if d.abs() < tiny { d = tiny; }
            c = b + an / c;
            if c.abs() < tiny { c = tiny; }
            d = 1.0 / d;
            let delta = d * c;
            h *= delta;
            if (delta - 1.0).abs() < 1.0e-15 { break; }
        }
        let q = (log_factor.exp() * h).clamp(0.0, 1.0);
        Some(((1.0 - q).clamp(0.0, 1.0), q))
    }
}

fn beta_continued_fraction(a: f64, b: f64, x: f64) -> Option<f64> {
    let qab = a + b;
    let qap = a + 1.0;
    let qam = a - 1.0;
    let tiny = 1.0e-300;
    let mut c = 1.0;
    let mut d = 1.0 - qab * x / qap;
    if d.abs() < tiny { d = tiny; }
    d = 1.0 / d;
    let mut h = d;
    for m in 1..=300 {
        let m_f = m as f64;
        let m2 = 2.0 * m_f;
        let mut aa = m_f * (b - m_f) * x / ((qam + m2) * (a + m2));
        d = 1.0 + aa * d;
        if d.abs() < tiny { d = tiny; }
        c = 1.0 + aa / c;
        if c.abs() < tiny { c = tiny; }
        d = 1.0 / d;
        h *= d * c;
        aa = -(a + m_f) * (qab + m_f) * x / ((a + m2) * (qap + m2));
        d = 1.0 + aa * d;
        if d.abs() < tiny { d = tiny; }
        c = 1.0 + aa / c;
        if c.abs() < tiny { c = tiny; }
        d = 1.0 / d;
        let delta = d * c;
        h *= delta;
        if (delta - 1.0).abs() < 3.0e-14 { break; }
    }
    h.is_finite().then_some(h)
}

fn regularized_beta_value(x: f64, a: f64, b: f64) -> Option<f64> {
    if !x.is_finite() || !a.is_finite() || !b.is_finite() || !(0.0..=1.0).contains(&x) || a <= 0.0 || b <= 0.0 || a > 100.0 || b > 100.0 { return None; }
    if x == 0.0 { return Some(0.0); }
    if x == 1.0 { return Some(1.0); }
    let bt = (gamma_function(a + b)?.ln() - gamma_function(a)?.ln() - gamma_function(b)?.ln() + a * x.ln() + b * (-x).ln_1p()).exp();
    let result = if x < (a + 1.0) / (a + b + 2.0) {
        bt * beta_continued_fraction(a, b, x)? / a
    } else {
        1.0 - bt * beta_continued_fraction(b, a, 1.0 - x)? / b
    };
    result.is_finite().then_some(result.clamp(0.0, 1.0))
}

fn poisson_pmf_value(k: f64, lambda: f64) -> Option<f64> {
    let k = integer_argument(k)?;
    if k < 0 || k > 500 || !lambda.is_finite() || lambda < 0.0 || lambda > 700.0 { return None; }
    if lambda == 0.0 { return Some(if k == 0 { 1.0 } else { 0.0 }); }
    let log_probability = -lambda + k as f64 * lambda.ln() - gamma_function(k as f64 + 1.0)?.ln();
    Some(log_probability.exp())
}

fn poisson_cdf_value(k: f64, lambda: f64) -> Option<f64> {
    let k = integer_argument(k)?;
    if k < 0 { return Some(0.0); }
    if k > 500 || !lambda.is_finite() || lambda < 0.0 || lambda > 700.0 { return None; }
    let mut sum = 0.0;
    for i in 0..=k { sum += poisson_pmf_value(i as f64, lambda)?; }
    Some(sum.clamp(0.0, 1.0))
}

fn exponential_pdf_value(x: f64, lambda: f64) -> Option<f64> {
    if !x.is_finite() || !lambda.is_finite() || lambda <= 0.0 { return None; }
    Some(if x < 0.0 { 0.0 } else { lambda * (-lambda * x).exp() })
}

fn exponential_cdf_value(x: f64, lambda: f64) -> Option<f64> {
    if !x.is_finite() || !lambda.is_finite() || lambda <= 0.0 { return None; }
    Some(if x < 0.0 { 0.0 } else { 1.0 - (-lambda * x).exp() })
}

fn student_t_pdf_value(x: f64, nu: f64) -> Option<f64> {
    if !x.is_finite() || !nu.is_finite() || nu <= 0.0 || nu > 100.0 { return None; }
    let log_norm = gamma_function((nu + 1.0) / 2.0)?.ln() - gamma_function(nu / 2.0)?.ln() - 0.5 * (nu * std::f64::consts::PI).ln();
    Some((log_norm - (nu + 1.0) / 2.0 * (1.0 + x * x / nu).ln()).exp())
}

fn student_t_cdf_value(x: f64, nu: f64) -> Option<f64> {
    if !x.is_finite() || !nu.is_finite() || nu <= 0.0 || nu > 100.0 { return None; }
    if x == 0.0 { return Some(0.5); }
    let ib = regularized_beta_value(nu / (nu + x * x), nu / 2.0, 0.5)?;
    Some(if x > 0.0 { 1.0 - 0.5 * ib } else { 0.5 * ib })
}

fn chi_squared_pdf_value(x: f64, degrees: f64) -> Option<f64> {
    if !x.is_finite() || !degrees.is_finite() || degrees <= 0.0 || degrees > 200.0 { return None; }
    if x < 0.0 { return Some(0.0); }
    if x == 0.0 { return Some(if degrees < 2.0 { f64::INFINITY } else if degrees == 2.0 { 0.5 } else { 0.0 }); }
    let a = degrees / 2.0;
    Some(((a - 1.0) * x.ln() - x / 2.0 - a * 2.0_f64.ln() - gamma_function(a)?.ln()).exp())
}

fn chi_squared_cdf_value(x: f64, degrees: f64) -> Option<f64> {
    if !x.is_finite() || !degrees.is_finite() || degrees <= 0.0 || degrees > 200.0 { return None; }
    if x <= 0.0 { return Some(0.0); }
    gamma_pq(degrees / 2.0, x / 2.0).map(|pair| pair.0)
}

fn gamma_pdf_value(x: f64, shape: f64, scale: f64) -> Option<f64> {
    if !x.is_finite() || !shape.is_finite() || !scale.is_finite() || shape <= 0.0 || scale <= 0.0 || shape > 100.0 { return None; }
    if x < 0.0 { return Some(0.0); }
    if x == 0.0 { return Some(if shape < 1.0 { f64::INFINITY } else if shape == 1.0 { 1.0 / scale } else { 0.0 }); }
    Some(((shape - 1.0) * x.ln() - x / scale - gamma_function(shape)?.ln() - shape * scale.ln()).exp())
}

fn gamma_cdf_value(x: f64, shape: f64, scale: f64) -> Option<f64> {
    if !x.is_finite() || !shape.is_finite() || !scale.is_finite() || shape <= 0.0 || scale <= 0.0 { return None; }
    if x <= 0.0 { return Some(0.0); }
    gamma_pq(shape, x / scale).map(|pair| pair.0)
}

fn beta_pdf_value(x: f64, alpha: f64, beta: f64) -> Option<f64> {
    if !x.is_finite() || !alpha.is_finite() || !beta.is_finite() || alpha <= 0.0 || beta <= 0.0 { return None; }
    if !(0.0..=1.0).contains(&x) { return Some(0.0); }
    if x == 0.0 || x == 1.0 { return None; }
    Some(((alpha - 1.0) * x.ln() + (beta - 1.0) * (-x).ln_1p() - beta_value(alpha, beta)?.ln()).exp())
}

fn beta_cdf_value(x: f64, alpha: f64, beta: f64) -> Option<f64> {
    regularized_beta_value(x, alpha, beta)
}

fn uniform_pdf_value(x: f64, a: f64, b: f64) -> Option<f64> {
    if !x.is_finite() || !a.is_finite() || !b.is_finite() || a >= b { return None; }
    Some(if x < a || x > b { 0.0 } else { 1.0 / (b - a) })
}

fn uniform_cdf_value(x: f64, a: f64, b: f64) -> Option<f64> {
    if !x.is_finite() || !a.is_finite() || !b.is_finite() || a >= b { return None; }
    Some(((x - a) / (b - a)).clamp(0.0, 1.0))
}

fn multinomial_value(n: f64, k1: f64, k2: f64) -> Option<f64> {
    let n = integer_argument(n)?;
    let k1 = integer_argument(k1)?;
    let k2 = integer_argument(k2)?;
    if n < 0 || n > 20 || k1 < 0 || k2 < 0 || k1 + k2 > n { return None; }
    let k3 = n - k1 - k2;
    let numerator = factorial(n as f64)?;
    let denominator = factorial(k1 as f64)? * factorial(k2 as f64)? * factorial(k3 as f64)?;
    Some(numerator / denominator)
}

fn for_each_chained_added_unary(value: f64, mut emit: impl FnMut(f64, UnaryOp)) {
    if let Some(v) = next_prime_value(value) { emit(v, UnaryOp::NextPrime); }
}

fn for_each_added_unary(value: f64, mut emit: impl FnMut(f64, UnaryOp)) {
    if let Some(v) = digamma_positive(value) { emit(v, UnaryOp::Digamma); }
    if let Some(v) = trigamma_positive(value) { emit(v, UnaryOp::Trigamma); }
    if let Some(v) = zeta_value(value) { emit(v, UnaryOp::Zeta); }
    if let Some(v) = eta_value(value) { emit(v, UnaryOp::Eta); }
    if let Some(v) = erfi_value(value) { emit(v, UnaryOp::Erfi); }
    if let Some(v) = erfcx_value(value) { emit(v, UnaryOp::Erfcx); }
    if let Some(v) = sine_integral_value(value) { emit(v, UnaryOp::SineIntegral); }
    if let Some(v) = cosine_integral_value(value) { emit(v, UnaryOp::CosineIntegral); }
    if let Some(v) = bessel_j0_value(value) { emit(v, UnaryOp::BesselJ0); }
    if let Some(v) = bessel_y0_value(value) { emit(v, UnaryOp::BesselY0); }
    if let Some(v) = inverse_erf_value(value) { emit(v, UnaryOp::InverseErf); }
    if let Some(v) = normal_quantile_value(value) { emit(v, UnaryOp::NormalQuantile); }
    if let Some(v) = prime_count_value(value) { emit(v, UnaryOp::PrimeCount); }
    if let Some(v) = nth_prime_value(value) { emit(v, UnaryOp::NthPrime); }
    if let Some(v) = next_prime_value(value) { emit(v, UnaryOp::NextPrime); }
    if let Some(v) = is_squarefree_value(value) { emit(v, UnaryOp::IsSquarefree); }
    if let Some(v) = aliquot_sum_value(value) { emit(v, UnaryOp::AliquotSum); }
    if let Some(v) = is_carmichael_value(value) { emit(v, UnaryOp::IsCarmichael); }
    if let Some(v) = lucas_value(value) { emit(v, UnaryOp::Lucas); }
    if let Some(v) = pell_value(value) { emit(v, UnaryOp::Pell); }
    if let Some(v) = tribonacci_value(value) { emit(v, UnaryOp::Tribonacci); }
    if let Some(v) = bernoulli_value(value) { emit(v, UnaryOp::Bernoulli); }
    if let Some(v) = euler_number_value(value) { emit(v, UnaryOp::EulerNumber); }
    if let Some(v) = airy_ai_value(value) { emit(v, UnaryOp::AiryAi); }
    if let Some(v) = airy_bi_value(value) { emit(v, UnaryOp::AiryBi); }
    if let Some(v) = dawson_value(value) { emit(v, UnaryOp::Dawson); }
    if let Some(v) = expint_ei_value(value) { emit(v, UnaryOp::Expint); }
    if let Some(v) = bessel_i0_value(value) { emit(v, UnaryOp::BesselI0); }
    if let Some(v) = bessel_i1_value(value) { emit(v, UnaryOp::BesselI1); }
    if let Some(v) = bessel_k_value(value, 0) { emit(v, UnaryOp::BesselK0); }
    if let Some(v) = bessel_k_value(value, 1) { emit(v, UnaryOp::BesselK1); }
    emit(if value.is_finite() { 1.0 } else { 0.0 }, UnaryOp::IsFinite);
    emit(if value.fract() == 0.0 { 1.0 } else { 0.0 }, UnaryOp::IsInteger);
    emit(if value.is_nan() { 1.0 } else { 0.0 }, UnaryOp::IsNan);
}

fn for_each_complex_unary_result(value: Complex, mut emit: impl FnMut(Complex, ComplexUnaryOp)) {
    if value.im == 0.0 {
        if value.re < 0.0 {
            let argument = std::f64::consts::PI;
            emit(Complex::new(argument, 0.0), ComplexUnaryOp::Argument);
            emit(Complex::new(argument, 0.0), ComplexUnaryOp::Phase);
            if let Some(logarithm) = value.ln() {
                emit(logarithm, ComplexUnaryOp::Ln);
            }
            emit(value.sqrt(), ComplexUnaryOp::Sqrt);
        }
        return;
    }

    emit(Complex::new(value.re, -value.im), ComplexUnaryOp::Conjugate);
    emit(Complex::new(value.re, 0.0), ComplexUnaryOp::Real);
    emit(Complex::new(value.im, 0.0), ComplexUnaryOp::Imaginary);
    let argument = value.im.atan2(value.re);
    emit(Complex::new(argument, 0.0), ComplexUnaryOp::Argument);
    let norm = value.re * value.re + value.im * value.im;
    emit(Complex::new(norm, 0.0), ComplexUnaryOp::Norm);
    emit(Complex::new(argument, 0.0), ComplexUnaryOp::Phase);
    emit(value.exp(), ComplexUnaryOp::Exp);
    if let Some(logarithm) = value.ln() {
        emit(logarithm, ComplexUnaryOp::Ln);
    }
    emit(value.sqrt(), ComplexUnaryOp::Sqrt);
    emit(value.sin(), ComplexUnaryOp::Sin);
    emit(value.cos(), ComplexUnaryOp::Cos);
    if let Some(tangent) = value.tan() {
        emit(tangent, ComplexUnaryOp::Tan);
    }
}

fn for_each_added_unary_cached(value: f64, mut emit: impl FnMut(f64, UnaryOp)) {
    let key = if value == 0.0 { 0.0_f64.to_bits() } else { value.to_bits() };
    let results = ADDED_UNARY_RESULT_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some(results) = cache.get(&key) {
            return Arc::clone(results);
        }

        let mut generated = Vec::with_capacity(24);
        for_each_added_unary(value, |result, op| generated.push((result, op)));
        let results: CachedUnaryResults = generated.into();
        if cache.len() < THREAD_CACHE_MAX_ENTRIES {
            cache.insert(key, Arc::clone(&results));
        }
        results
    });

    for &(result, op) in results.iter() {
        emit(result, op);
    }
}

fn render_node(node_id: usize, nodes: &[ExprNode], digits: &[u8], out: &mut String) {
    match nodes[node_id] {
        ExprNode::Number {
            packed_digits,
            len,
            decimal_places,
        } => {
            if decimal_places == len {
                out.push_str("0.");
            }

            for index in 0..len as usize {
                if decimal_places > 0
                    && decimal_places < len
                    && index == len as usize - decimal_places as usize
                {
                    out.push('.');
                }

                let shift = (len as usize - 1 - index) * 4;
                let digit = ((packed_digits >> shift) & 0xF) as u8;
                out.push(char::from(b'0' + digit));
            }
        }
        ExprNode::NumberRange {
            start,
            len,
            decimal_places,
        } => {
            if decimal_places == len {
                out.push_str("0.");
            }
            for index in 0..len as usize {
                if decimal_places > 0
                    && decimal_places < len
                    && index == len as usize - decimal_places as usize
                {
                    out.push('.');
                }
                out.push(char::from(b'0' + digits[start as usize + index]));
            }
        }
        ExprNode::Unary(op, child) => {
            match op {
                UnaryOp::Neg => {
                    out.push_str("(-");
                    render_node(child, nodes, digits, out);
                    out.push(')');
                }
                UnaryOp::Square | UnaryOp::Cube => {
                    out.push('(');
                    render_node(child, nodes, digits, out);
                    out.push_str(if matches!(op, UnaryOp::Square) { " ^ 2)" } else { " ^ 3)" });
                }
                UnaryOp::Reciprocal => {
                    out.push_str("(1 / ");
                    render_node(child, nodes, digits, out);
                    out.push(')');
                }
                UnaryOp::Percent => {
                    out.push('(');
                    render_node(child, nodes, digits, out);
                    out.push_str(" / 100)");
                }
                UnaryOp::Factorial => {
                    out.push('(');
                    render_node(child, nodes, digits, out);
                    out.push_str(")!");
                }
                _ => {
                    out.push_str(unary_name(op));
                    out.push('(');
                    render_node(child, nodes, digits, out);
                    out.push(')');
                }
            }
        }
        ExprNode::Binary(op, left, right) => {
            if let Some(symbol) = binary_symbol(op) {
                out.push('(');
                render_node(left, nodes, digits, out);
                out.push_str(symbol);
                render_node(right, nodes, digits, out);
                out.push(')');
            } else {
                out.push_str(binary_name(op));
                out.push('(');
                render_node(left, nodes, digits, out);
                out.push_str(", ");
                render_node(right, nodes, digits, out);
                out.push(')');
            }
        }
        ExprNode::Ternary(op, first, second, third) => {
            out.push_str(ternary_name(op));
            out.push('(');
            render_node(first, nodes, digits, out);
            out.push_str(", ");
            render_node(second, nodes, digits, out);
            out.push_str(", ");
            render_node(third, nodes, digits, out);
            out.push(')');
        }
        ExprNode::Quaternary(op, first, second, third, fourth) => {
            out.push_str(quaternary_name(op));
            out.push('(');
            render_node(first, nodes, digits, out);
            out.push_str(", ");
            render_node(second, nodes, digits, out);
            out.push_str(", ");
            render_node(third, nodes, digits, out);
            out.push_str(", ");
            render_node(fourth, nodes, digits, out);
            out.push(')');
        }
        ExprNode::Quinary(op, first, second, third, fourth, fifth) => {
            out.push_str(quinary_name(op));
            out.push('(');
            render_node(first, nodes, digits, out);
            out.push_str(", ");
            render_node(second, nodes, digits, out);
            out.push_str(", ");
            render_node(third, nodes, digits, out);
            out.push_str(", ");
            render_node(fourth, nodes, digits, out);
            out.push_str(", ");
            render_node(fifth, nodes, digits, out);
            out.push(')');
        }
        ExprNode::ComplexUnary(op, child) => {
            out.push_str(match op {
                ComplexUnaryOp::Conjugate => "conj",
                ComplexUnaryOp::Real => "real",
                ComplexUnaryOp::Imaginary => "imag",
                ComplexUnaryOp::Argument => "arg",
                ComplexUnaryOp::Norm => "norm",
                ComplexUnaryOp::Phase => "phase",
                ComplexUnaryOp::Exp => "complex_exp",
                ComplexUnaryOp::Ln => "complex_ln",
                ComplexUnaryOp::Sqrt => "complex_sqrt",
                ComplexUnaryOp::Sin => "complex_sin",
                ComplexUnaryOp::Cos => "complex_cos",
                ComplexUnaryOp::Tan => "complex_tan",
            });
            out.push('(');
            render_node(child, nodes, digits, out);
            out.push(')');
        }
        ExprNode::ComplexBinary(op, left, right) => {
            out.push_str(match op {
                ComplexBinaryOp::Add => "complex_add",
                ComplexBinaryOp::Subtract => "complex_sub",
                ComplexBinaryOp::Multiply => "complex_mul",
                ComplexBinaryOp::Divide => "complex_div",
                ComplexBinaryOp::Power => "complex_pow",
                ComplexBinaryOp::Logarithm => "complex_log",
            });
            out.push('(');
            render_node(left, nodes, digits, out);
            out.push_str(", ");
            render_node(right, nodes, digits, out);
            out.push(')');
        }
    }
}

fn render_expr(node_id: usize, nodes: &[ExprNode], digits: &[u8]) -> String {
    let mut result = String::with_capacity(128);
    render_node(node_id, nodes, digits, &mut result);
    result
}

#[inline]
fn unary_name(op: UnaryOp) -> &'static str {
    match op {
        UnaryOp::Cbrt => "cbrt",
        UnaryOp::Ceil => "ceil",
        UnaryOp::Floor => "floor",
        UnaryOp::Round => "round",
        UnaryOp::Trunc => "trunc",
        UnaryOp::Fract => "fract",
        UnaryOp::Abs => "abs",
        UnaryOp::Sin => "sin",
        UnaryOp::Cos => "cos",
        UnaryOp::Tan => "tan",
        UnaryOp::Asin => "asin",
        UnaryOp::Acos => "acos",
        UnaryOp::Atan => "atan",
        UnaryOp::Sinh => "sinh",
        UnaryOp::Cosh => "cosh",
        UnaryOp::Tanh => "tanh",
        UnaryOp::Asinh => "asinh",
        UnaryOp::Acosh => "acosh",
        UnaryOp::Atanh => "atanh",
        UnaryOp::Degrees => "degrees",
        UnaryOp::Radians => "radians",
        UnaryOp::Exp => "exp",
        UnaryOp::TenPow => "10^",
        UnaryOp::TwoPow => "2^",
        UnaryOp::Ln => "ln",
        UnaryOp::Log2 => "log2",
        UnaryOp::Log10 => "log10",
        UnaryOp::Ln1p => "ln1p",
        UnaryOp::Expm1 => "expm1",
        UnaryOp::Sqrt => "sqrt",
        UnaryOp::Signum => "sign",
        UnaryOp::Heaviside => "heaviside",
        UnaryOp::Exp2 => "exp2",
        UnaryOp::DoubleFactorial => "double_factorial",
        UnaryOp::Primorial => "primorial",
        UnaryOp::DigitSum => "digit_sum",
        UnaryOp::DigitalRoot => "digital_root",
        UnaryOp::Sinc => "sinc",
        UnaryOp::Gamma => "gamma",
        UnaryOp::Logistic => "logistic",
        UnaryOp::Softplus => "softplus",
        UnaryOp::Csc => "csc",
        UnaryOp::Sec => "sec",
        UnaryOp::Cot => "cot",
        UnaryOp::Sech => "sech",
        UnaryOp::Csch => "csch",
        UnaryOp::Coth => "coth",
        UnaryOp::Logit => "logit",
        UnaryOp::Softsign => "softsign",
        UnaryOp::Erf => "erf",
        UnaryOp::Erfc => "erfc",
        UnaryOp::NormalCdf => "normal_cdf",
        UnaryOp::Fibonacci => "fibonacci",
        UnaryOp::Triangular => "triangular",
        UnaryOp::Catalan => "catalan",
        UnaryOp::IsPrime => "is_prime",
        UnaryOp::Totient => "totient",
        UnaryOp::DivisorCount => "divisor_count",
        UnaryOp::PrimeOmega => "prime_omega",
        UnaryOp::Mobius => "mobius",
        UnaryOp::Popcount => "popcount",
        UnaryOp::BitLength => "bit_length",
        UnaryOp::DigitProduct => "digit_product",
        UnaryOp::AlternatingDigitSum => "alternating_digit_sum",
        UnaryOp::LogAbs => "ln_abs",
        UnaryOp::Log2Abs => "log2_abs",
        UnaryOp::Log10Abs => "log10_abs",
        UnaryOp::SqrtAbs => "sqrt_abs",
        UnaryOp::FourthPower => "fourth_power",
        UnaryOp::FifthPower => "fifth_power",
        UnaryOp::ReciprocalSquare => "reciprocal_square",
        UnaryOp::ReciprocalSqrt => "reciprocal_sqrt",
        UnaryOp::NegativeExp => "exp_negative",
        UnaryOp::NormalPdf => "normal_pdf",
        UnaryOp::Gudermannian => "gudermannian",
        UnaryOp::InverseGudermannian => "inverse_gudermannian",
        UnaryOp::Versin => "versin",
        UnaryOp::Vercosine => "vercosine",
        UnaryOp::Haversine => "haversine",
        UnaryOp::Havercosine => "havercosine",
        UnaryOp::Chord => "chord",
        UnaryOp::Exsec => "exsec",
        UnaryOp::Excsc => "excsc",
        UnaryOp::LogGamma => "log_gamma",
        UnaryOp::HarmonicNumber => "harmonic_number",
        UnaryOp::Derangement => "derangement",
        UnaryOp::PartitionNumber => "partition_number",
        UnaryOp::BellNumber => "bell_number",
        UnaryOp::IsEven => "is_even",
        UnaryOp::IsOdd => "is_odd",
        UnaryOp::IsPerfectSquare => "is_perfect_square",
        UnaryOp::IsPerfectNumber => "is_perfect_number",
        UnaryOp::ReverseDigits => "reverse_digits",
        UnaryOp::DigitCount => "digit_count",
        UnaryOp::Radical => "radical",
        UnaryOp::DivisorSum => "divisor_sum",
        UnaryOp::Liouville => "liouville",
        UnaryOp::CarmichaelLambda => "carmichael_lambda",
        UnaryOp::Sinhc => "sinhc",
        UnaryOp::Exprel => "exprel",
        UnaryOp::Coshm1 => "coshm1",
        UnaryOp::Log1mexp => "log1mexp",
        UnaryOp::SinPi => "sin_pi",
        UnaryOp::CosPi => "cos_pi",
        UnaryOp::TanPi => "tan_pi",
        UnaryOp::LambertW => "lambert_w",
        UnaryOp::Digamma => "digamma",
        UnaryOp::Trigamma => "trigamma",
        UnaryOp::Zeta => "zeta",
        UnaryOp::Eta => "eta",
        UnaryOp::Erfi => "erfi",
        UnaryOp::Erfcx => "erfcx",
        UnaryOp::SineIntegral => "sine_integral",
        UnaryOp::CosineIntegral => "cosine_integral",
        UnaryOp::BesselJ0 => "bessel_j0",
        UnaryOp::BesselY0 => "bessel_y0",
        UnaryOp::InverseErf => "inverse_erf",
        UnaryOp::NormalQuantile => "normal_quantile",
        UnaryOp::PrimeCount => "prime_count",
        UnaryOp::NthPrime => "nth_prime",
        UnaryOp::NextPrime => "next_prime",
        UnaryOp::IsSquarefree => "is_squarefree",
        UnaryOp::AliquotSum => "aliquot_sum",
        UnaryOp::IsCarmichael => "is_carmichael",
        UnaryOp::Lucas => "lucas",
        UnaryOp::Pell => "pell",
        UnaryOp::Tribonacci => "tribonacci",
        UnaryOp::Bernoulli => "bernoulli",
        UnaryOp::EulerNumber => "euler_number",
        UnaryOp::IsFinite => "is_finite",
        UnaryOp::IsInteger => "is_integer",
        UnaryOp::IsNan => "is_nan",
        UnaryOp::RoundEven => "round_even",
        UnaryOp::RoundAway => "round_away",
        UnaryOp::Rint => "rint",
        UnaryOp::NearbyInt => "nearbyint",
        UnaryOp::InverseSquare => "inverse_square",
        UnaryOp::InverseCube => "inverse_cube",
        UnaryOp::Acsc => "acsc",
        UnaryOp::Asec => "asec",
        UnaryOp::Acot => "acot",
        UnaryOp::Acsch => "acsch",
        UnaryOp::Asech => "asech",
        UnaryOp::Acoth => "acoth",
        UnaryOp::NextUp => "next_up",
        UnaryOp::NextDown => "next_down",
        UnaryOp::Ulp => "ulp",
        UnaryOp::Ilogb => "ilogb",
        UnaryOp::IsNormal => "is_normal",
        UnaryOp::IsSubnormal => "is_subnormal",
        UnaryOp::IsInfinite => "is_infinite",
        UnaryOp::IsSignPositive => "is_sign_positive",
        UnaryOp::IsSignNegative => "is_sign_negative",
        UnaryOp::IsComposite => "is_composite",
        UnaryOp::IsPrimePower => "is_prime_power",
        UnaryOp::SmallestPrimeFactor => "smallest_prime_factor",
        UnaryOp::LargestPrimeFactor => "largest_prime_factor",
        UnaryOp::Omega => "omega",
        UnaryOp::BigOmega => "big_omega",
        UnaryOp::IsAbundant => "is_abundant",
        UnaryOp::IsDeficient => "is_deficient",
        UnaryOp::Subfactorial => "subfactorial",
        UnaryOp::Superfactorial => "superfactorial",
        UnaryOp::Hyperfactorial => "hyperfactorial",
        UnaryOp::BitNot => "bit_not",
        UnaryOp::CountOnes => "count_ones",
        UnaryOp::CountZeros => "count_zeros",
        UnaryOp::LeadingZeros => "leading_zeros",
        UnaryOp::TrailingZeros => "trailing_zeros",
        UnaryOp::ReverseBits => "reverse_bits",
        UnaryOp::ByteSwap => "byte_swap",
        UnaryOp::AiryAi => "airy_ai",
        UnaryOp::AiryBi => "airy_bi",
        UnaryOp::Dawson => "dawson",
        UnaryOp::Expint => "expint",
        UnaryOp::BesselI0 => "bessel_i0",
        UnaryOp::BesselI1 => "bessel_i1",
        UnaryOp::BesselK0 => "bessel_k0",
        UnaryOp::BesselK1 => "bessel_k1",
        UnaryOp::OneMinus => "one_minus",
        UnaryOp::Frac => "frac",
        UnaryOp::Rsqrt => "rsqrt",
        UnaryOp::IsPositive => "is_positive",
        UnaryOp::IsNegative => "is_negative",
        UnaryOp::IsZero => "is_zero",
        UnaryOp::Log1pmx => "log1pmx",
        UnaryOp::Neg | UnaryOp::Square | UnaryOp::Cube | UnaryOp::Reciprocal
        | UnaryOp::Percent | UnaryOp::Factorial => unreachable!(),
    }
}

#[inline]
fn binary_symbol(op: BinaryOp) -> Option<&'static str> {
    match op {
        BinaryOp::Add => Some(" + "),
        BinaryOp::Sub => Some(" - "),
        BinaryOp::Mul => Some(" * "),
        BinaryOp::Div => Some(" / "),
        BinaryOp::Mod => Some(" % "),
        BinaryOp::Pow => Some(" ^ "),
        _ => None,
    }
}

#[inline]
fn binary_name(op: BinaryOp) -> &'static str {
    match op {
        BinaryOp::Min => "min",
        BinaryOp::Max => "max",
        BinaryOp::Hypot => "hypot",
        BinaryOp::Atan2 => "atan2",
        BinaryOp::LogBase => "log_base",
        BinaryOp::Root => "root",
        BinaryOp::Gcd => "gcd",
        BinaryOp::Lcm => "lcm",
        BinaryOp::NCr => "nCr",
        BinaryOp::NPr => "nPr",
        BinaryOp::Average => "average",
        BinaryOp::GeometricMean => "geometric_mean",
        BinaryOp::HarmonicMean => "harmonic_mean",
        BinaryOp::Rms => "rms",
        BinaryOp::Distance => "distance",
        BinaryOp::CopySign => "copysign",
        BinaryOp::FloorDiv => "floor_div",
        BinaryOp::EuclideanMod => "rem_euclid",
        BinaryOp::LogAddExp => "logaddexp",
        BinaryOp::XLogY => "xlogy",
        BinaryOp::Beta => "beta",
        BinaryOp::LogarithmicMean => "logarithmic_mean",
        BinaryOp::ArithmeticGeometricMean => "agm",
        BinaryOp::SumSquares => "sum_squares",
        BinaryOp::DifferenceSquares => "difference_squares",
        BinaryOp::IsCoprime => "is_coprime",
        BinaryOp::Polygonal => "polygonal",
        BinaryOp::BitAnd => "bit_and",
        BinaryOp::BitOr => "bit_or",
        BinaryOp::BitXor => "bit_xor",
        BinaryOp::IsDivisible => "is_divisible",
        BinaryOp::ContraharmonicMean => "contraharmonic_mean",
        BinaryOp::HeronianMean => "heronian_mean",
        BinaryOp::ParallelSum => "parallel_sum",
        BinaryOp::ReciprocalSum => "reciprocal_sum",
        BinaryOp::IdentricMean => "identric_mean",
        BinaryOp::LehmerMean3 => "lehmer_mean_3",
        BinaryOp::MaxMagnitude => "max_magnitude",
        BinaryOp::MinMagnitude => "min_magnitude",
        BinaryOp::NextAfter => "next_after",
        BinaryOp::HammingDistance => "hamming_distance",
        BinaryOp::ProductMinusSum => "product_minus_sum",
        BinaryOp::SmoothMinimum => "smooth_minimum",
        BinaryOp::ModularInverse => "modular_inverse",
        BinaryOp::IsPowerOf => "is_power_of",
        BinaryOp::Polylog => "polylog",
        BinaryOp::SigmaK => "sigma_k",
        BinaryOp::JordanTotient => "jordan_totient",
        BinaryOp::MultiplicativeOrder => "multiplicative_order",
        BinaryOp::StirlingFirst => "stirling1",
        BinaryOp::StirlingSecond => "stirling2",
        BinaryOp::Eulerian => "eulerian",
        BinaryOp::LegendreSymbol => "legendre",
        BinaryOp::JacobiSymbol => "jacobi",
        BinaryOp::KroneckerSymbol => "kronecker",
        BinaryOp::RoundTo => "round_to",
        BinaryOp::Fmod => "fmod",
        BinaryOp::Remainder => "remainder",
        BinaryOp::Quotient => "quotient",
        BinaryOp::IntegerRoot => "integer_root",
        BinaryOp::SignedPow => "signed_pow",
        BinaryOp::SmallestCommonMultiple => "smallest_common_multiple",
        BinaryOp::Binomial => "binomial",
        BinaryOp::Choose => "choose",
        BinaryOp::Permute => "permute",
        BinaryOp::LogAddExp2 => "logaddexp2",
        BinaryOp::LogSubExp => "log_sub_exp",
        BinaryOp::HurwitzZeta => "hurwitz_zeta",
        BinaryOp::IncompleteGamma => "incomplete_gamma",
        BinaryOp::RegularizedGammaP => "regularized_gamma_p",
        BinaryOp::RegularizedGammaQ => "regularized_gamma_q",
        BinaryOp::ShiftLeft => "shift_left",
        BinaryOp::ShiftRight => "shift_right",
        BinaryOp::RotateLeft => "rotate_left",
        BinaryOp::RotateRight => "rotate_right",
        BinaryOp::BitTest => "bit_test",
        BinaryOp::SetBit => "set_bit",
        BinaryOp::ClearBit => "clear_bit",
        BinaryOp::Equal => "equal",
        BinaryOp::NotEqual => "not_equal",
        BinaryOp::LessThan => "less_than",
        BinaryOp::GreaterThan => "greater_than",
        BinaryOp::LessEqual => "less_equal",
        BinaryOp::GreaterEqual => "greater_equal",
        BinaryOp::ExponentialPdf => "exponential_pdf",
        BinaryOp::ExponentialCdf => "exponential_cdf",
        BinaryOp::PoissonPmf => "poisson_pmf",
        BinaryOp::PoissonCdf => "poisson_cdf",
        BinaryOp::StudentTPdf => "student_t_pdf",
        BinaryOp::StudentTCdf => "student_t_cdf",
        BinaryOp::ChiSquaredPdf => "chi_squared_pdf",
        BinaryOp::ChiSquaredCdf => "chi_squared_cdf",
        BinaryOp::DivRemQuotient => "div_rem_quotient",
        BinaryOp::DivRemRemainder => "div_rem_remainder",
        BinaryOp::ExtendedGcdGcd => "extended_gcd_gcd",
        BinaryOp::ExtendedGcdX => "extended_gcd_x",
        BinaryOp::ExtendedGcdY => "extended_gcd_y",
        BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod
        | BinaryOp::Pow => unreachable!(),
    }
}

#[inline]
fn ternary_name(op: TernaryOp) -> &'static str {
    match op {
        TernaryOp::Clamp => "clamp",
        TernaryOp::Lerp => "lerp",
        TernaryOp::Smoothstep => "smoothstep",
        TernaryOp::ModularPower => "mod_pow",
        TernaryOp::BinomialCdf => "binomial_cdf",
        TernaryOp::Fma => "fma",
        TernaryOp::MulAdd => "mul_add",
        TernaryOp::Median3 => "median3",
        TernaryOp::Hypot3 => "hypot3",
        TernaryOp::InverseLerp => "inv_lerp",
        TernaryOp::IsBetween => "is_between",
        TernaryOp::BinomialPmf => "binomial_pmf",
        TernaryOp::BetaInc => "beta_inc",
        TernaryOp::RegularizedBeta => "regularized_beta",
        TernaryOp::ModularAdd => "modular_add",
        TernaryOp::ModularSub => "modular_sub",
        TernaryOp::ModularMul => "modular_mul",
        TernaryOp::Multinomial => "multinomial",
        TernaryOp::GammaPdf => "gamma_pdf",
        TernaryOp::GammaCdf => "gamma_cdf",
        TernaryOp::BetaPdf => "beta_pdf",
        TernaryOp::BetaCdf => "beta_cdf",
        TernaryOp::UniformPdf => "uniform_pdf",
        TernaryOp::UniformCdf => "uniform_cdf",
    }
}

#[inline]
fn quaternary_name(op: QuaternaryOp) -> &'static str {
    match op {
        QuaternaryOp::ChineseRemainder => "crt",
        QuaternaryOp::Bilinear => "bilinear",
        QuaternaryOp::Multilinear => "multilinear",
        QuaternaryOp::CubicInterp => "cubic_interp",
    }
}

#[inline]
fn quinary_name(op: QuinaryOp) -> &'static str {
    match op { QuinaryOp::Remap => "remap" }
}

fn insert_range_options(
    cell: &mut Cell,
    nodes: &mut Vec<ExprNode>,
    target: f64,
    digits: &[u8],
    start: usize,
    len: usize,
) {
    let mut raw_value = 0.0_f64;
    for &digit in &digits[start..start + len] {
        raw_value = raw_value * 10.0 + digit as f64;
    }

    for decimal_places in 0..=len {
        let value = raw_value / 10.0_f64.powi(decimal_places as i32);
        insert_into_cell(
            cell,
            nodes,
            target,
            value,
            ExprNode::NumberRange {
                start: start as u8,
                len: len as u8,
                decimal_places: decimal_places as u8,
            },
        );
    }
}

fn insert_long_binary(
    cell: &mut Cell,
    nodes: &mut Vec<ExprNode>,
    target: f64,
    value: f64,
    left: Candidate,
    right: Candidate,
    op: BinaryOp,
) {
    insert_into_cell(
        cell,
        nodes,
        target,
        value,
        ExprNode::Binary(op, left.node_id, right.node_id),
    );
}

fn generate_long_binary_results(
    cell: &mut Cell,
    nodes: &mut Vec<ExprNode>,
    target: f64,
    a: Candidate,
    b: Candidate,
) {
    for_each_complex_binary_pair_result(a, b, |value, op| {
        insert_complex_into_cell(
            cell,
            nodes,
            target,
            value,
            ExprNode::ComplexBinary(op, a.node_id, b.node_id),
        );
    });
    if a.imag != 0.0 || b.imag != 0.0 {
        return;
    }
    let av = a.value;
    let bv = b.value;
    insert_long_binary(cell, nodes, target, av + bv, a, b, BinaryOp::Add);
    insert_long_binary(cell, nodes, target, av - bv, a, b, BinaryOp::Sub);
    insert_long_binary(cell, nodes, target, bv - av, b, a, BinaryOp::Sub);
    insert_long_binary(cell, nodes, target, av * bv, a, b, BinaryOp::Mul);
    insert_long_binary(cell, nodes, target, av.min(bv), a, b, BinaryOp::Min);
    insert_long_binary(cell, nodes, target, av.max(bv), a, b, BinaryOp::Max);
    insert_long_binary(cell, nodes, target, av.hypot(bv), a, b, BinaryOp::Hypot);
    insert_long_binary(cell, nodes, target, (av + bv) / 2.0, a, b, BinaryOp::Average);
    insert_long_binary(
        cell,
        nodes,
        target,
        (av * av + bv * bv).sqrt() / 2.0_f64.sqrt(),
        a,
        b,
        BinaryOp::Rms,
    );
    insert_long_binary(cell, nodes, target, (av - bv).abs(), a, b, BinaryOp::Distance);
    insert_long_binary(cell, nodes, target, av.abs().copysign(bv), a, b, BinaryOp::CopySign);

    if av >= 0.0 && bv >= 0.0 {
        insert_long_binary(cell, nodes, target, (av * bv).sqrt(), a, b, BinaryOp::GeometricMean);
    }
    if av + bv != 0.0 {
        insert_long_binary(
            cell,
            nodes,
            target,
            2.0 * av * bv / (av + bv),
            a,
            b,
            BinaryOp::HarmonicMean,
        );
    }

    let max_ab = av.max(bv);
    insert_long_binary(
        cell,
        nodes,
        target,
        max_ab + (-(av - bv).abs()).exp().ln_1p(),
        a,
        b,
        BinaryOp::LogAddExp,
    );
    if av == 0.0 {
        insert_long_binary(cell, nodes, target, 0.0, a, b, BinaryOp::XLogY);
    } else if bv > 0.0 {
        insert_long_binary(cell, nodes, target, av * bv.ln(), a, b, BinaryOp::XLogY);
    }

    insert_long_binary(cell, nodes, target, av * av + bv * bv, a, b, BinaryOp::SumSquares);
    insert_long_binary(cell, nodes, target, av * av - bv * bv, a, b, BinaryOp::DifferenceSquares);
    if let Some(result) = beta_value(av, bv) {
        insert_long_binary(cell, nodes, target, result, a, b, BinaryOp::Beta);
    }
    if let Some(result) = logarithmic_mean(av, bv) {
        insert_long_binary(cell, nodes, target, result, a, b, BinaryOp::LogarithmicMean);
    }
    if let Some(result) = arithmetic_geometric_mean(av, bv) {
        insert_long_binary(cell, nodes, target, result, a, b, BinaryOp::ArithmeticGeometricMean);
    }
    if let (Some(ai), Some(bi)) = (integer_argument(av), integer_argument(bv)) {
        insert_long_binary(
            cell,
            nodes,
            target,
            if gcd_i64(ai, bi) == 1 { 1.0 } else { 0.0 },
            a,
            b,
            BinaryOp::IsCoprime,
        );
    }
    if let Some(result) = polygonal_value(av, bv) {
        insert_long_binary(cell, nodes, target, result, a, b, BinaryOp::Polygonal);
    }

    if bv != 0.0 {
        insert_long_binary(cell, nodes, target, av / bv, a, b, BinaryOp::Div);
        insert_long_binary(cell, nodes, target, av % bv, a, b, BinaryOp::Mod);
        insert_long_binary(cell, nodes, target, (av / bv).floor(), a, b, BinaryOp::FloorDiv);
        insert_long_binary(cell, nodes, target, av.rem_euclid(bv), a, b, BinaryOp::EuclideanMod);
    }
    if av != 0.0 {
        insert_long_binary(cell, nodes, target, bv / av, b, a, BinaryOp::Div);
        insert_long_binary(cell, nodes, target, bv % av, b, a, BinaryOp::Mod);
    }
    if bv.abs() <= 12.0 {
        insert_long_binary(cell, nodes, target, av.powf(bv), a, b, BinaryOp::Pow);
    }
    if av.abs() <= 12.0 {
        insert_long_binary(cell, nodes, target, bv.powf(av), b, a, BinaryOp::Pow);
    }
    if av > 0.0 && bv > 0.0 && bv != 1.0 {
        insert_long_binary(cell, nodes, target, av.log(bv), a, b, BinaryOp::LogBase);
    }
    if let Some(result) = nth_root(av, bv) {
        insert_long_binary(cell, nodes, target, result, a, b, BinaryOp::Root);
    }
    if let Some(result) = gcd_value(av, bv) {
        insert_long_binary(cell, nodes, target, result, a, b, BinaryOp::Gcd);
    }
    if let Some(result) = lcm_value(av, bv) {
        insert_long_binary(cell, nodes, target, result, a, b, BinaryOp::Lcm);
    }
    if let Some(result) = combinations(av, bv) {
        insert_long_binary(cell, nodes, target, result, a, b, BinaryOp::NCr);
    }
    if let Some(result) = permutations(av, bv) {
        insert_long_binary(cell, nodes, target, result, a, b, BinaryOp::NPr);
    }
    for_each_extra_binary(av, bv, |result, op| {
        insert_long_binary(cell, nodes, target, result, a, b, op);
    });
    for_each_extra_binary_directional(av, bv, |result, op| {
        insert_long_binary(cell, nodes, target, result, a, b, op);
    });
    for_each_extra_binary_directional(bv, av, |result, op| {
        insert_long_binary(cell, nodes, target, result, b, a, op);
    });
}

fn consider_existing_long_best(
    best: &mut Option<(f64, f64, f64, usize, String)>,
    candidate: Candidate,
    cost: usize,
    _target: f64,
    nodes: &[ExprNode],
    digits: &[u8],
) {
    let score = candidate.score;
    let should_replace = best.as_ref().map_or(true, |(best_score, _, _, best_cost, _)| {
        score < *best_score || (score == *best_score && cost < *best_cost)
    });
    if should_replace {
        *best = Some((
            score,
            candidate.value,
            candidate.imag,
            cost,
            render_expr(candidate.node_id, nodes, digits),
        ));
    }
}

fn consider_unary_long_best(
    best: &mut Option<(f64, f64, f64, usize, String)>,
    nodes: &mut Vec<ExprNode>,
    digits: &[u8],
    target: f64,
    value: f64,
    child: usize,
    op: UnaryOp,
    cost: usize,
) {
    if !value.is_finite() || value.abs() > MAX_ABS_VALUE {
        return;
    }
    let value = if value == 0.0 { 0.0 } else { value };
    let score = (value - target).abs();
    let should_replace = best.as_ref().map_or(true, |(best_score, _, _, best_cost, _)| {
        score < *best_score || (score == *best_score && cost < *best_cost)
    });
    if should_replace {
        let node_id = nodes.len();
        nodes.push(ExprNode::Unary(op, child));
        *best = Some((score, value, 0.0, cost, render_expr(node_id, nodes, digits)));
    }
}

fn consider_complex_unary_long_best(
    best: &mut Option<(f64, f64, f64, usize, String)>,
    nodes: &mut Vec<ExprNode>,
    digits: &[u8],
    target: f64,
    value: Complex,
    child: usize,
    op: ComplexUnaryOp,
    cost: usize,
) {
    if !value.is_finite() || value.magnitude() > MAX_ABS_VALUE {
        return;
    }
    let re = if value.re == 0.0 { 0.0 } else { value.re };
    let im = if value.im == 0.0 { 0.0 } else { value.im };
    let score = (re - target).hypot(im);
    let should_replace = best.as_ref().map_or(true, |(best_score, _, _, best_cost, _)| {
        score < *best_score || (score == *best_score && cost < *best_cost)
    });
    if should_replace {
        let node_id = nodes.len();
        nodes.push(ExprNode::ComplexUnary(op, child));
        *best = Some((score, re, im, cost, render_expr(node_id, nodes, digits)));
    }
}

fn generate_long_unary_best(
    best: &mut Option<(f64, f64, f64, usize, String)>,
    nodes: &mut Vec<ExprNode>,
    digits: &[u8],
    target: f64,
    candidate: Candidate,
    cost: usize,
) {
    let value = candidate.value;
    let child = candidate.node_id;
    let complex_value = Complex::new(candidate.value, candidate.imag);
    for_each_complex_unary_result(complex_value, |result, op| {
        consider_complex_unary_long_best(best, nodes, digits, target, result, child, op, cost);
    });
    if candidate.imag != 0.0 {
        return;
    }
    macro_rules! consider {
        ($result:expr, $op:expr) => {
            consider_unary_long_best(best, nodes, digits, target, $result, child, $op, cost)
        };
    }

    consider!(value.cbrt(), UnaryOp::Cbrt);
    consider!(value.ceil(), UnaryOp::Ceil);
    consider!(value.floor(), UnaryOp::Floor);
    consider!(value.round(), UnaryOp::Round);
    consider!(value.trunc(), UnaryOp::Trunc);
    consider!(value.fract(), UnaryOp::Fract);
    consider!(value.abs(), UnaryOp::Abs);
    consider!(-value, UnaryOp::Neg);
    consider!(value * value, UnaryOp::Square);
    consider!(value * value * value, UnaryOp::Cube);
    if value != 0.0 {
        consider!(1.0 / value, UnaryOp::Reciprocal);
    }
    consider!(value.sin(), UnaryOp::Sin);
    consider!(value.cos(), UnaryOp::Cos);
    consider!(value.tan(), UnaryOp::Tan);
    consider!(value.atan(), UnaryOp::Atan);
    consider!(value.tanh(), UnaryOp::Tanh);
    consider!(value.asinh(), UnaryOp::Asinh);
    if value.abs() <= 28.0 {
        consider!(value.sinh(), UnaryOp::Sinh);
        consider!(value.cosh(), UnaryOp::Cosh);
    }
    if value.abs() <= 1.0 {
        consider!(value.asin(), UnaryOp::Asin);
        consider!(value.acos(), UnaryOp::Acos);
    }
    if value >= 1.0 {
        consider!(value.acosh(), UnaryOp::Acosh);
    }
    if value > -1.0 && value < 1.0 {
        consider!(value.atanh(), UnaryOp::Atanh);
    }
    if value <= 28.0 {
        consider!(value.exp(), UnaryOp::Exp);
        consider!(value.exp_m1(), UnaryOp::Expm1);
    }
    consider!(value.to_degrees(), UnaryOp::Degrees);
    consider!(value.to_radians(), UnaryOp::Radians);
    consider!(value / 100.0, UnaryOp::Percent);
    if value <= 12.0 && value >= -1074.0 {
        consider!(10.0_f64.powf(value), UnaryOp::TenPow);
    }
    if value <= 39.0 && value >= -1074.0 {
        consider!(2.0_f64.powf(value), UnaryOp::TwoPow);
        consider!(value.exp2(), UnaryOp::Exp2);
    }
    if value > 0.0 {
        consider!(value.ln(), UnaryOp::Ln);
        consider!(value.log2(), UnaryOp::Log2);
        consider!(value.log10(), UnaryOp::Log10);
    }
    if value > -1.0 {
        consider!(value.ln_1p(), UnaryOp::Ln1p);
    }
    if value >= 0.0 {
        consider!(value.sqrt(), UnaryOp::Sqrt);
    }
    if let Some(result) = factorial(value) {
        consider!(result, UnaryOp::Factorial);
    }
    consider!(if value == 0.0 { 0.0 } else { value.signum() }, UnaryOp::Signum);
    consider!(if value < 0.0 { 0.0 } else if value == 0.0 { 0.5 } else { 1.0 }, UnaryOp::Heaviside);
    if let Some(result) = double_factorial(value) {
        consider!(result, UnaryOp::DoubleFactorial);
    }
    if let Some(result) = primorial(value) {
        consider!(result, UnaryOp::Primorial);
    }
    if let Some(result) = digit_sum(value) {
        consider!(result, UnaryOp::DigitSum);
    }
    if let Some(result) = digital_root(value) {
        consider!(result, UnaryOp::DigitalRoot);
    }
    let sinc = if value == 0.0 {
        1.0
    } else {
        let angle = std::f64::consts::PI * value;
        angle.sin() / angle
    };
    consider!(sinc, UnaryOp::Sinc);
    if (-5.0..=20.0).contains(&value) {
        if let Some(result) = gamma_function(value) {
            consider!(result, UnaryOp::Gamma);
        }
    }
    consider!(logistic(value), UnaryOp::Logistic);
    consider!(value.max(0.0) + (-value.abs()).exp().ln_1p(), UnaryOp::Softplus);

    let sin_value = value.sin();
    let cos_value = value.cos();
    if sin_value != 0.0 {
        consider!(1.0 / sin_value, UnaryOp::Csc);
        consider!(cos_value / sin_value, UnaryOp::Cot);
    }
    if cos_value != 0.0 {
        consider!(1.0 / cos_value, UnaryOp::Sec);
    }
    if value.abs() <= 28.0 {
        let sinh_value = value.sinh();
        let cosh_value = value.cosh();
        if cosh_value != 0.0 {
            consider!(1.0 / cosh_value, UnaryOp::Sech);
        }
        if sinh_value != 0.0 {
            consider!(1.0 / sinh_value, UnaryOp::Csch);
            consider!(cosh_value / sinh_value, UnaryOp::Coth);
        }
    }
    if value > 0.0 && value < 1.0 {
        consider!((value / (1.0 - value)).ln(), UnaryOp::Logit);
    }
    consider!(value / (1.0 + value.abs()), UnaryOp::Softsign);
    consider!(erf_approx(value), UnaryOp::Erf);
    consider!(erfc_approx(value), UnaryOp::Erfc);
    consider!(0.5 * (1.0 + erf_approx(value / std::f64::consts::SQRT_2)), UnaryOp::NormalCdf);
    if let Some(result) = fibonacci_value(value) {
        consider!(result, UnaryOp::Fibonacci);
    }
    if let Some(result) = triangular_value(value) {
        consider!(result, UnaryOp::Triangular);
    }
    if let Some(result) = catalan_value(value) {
        consider!(result, UnaryOp::Catalan);
    }
    if let Some(facts) = integer_facts(value) {
        consider!(facts.is_prime, UnaryOp::IsPrime);
        if facts.totient.is_finite() {
            consider!(facts.totient, UnaryOp::Totient);
            consider!(facts.divisor_count, UnaryOp::DivisorCount);
            consider!(facts.prime_omega, UnaryOp::PrimeOmega);
            consider!(facts.mobius, UnaryOp::Mobius);
            consider!(facts.is_perfect, UnaryOp::IsPerfectNumber);
            consider!(facts.radical, UnaryOp::Radical);
            consider!(facts.divisor_sum, UnaryOp::DivisorSum);
            consider!(facts.liouville, UnaryOp::Liouville);
            consider!(facts.carmichael_lambda, UnaryOp::CarmichaelLambda);
        }
    }
    if value.fract() == 0.0 && value.abs() <= MAX_INTEGER_ARGUMENT {
        let n = value.abs() as i64;
        consider!(n.count_ones() as f64, UnaryOp::Popcount);
        consider!((64 - (n as u64).leading_zeros()) as f64, UnaryOp::BitLength);
    }
    if let Some(result) = digit_product(value) {
        consider!(result, UnaryOp::DigitProduct);
    }
    if let Some(result) = alternating_digit_sum(value) {
        consider!(result, UnaryOp::AlternatingDigitSum);
    }
    for_each_extra_unary(value, |result, op| {
        consider_unary_long_best(best, nodes, digits, target, result, child, op, cost);
    }, sin_value, cos_value, None);
    if cost <= 1 {
        for_each_added_unary_cached(value, |result, op| {
            consider_unary_long_best(best, nodes, digits, target, result, child, op, cost);
        });
    }
    for_each_chained_added_unary(value, |result, op| {
        consider_unary_long_best(best, nodes, digits, target, result, child, op, cost);
    });
}

fn for_each_extra_unary(
    value: f64,
    mut emit: impl FnMut(f64, UnaryOp),
    sin_value: f64,
    cos_value: f64,
    mut gamma_cache: Option<&mut HashMap<u64, Option<f64>>>,
) {
    emit(value.abs().ln(), UnaryOp::LogAbs);
    emit(value.abs().log2(), UnaryOp::Log2Abs);
    emit(value.abs().log10(), UnaryOp::Log10Abs);
    emit(value.abs().sqrt(), UnaryOp::SqrtAbs);
    emit(value.powi(4), UnaryOp::FourthPower);
    emit(value.powi(5), UnaryOp::FifthPower);
    if value != 0.0 {
        emit(1.0 / (value * value), UnaryOp::ReciprocalSquare);
    }
    if value > 0.0 {
        emit(1.0 / value.sqrt(), UnaryOp::ReciprocalSqrt);
    }
    emit((-value).exp(), UnaryOp::NegativeExp);
    emit((-0.5 * value * value).exp() / (2.0 * std::f64::consts::PI).sqrt(), UnaryOp::NormalPdf);
    emit(2.0 * (value / 2.0).tanh().atan(), UnaryOp::Gudermannian);
    if value.abs() < std::f64::consts::FRAC_PI_2 {
        emit(value.tan().asinh(), UnaryOp::InverseGudermannian);
    }
    emit(1.0 - cos_value, UnaryOp::Versin);
    emit(1.0 + cos_value, UnaryOp::Vercosine);
    emit((value / 2.0).sin().powi(2), UnaryOp::Haversine);
    emit((value / 2.0).cos().powi(2), UnaryOp::Havercosine);
    emit(2.0 * (value / 2.0).sin(), UnaryOp::Chord);
    if cos_value != 0.0 {
        emit(1.0 / cos_value - 1.0, UnaryOp::Exsec);
    }
    if sin_value != 0.0 {
        emit(1.0 / sin_value - 1.0, UnaryOp::Excsc);
    }
    if value > 0.0 && value <= 20.0 {
        let gamma = match gamma_cache.as_deref_mut() {
            Some(cache) => gamma_cached(value, cache),
            None => gamma_function(value),
        };
        if let Some(gamma) = gamma {
            if gamma > 0.0 {
                emit(gamma.ln(), UnaryOp::LogGamma);
            }
        }
    }
    if let Some(result) = harmonic_number(value) {
        emit(result, UnaryOp::HarmonicNumber);
    }
    if let Some(result) = derangement_value(value) {
        emit(result, UnaryOp::Derangement);
    }
    if let Some(result) = partition_number(value) {
        emit(result, UnaryOp::PartitionNumber);
    }
    if let Some(result) = bell_number(value) {
        emit(result, UnaryOp::BellNumber);
    }
    if let Some(n) = integer_argument(value) {
        emit(if n.rem_euclid(2) == 0 { 1.0 } else { 0.0 }, UnaryOp::IsEven);
        emit(if n.rem_euclid(2) != 0 { 1.0 } else { 0.0 }, UnaryOp::IsOdd);
    }
    if let Some(result) = is_perfect_square(value) {
        emit(result, UnaryOp::IsPerfectSquare);
    }
    if let Some(result) = reverse_digits(value) {
        emit(result, UnaryOp::ReverseDigits);
    }
    if let Some(result) = digit_count(value) {
        emit(result, UnaryOp::DigitCount);
    }
    if value.abs() <= 28.0 {
        emit(if value == 0.0 { 1.0 } else { value.sinh() / value }, UnaryOp::Sinhc);
        emit(value.cosh() - 1.0, UnaryOp::Coshm1);
    }
    emit(if value == 0.0 { 1.0 } else { value.exp_m1() / value }, UnaryOp::Exprel);
    if value < 0.0 {
        emit((-value.exp_m1()).ln(), UnaryOp::Log1mexp);
    }
    let angle = std::f64::consts::PI * value;
    emit(angle.sin(), UnaryOp::SinPi);
    emit(angle.cos(), UnaryOp::CosPi);
    emit(angle.tan(), UnaryOp::TanPi);
    if let Some(result) = lambert_w0(value) {
        emit(result, UnaryOp::LambertW);
    }

    let rounded_even = round_ties_even(value);
    emit(rounded_even, UnaryOp::RoundEven);
    emit(value.round(), UnaryOp::RoundAway);
    emit(rounded_even, UnaryOp::Rint);
    emit(rounded_even, UnaryOp::NearbyInt);
    if value != 0.0 {
        emit(1.0 / value.powi(2), UnaryOp::InverseSquare);
        emit(1.0 / value.powi(3), UnaryOp::InverseCube);
        emit(value.recip().asin(), UnaryOp::Acsc);
        emit(value.recip().acos(), UnaryOp::Asec);
        emit(value.recip().atan(), UnaryOp::Acot);
        emit(value.recip().asinh(), UnaryOp::Acsch);
        if value.recip() >= 1.0 { emit(value.recip().acosh(), UnaryOp::Asech); }
        if value.abs() > 1.0 { emit(value.recip().atanh(), UnaryOp::Acoth); }
        emit(next_after_value(value, f64::INFINITY), UnaryOp::NextUp);
        emit(next_after_value(value, f64::NEG_INFINITY), UnaryOp::NextDown);
        if let Some(result) = ulp_value(value) { emit(result, UnaryOp::Ulp); }
        if let Some(result) = ilogb_value(value) { emit(result, UnaryOp::Ilogb); }
    } else {
        emit(f64::from_bits(1), UnaryOp::NextUp);
        emit(-f64::from_bits(1), UnaryOp::NextDown);
        emit(f64::from_bits(1), UnaryOp::Ulp);
    }
    emit(if value.is_normal() { 1.0 } else { 0.0 }, UnaryOp::IsNormal);
    emit(if value != 0.0 && value.is_finite() && !value.is_normal() { 1.0 } else { 0.0 }, UnaryOp::IsSubnormal);
    emit(if value.is_infinite() { 1.0 } else { 0.0 }, UnaryOp::IsInfinite);
    emit(if value.is_sign_positive() { 1.0 } else { 0.0 }, UnaryOp::IsSignPositive);
    emit(if value.is_sign_negative() { 1.0 } else { 0.0 }, UnaryOp::IsSignNegative);
    emit(if value > 0.0 { 1.0 } else { 0.0 }, UnaryOp::IsPositive);
    emit(if value < 0.0 { 1.0 } else { 0.0 }, UnaryOp::IsNegative);
    emit(if value == 0.0 { 1.0 } else { 0.0 }, UnaryOp::IsZero);
    emit(1.0 - value, UnaryOp::OneMinus);
    emit(value.fract(), UnaryOp::Frac);
    if let Some(result) = log1pmx_value(value) { emit(result, UnaryOp::Log1pmx); }
    if value > 0.0 { emit(1.0 / value.sqrt(), UnaryOp::Rsqrt); }

    if let Some(n) = integer_argument(value) {
        if n < 2 {
            emit(0.0, UnaryOp::IsComposite);
            emit(0.0, UnaryOp::IsPrimePower);
        }
        let bits = n as u64;
        emit((!bits) as i64 as f64, UnaryOp::BitNot);
        emit(bits.count_ones() as f64, UnaryOp::CountOnes);
        emit((64 - bits.count_ones()) as f64, UnaryOp::CountZeros);
        emit(bits.leading_zeros() as f64, UnaryOp::LeadingZeros);
        emit(bits.trailing_zeros() as f64, UnaryOp::TrailingZeros);
        emit(bits.reverse_bits() as f64, UnaryOp::ReverseBits);
        emit(bits.swap_bytes() as f64, UnaryOp::ByteSwap);
        if n >= 2 {
            if let Some((smallest, largest, distinct, total)) = prime_factor_summary(value) {
                emit(if distinct == 1 { 1.0 } else { 0.0 }, UnaryOp::IsPrimePower);
                emit(smallest as f64, UnaryOp::SmallestPrimeFactor);
                emit(largest as f64, UnaryOp::LargestPrimeFactor);
                emit(distinct as f64, UnaryOp::Omega);
                emit(total as f64, UnaryOp::BigOmega);
                emit(if is_prime_u64(n as u64) { 0.0 } else { 1.0 }, UnaryOp::IsComposite);
            }
        }
        if let Some((abundant, deficient)) = is_abundant_or_deficient(value) {
            emit(abundant, UnaryOp::IsAbundant);
            emit(deficient, UnaryOp::IsDeficient);
        }
    }
    if let Some(result) = derangement_value(value) { emit(result, UnaryOp::Subfactorial); }
    if let Some(result) = superfactorial_value(value) { emit(result, UnaryOp::Superfactorial); }
    if let Some(result) = hyperfactorial_value(value) { emit(result, UnaryOp::Hyperfactorial); }
}

fn for_each_extra_binary(a: f64, b: f64, mut emit: impl FnMut(f64, BinaryOp)) {
    if let (Some(ai), Some(bi)) = (integer_argument(a), integer_argument(b)) {
        emit((ai & bi) as f64, BinaryOp::BitAnd);
        emit((ai | bi) as f64, BinaryOp::BitOr);
        emit((ai ^ bi) as f64, BinaryOp::BitXor);
        emit((ai.abs() ^ bi.abs()).count_ones() as f64, BinaryOp::HammingDistance);
    }
    let sum = a + b;
    if sum != 0.0 {
        emit((a * a + b * b) / sum, BinaryOp::ContraharmonicMean);
        emit(a * b / sum, BinaryOp::ParallelSum);
    }
    if a >= 0.0 && b >= 0.0 {
        emit((a + b + (a * b).sqrt()) / 3.0, BinaryOp::HeronianMean);
    }
    if a != 0.0 && b != 0.0 {
        emit(1.0 / a + 1.0 / b, BinaryOp::ReciprocalSum);
    }
    if a > 0.0 && b > 0.0 {
        if a == b {
            emit(a, BinaryOp::IdentricMean);
        } else {
            emit(((a * a.ln() - b * b.ln()) / (a - b) - 1.0).exp(), BinaryOp::IdentricMean);
        }
    }
    let denominator = a * a + b * b;
    if denominator != 0.0 {
        emit((a.powi(3) + b.powi(3)) / denominator, BinaryOp::LehmerMean3);
    }
    emit(if a.abs() >= b.abs() { a } else { b }, BinaryOp::MaxMagnitude);
    emit(if a.abs() <= b.abs() { a } else { b }, BinaryOp::MinMagnitude);
    emit(a * b - a - b, BinaryOp::ProductMinusSum);
    let minimum = a.min(b);
    emit(minimum - ((-(a - minimum)).exp() + (-(b - minimum)).exp()).ln(), BinaryOp::SmoothMinimum);


}

fn for_each_added_binary_directional(a: f64, b: f64, mut emit: impl FnMut(f64, BinaryOp)) {
    if a.fract() == 0.0 && (0.0..=10.0).contains(&a) && b.abs() <= 0.5 {
        if let Some(v) = polylog_value(a, b) { emit(v, BinaryOp::Polylog); }
    }
    if let Some(v) = sigma_k_value(a, b) { emit(v, BinaryOp::SigmaK); }
    if let Some(v) = jordan_totient_value(a, b) { emit(v, BinaryOp::JordanTotient); }
    if let Some(v) = multiplicative_order_value(a, b) { emit(v, BinaryOp::MultiplicativeOrder); }
    if let Some(v) = stirling_first_value(a, b) { emit(v, BinaryOp::StirlingFirst); }
    if let Some(v) = stirling_second_value(a, b) { emit(v, BinaryOp::StirlingSecond); }
    if let Some(v) = eulerian_value(a, b) { emit(v, BinaryOp::Eulerian); }
    if let (Some(ai), Some(bi)) = (integer_argument(a), integer_argument(b)) {
        if ai.abs() <= 1_000_000 && bi.abs() <= 1_000_000 {
            if let Some(v) = legendre_symbol_value(ai, bi) { emit(v, BinaryOp::LegendreSymbol); }
            if let Some(v) = jacobi_symbol_value(ai, bi) { emit(v, BinaryOp::JacobiSymbol); }
            if let Some(v) = kronecker_symbol_value(ai, bi) { emit(v, BinaryOp::KroneckerSymbol); }
        }
    }
    if b != 0.0 { emit((a / b).round() * b, BinaryOp::RoundTo); }
    if b != 0.0 {
        let quotient = (a / b).trunc();
        let trunc_remainder = a - quotient * b;
        emit(a % b, BinaryOp::Fmod);
        emit(quotient, BinaryOp::Quotient);
        emit(quotient, BinaryOp::DivRemQuotient);
        emit(trunc_remainder, BinaryOp::DivRemRemainder);
        let nearest_quotient = round_ties_even(a / b);
        emit(a - nearest_quotient * b, BinaryOp::Remainder);
    }
    if let Some((g, x, y)) = extended_gcd_values(a, b) {
        emit(g, BinaryOp::ExtendedGcdGcd);
        emit(x, BinaryOp::ExtendedGcdX);
        emit(y, BinaryOp::ExtendedGcdY);
    }
    if let Some(result) = nth_root(a, b) { emit(result, BinaryOp::IntegerRoot); }
    if a != 0.0 || b >= 0.0 {
        let result = a.signum() * a.abs().powf(b);
        emit(result, BinaryOp::SignedPow);
    }
    if let Some(result) = lcm_value(a, b) { emit(result, BinaryOp::SmallestCommonMultiple); }
    if let Some(result) = combinations(a, b) {
        emit(result, BinaryOp::Binomial);
        emit(result, BinaryOp::Choose);
    }
    if let Some(result) = permutations(a, b) { emit(result, BinaryOp::Permute); }
    let max2 = a.max(b);
    emit(max2 + (2.0_f64.powf(a - max2) + 2.0_f64.powf(b - max2)).log2(), BinaryOp::LogAddExp2);
    if a > b { emit(a + (-(b - a).exp()).ln_1p(), BinaryOp::LogSubExp); }
    if let Some(result) = hurwitz_zeta_value(a, b) { emit(result, BinaryOp::HurwitzZeta); }
    if let Some((p, q)) = gamma_pq(a, b) {
        if let Some(gamma) = gamma_function(a) { emit(p * gamma, BinaryOp::IncompleteGamma); }
        emit(p, BinaryOp::RegularizedGammaP);
        emit(q, BinaryOp::RegularizedGammaQ);
    }

    if let (Some(ai), Some(bi)) = (integer_argument(a), integer_argument(b)) {
        if (0..64).contains(&bi) {
            let shift = bi as u32;
            let ua = ai as u64;
            emit(ua.wrapping_shl(shift) as i64 as f64, BinaryOp::ShiftLeft);
            emit((ai >> shift) as f64, BinaryOp::ShiftRight);
            emit(ua.rotate_left(shift) as f64, BinaryOp::RotateLeft);
            emit(ua.rotate_right(shift) as f64, BinaryOp::RotateRight);
            emit(if (ua & (1_u64 << shift)) != 0 { 1.0 } else { 0.0 }, BinaryOp::BitTest);
            emit((ua | (1_u64 << shift)) as i64 as f64, BinaryOp::SetBit);
            emit((ua & !(1_u64 << shift)) as i64 as f64, BinaryOp::ClearBit);
        }
    }
    emit(if a == b { 1.0 } else { 0.0 }, BinaryOp::Equal);
    emit(if a != b { 1.0 } else { 0.0 }, BinaryOp::NotEqual);
    emit(if a < b { 1.0 } else { 0.0 }, BinaryOp::LessThan);
    emit(if a > b { 1.0 } else { 0.0 }, BinaryOp::GreaterThan);
    emit(if a <= b { 1.0 } else { 0.0 }, BinaryOp::LessEqual);
    emit(if a >= b { 1.0 } else { 0.0 }, BinaryOp::GreaterEqual);

    if let Some(v) = exponential_pdf_value(a, b) { emit(v, BinaryOp::ExponentialPdf); }
    if let Some(v) = exponential_cdf_value(a, b) { emit(v, BinaryOp::ExponentialCdf); }
    if let Some(v) = poisson_pmf_value(a, b) { emit(v, BinaryOp::PoissonPmf); }
    if let Some(v) = poisson_cdf_value(a, b) { emit(v, BinaryOp::PoissonCdf); }
    if let Some(v) = student_t_pdf_value(a, b) { emit(v, BinaryOp::StudentTPdf); }
    if let Some(v) = student_t_cdf_value(a, b) { emit(v, BinaryOp::StudentTCdf); }
    if let Some(v) = chi_squared_pdf_value(a, b) { emit(v, BinaryOp::ChiSquaredPdf); }
    if let Some(v) = chi_squared_cdf_value(a, b) { emit(v, BinaryOp::ChiSquaredCdf); }
}

fn for_each_extra_binary_directional(a: f64, b: f64, mut emit: impl FnMut(f64, BinaryOp)) {
    if let (Some(ai), Some(bi)) = (integer_argument(a), integer_argument(b)) {
        if bi != 0 {
            emit(if ai % bi == 0 { 1.0 } else { 0.0 }, BinaryOp::IsDivisible);
        }
        if let Some(result) = modular_inverse_value(a, b) {
            emit(result, BinaryOp::ModularInverse);
        }
        if let Some(result) = is_power_of_value(a, b) {
            emit(result, BinaryOp::IsPowerOf);
        }
    }
    emit(next_after_value(a, b), BinaryOp::NextAfter);
}

fn cached_added_binary_results(a: f64, b: f64) -> CachedBinaryResults {
    let key = (a.to_bits(), b.to_bits());
    ADDED_BINARY_RESULT_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some(results) = cache.get(&key) {
            return Arc::clone(results);
        }

        let mut generated = Vec::with_capacity(20);
        for_each_added_binary_directional(a, b, |value, op| generated.push((value, op, false)));
        for_each_added_binary_directional(b, a, |value, op| generated.push((value, op, true)));

        let results: CachedBinaryResults = generated.into();
        if cache.len() < THREAD_CACHE_MAX_ENTRIES {
            cache.insert(key, Arc::clone(&results));
        }
        results
    })
}

fn heuristic_search_for_order(
    digits: &[u8],
    target: f64,
    tolerance: f64,
    depth: usize,
    candidate_limit: usize,
    fold_from_right: bool,
) -> Option<(f64, f64, f64, usize, String)> {
    let max_chunks = (depth + 1).min(digits.len());
    let mut best_overall: Option<(f64, f64, f64, usize, String)> = None;

    for chunk_count in 1..=max_chunks {
        let base_len = digits.len() / chunk_count;
        let extra = digits.len() % chunk_count;
        let mut chunks = Vec::with_capacity(chunk_count);
        let mut start = 0usize;
        for index in 0..chunk_count {
            let len = base_len + (index < extra) as usize;
            chunks.push((start, len));
            start += len;
        }
        if fold_from_right {
            chunks.reverse();
        }

        let mut nodes = Vec::<ExprNode>::new();
        let (first_start, first_len) = chunks[0];
        let mut current = Cell::new_diverse(candidate_limit);
        insert_range_options(
            &mut current,
            &mut nodes,
            target,
            digits,
            first_start,
            first_len,
        );

        let mut valid = !current.candidates.is_empty();
        for &(chunk_start, chunk_len) in chunks.iter().skip(1) {
            if !valid {
                break;
            }
            let mut chunk_options = Cell::new(chunk_len + 1);
            insert_range_options(
                &mut chunk_options,
                &mut nodes,
                target,
                digits,
                chunk_start,
                chunk_len,
            );
            if chunk_options.candidates.is_empty() {
                valid = false;
                break;
            }

            let mut next = Cell::new_diverse(candidate_limit);
            for current_candidate in current.candidates.iter().copied() {
                for chunk_candidate in chunk_options.candidates.iter().copied() {
                    if fold_from_right {
                        generate_long_binary_results(
                            &mut next,
                            &mut nodes,
                            target,
                            chunk_candidate,
                            current_candidate,
                        );
                    } else {
                        generate_long_binary_results(
                            &mut next,
                            &mut nodes,
                            target,
                            current_candidate,
                            chunk_candidate,
                        );
                    }
                }
            }
            current = next;
            valid = !current.candidates.is_empty();
        }
        if !valid {
            continue;
        }

        let binary_cost = chunk_count - 1;
        let mut best_for_chunks: Option<(f64, f64, f64, usize, String)> = None;
        for candidate in current.candidates.iter().copied() {
            consider_existing_long_best(
                &mut best_for_chunks,
                candidate,
                binary_cost,
                target,
                &nodes,
                digits,
            );
        }

        if binary_cost < depth {
            for candidate in current.candidates.iter().copied() {
                generate_long_unary_best(
                    &mut best_for_chunks,
                    &mut nodes,
                    digits,
                    target,
                    candidate,
                    binary_cost + 1,
                );
            }
        }

        if let Some((score, value, imag, cost, expression)) = best_for_chunks {
            let should_replace = best_overall.as_ref().map_or(true, |(old_score, _, _, old_cost, _)| {
                score < *old_score || (score == *old_score && cost < *old_cost)
            });
            if should_replace {
                best_overall = Some((score, value, imag, cost, expression));
            }
            if score <= tolerance {
                break;
            }
        }
    }

    best_overall
}

fn run_long_input_search(
    digits: &[u8],
    target: f64,
    tolerance: f64,
    depth: usize,
    candidate_limit: usize,
    worker_count: usize,
) {
    let mut orderings = Vec::<Vec<u8>>::new();
    orderings.push(digits.to_vec());
    let mut reversed = digits.to_vec();
    reversed.reverse();
    orderings.push(reversed);
    let mut ascending = digits.to_vec();
    ascending.sort_unstable();
    orderings.push(ascending);
    let mut descending = digits.to_vec();
    descending.sort_unstable_by(|a, b| b.cmp(a));
    orderings.push(descending);

    for seed in [0x9E37_79B9_7F4A_7C15_u64, 0xD1B5_4A32_D192_ED03, 0x94D0_49BB_1331_11EB, 0xBF58_476D_1CE4_E5B9] {
        let mut shuffled = digits.to_vec();
        let mut state = seed ^ (digits.len() as u64);
        for index in (1..shuffled.len()).rev() {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let swap_index = (state as usize) % (index + 1);
            shuffled.swap(index, swap_index);
        }
        orderings.push(shuffled);
    }
    orderings.dedup();

    let mut tasks = Vec::with_capacity(orderings.len() * 2);
    for ordering in orderings {
        tasks.push((ordering.clone(), false));
        tasks.push((ordering, true));
    }

    let workers = worker_count.min(tasks.len()).max(1);
    let chunk_size = (tasks.len() + workers - 1) / workers;
    let partial_results = std::thread::scope(|scope| {
        let mut handles = Vec::with_capacity(workers);
        for chunk in tasks.chunks(chunk_size) {
            handles.push(scope.spawn(move || {
                let mut local_best: Option<(f64, f64, f64, usize, String)> = None;
                for (ordering, fold_from_right) in chunk {
                    if let Some(candidate) = heuristic_search_for_order(
                        ordering,
                        target,
                        tolerance,
                        depth,
                        candidate_limit,
                        *fold_from_right,
                    ) {
                        let should_replace = local_best.as_ref().map_or(true, |(old_score, _, _, old_cost, _)| {
                            candidate.0 < *old_score
                                || (candidate.0 == *old_score && candidate.3 < *old_cost)
                        });
                        if should_replace {
                            local_best = Some(candidate);
                        }
                    }
                    if local_best.as_ref().is_some_and(|(score, _, _, _, _)| *score <= tolerance) {
                        break;
                    }
                }
                local_best
            }));
        }
        handles
            .into_iter()
            .map(|handle| handle.join().expect("long-input search worker panicked"))
            .collect::<Vec<_>>()
    });

    let mut best: Option<(f64, f64, f64, usize, String)> = None;
    for candidate in partial_results.into_iter().flatten() {
        let should_replace = best.as_ref().map_or(true, |(old_score, _, _, old_cost, _)| {
            candidate.0 < *old_score
                || (candidate.0 == *old_score && candidate.3 < *old_cost)
        });
        if should_replace {
            best = Some(candidate);
        }
    }

    println!(
        "Input exceeds {MAX_EXACT_DIGITS} digits; used heuristic search (not exhaustive)."
    );
    println!("Candidates per beam: {candidate_limit}; diverse magnitude buckets and alternate digit orders/folds enabled.");
    match best {
        Some((score, value, imag, cost, expression)) if score <= tolerance => {
            println!("Found a heuristic match using {cost} operation(s):");
            println!("{expression} = {}", complex_display(value, imag));
        }
        Some((score, value, imag, cost, expression)) => {
            println!("No exact match found in the candidates explored.");
            println!("Closest result ({cost} operations): {expression} = {} (difference: {score:.15})", complex_display(value, imag));
        }
        None => {
            println!("No valid expression survived the heuristic search.");
        }
    }
}

fn configured_worker_count() -> usize {
    let available = std::thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(1);
    std::env::var("MATHBRUTE_THREADS")
        .ok()
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|&count| (1..=256).contains(&count))
        .unwrap_or(available)
}

fn for_each_binary_pair_result(
    a: Candidate,
    b: Candidate,
    include_added: bool,
    gamma_cache: &mut HashMap<u64, Option<f64>>,
    mut emit: impl FnMut(f64, ExprNode),
) {
    if a.imag != 0.0 || b.imag != 0.0 {
        return;
    }
    let av = a.value;
    let bv = b.value;
    let ai = a.node_id;
    let bi = b.node_id;

    macro_rules! push {
        ($value:expr, $left:expr, $right:expr, $op:expr) => {
            emit($value, ExprNode::Binary($op, $left, $right));
        };
    }

    let sum = av + bv;
    let product = av * bv;
    let av_sq = av * av;
    let bv_sq = bv * bv;
    push!(sum, ai, bi, BinaryOp::Add);
    push!(av - bv, ai, bi, BinaryOp::Sub);
    push!(bv - av, bi, ai, BinaryOp::Sub);
    push!(product, ai, bi, BinaryOp::Mul);
    push!(av.min(bv), ai, bi, BinaryOp::Min);
    push!(av.max(bv), ai, bi, BinaryOp::Max);
    push!(av.hypot(bv), ai, bi, BinaryOp::Hypot);
    push!(av.atan2(bv), ai, bi, BinaryOp::Atan2);
    push!((av + bv) / 2.0, ai, bi, BinaryOp::Average);
    push!((av_sq + bv_sq).sqrt() / 2.0_f64.sqrt(), ai, bi, BinaryOp::Rms);
    push!((av - bv).abs(), ai, bi, BinaryOp::Distance);
    push!(av.abs().copysign(bv), ai, bi, BinaryOp::CopySign);

    if av >= 0.0 && bv >= 0.0 {
        push!((av * bv).sqrt(), ai, bi, BinaryOp::GeometricMean);
    }
    if sum != 0.0 {
        push!(2.0 * av * bv / sum, ai, bi, BinaryOp::HarmonicMean);
    }
    push!(av_sq + bv_sq, ai, bi, BinaryOp::SumSquares);
    push!(av_sq - bv_sq, ai, bi, BinaryOp::DifferenceSquares);
    if let Some(result) = beta_value_cached(av, bv, gamma_cache) {
        push!(result, ai, bi, BinaryOp::Beta);
    }
    if let Some(result) = logarithmic_mean(av, bv) {
        push!(result, ai, bi, BinaryOp::LogarithmicMean);
    }
    if let Some(result) = arithmetic_geometric_mean(av, bv) {
        push!(result, ai, bi, BinaryOp::ArithmeticGeometricMean);
    }
    if let (Some(a_int), Some(b_int)) = (integer_argument(av), integer_argument(bv)) {
        push!(if gcd_i64(a_int, b_int) == 1 { 1.0 } else { 0.0 }, ai, bi, BinaryOp::IsCoprime);
    }
    if let Some(result) = polygonal_value(av, bv) {
        push!(result, ai, bi, BinaryOp::Polygonal);
    }

    let max_ab = av.max(bv);
    push!(max_ab + (-(av - bv).abs()).exp().ln_1p(), ai, bi, BinaryOp::LogAddExp);
    if av == 0.0 {
        push!(0.0, ai, bi, BinaryOp::XLogY);
    } else if bv > 0.0 {
        push!(av * bv.ln(), ai, bi, BinaryOp::XLogY);
    }

    if bv != 0.0 {
        push!((av / bv).floor(), ai, bi, BinaryOp::FloorDiv);
        push!(av.rem_euclid(bv), ai, bi, BinaryOp::EuclideanMod);
        push!(av / bv, ai, bi, BinaryOp::Div);
        push!(av % bv, ai, bi, BinaryOp::Mod);
    }
    if av != 0.0 {
        push!(bv / av, bi, ai, BinaryOp::Div);
        push!(bv % av, bi, ai, BinaryOp::Mod);
    }
    if bv.abs() <= 12.0 {
        push!(av.powf(bv), ai, bi, BinaryOp::Pow);
    }
    if av.abs() <= 12.0 {
        push!(bv.powf(av), bi, ai, BinaryOp::Pow);
    }
    if av > 0.0 && bv > 0.0 && bv != 1.0 {
        push!(av.log(bv), ai, bi, BinaryOp::LogBase);
    }
    if let Some(result) = nth_root(av, bv) {
        push!(result, ai, bi, BinaryOp::Root);
    }
    if let Some(result) = gcd_value(av, bv) {
        push!(result, ai, bi, BinaryOp::Gcd);
    }
    if let Some(result) = lcm_value(av, bv) {
        push!(result, ai, bi, BinaryOp::Lcm);
    }
    if let Some(result) = combinations(av, bv) {
        push!(result, ai, bi, BinaryOp::NCr);
    }
    if let Some(result) = permutations(av, bv) {
        push!(result, ai, bi, BinaryOp::NPr);
    }

    for_each_extra_binary(av, bv, |result, op| {
        emit(result, ExprNode::Binary(op, ai, bi));
    });
    for_each_extra_binary_directional(av, bv, |result, op| {
        emit(result, ExprNode::Binary(op, ai, bi));
    });
    for_each_extra_binary_directional(bv, av, |result, op| {
        emit(result, ExprNode::Binary(op, bi, ai));
    });

    if include_added {
        for &(result, op, swapped) in cached_added_binary_results(av, bv).iter() {
            let (left, right) = if swapped { (bi, ai) } else { (ai, bi) };
            emit(result, ExprNode::Binary(op, left, right));
        }
    }
}


fn for_each_complex_binary_pair_result(
    a: Candidate,
    b: Candidate,
    mut emit: impl FnMut(Complex, ComplexBinaryOp),
) {
    let left = Complex::new(a.value, a.imag);
    let right = Complex::new(b.value, b.imag);
    if left.im != 0.0 || right.im != 0.0 {
        let add = Complex::new(left.re + right.re, left.im + right.im);
        let sub = Complex::new(left.re - right.re, left.im - right.im);
        let mul = Complex::new(
            left.re * right.re - left.im * right.im,
            left.re * right.im + left.im * right.re,
        );
        emit(add, ComplexBinaryOp::Add);
        emit(sub, ComplexBinaryOp::Subtract);
        emit(mul, ComplexBinaryOp::Multiply);
        if let Some(value) = left.div(right) {
            emit(value, ComplexBinaryOp::Divide);
        }
    }
    if let Some(value) = left.pow(right) {
        emit(value, ComplexBinaryOp::Power);
    }
    if let (Some(left_log), Some(right_log)) = (left.ln(), right.ln()) {
        if let Some(value) = left_log.div(right_log) {
            emit(value, ComplexBinaryOp::Logarithm);
        }
    }
}


struct MaskWork {
    mask: usize,
    full_mask: usize,
    cost: usize,
    node_base: usize,
    target: f64,
    tolerance: f64,
    cells: Vec<Cell>,
    nodes: Vec<ExprNode>,
    found_solution: bool,
}

impl MaskWork {
    fn new(
        mask: usize,
        full_mask: usize,
        cost: usize,
        target: f64,
        tolerance: f64,
        candidate_limit: usize,
        node_base: usize,
    ) -> Self {
        let limit = (candidate_limit / (cost + 1)).max(1);
        Self {
            mask,
            full_mask,
            cost,
            node_base,
            target,
            tolerance,
            cells: vec![Cell::new(limit)],
            nodes: Vec::with_capacity(limit),
            found_solution: false,
        }
    }

    #[inline]
    fn insert(&mut self, _cost: usize, value: f64, node: ExprNode) {
        if let Some(score) = insert_into_cell_with_base(
            &mut self.cells[0],
            &mut self.nodes,
            self.target,
            value,
            node,
            self.node_base,
        ) {
            if self.mask == self.full_mask && score <= self.tolerance {
                self.found_solution = true;
            }
        }
    }

    #[inline]
    fn insert_complex(&mut self, _cost: usize, value: Complex, node: ExprNode) {
        if let Some(score) = insert_complex_into_cell_with_base(
            &mut self.cells[0],
            &mut self.nodes,
            self.target,
            value.re,
            value.im,
            node,
            self.node_base,
        ) {
            if self.mask == self.full_mask && score <= self.tolerance {
                self.found_solution = true;
            }
        }
    }

    #[inline]
    fn unary(&mut self, cost: usize, value: f64, child: usize, op: UnaryOp) {
        self.insert(cost, value, ExprNode::Unary(op, child));
    }

    #[inline]
    fn complex_unary(&mut self, cost: usize, value: Complex, child: usize, op: ComplexUnaryOp) {
        self.insert_complex(cost, value, ExprNode::ComplexUnary(op, child));
    }

    #[inline]
    fn complex_binary(&mut self, cost: usize, value: Complex, left: usize, right: usize, op: ComplexBinaryOp) {
        self.insert_complex(cost, value, ExprNode::ComplexBinary(op, left, right));
    }

    #[inline]
    fn ternary(
        &mut self,
        cost: usize,
        value: f64,
        first: usize,
        second: usize,
        third: usize,
        op: TernaryOp,
    ) {
        self.insert(cost, value, ExprNode::Ternary(op, first, second, third));
    }

    #[inline]
    fn quaternary(
        &mut self,
        cost: usize,
        value: f64,
        first: usize,
        second: usize,
        third: usize,
        fourth: usize,
        op: QuaternaryOp,
    ) {
        self.insert(
            cost,
            value,
            ExprNode::Quaternary(op, first, second, third, fourth),
        );
    }

    #[inline]
    fn quinary(
        &mut self,
        cost: usize,
        value: f64,
        first: usize,
        second: usize,
        third: usize,
        fourth: usize,
        fifth: usize,
        op: QuinaryOp,
    ) {
        self.insert(
            cost,
            value,
            ExprNode::Quinary(op, first, second, third, fourth, fifth),
        );
    }
}

fn insert_number_permutations_local(work: &mut MaskWork, digits: &[u8], mask: usize) {
    fn recurse(
        work: &mut MaskWork,
        digits: &[u8],
        mask: usize,
        remaining: usize,
        packed_digits: u32,
        len: u8,
    ) {
        if remaining == 0 {
            let mut raw_value = 0.0_f64;
            for index in 0..len as usize {
                let shift = (len as usize - 1 - index) * 4;
                let digit = ((packed_digits >> shift) & 0xF) as u8;
                raw_value = raw_value * 10.0 + digit as f64;
            }

            for decimal_places in 0..=len {
                let value = raw_value / 10.0_f64.powi(decimal_places as i32);
                work.insert(
                    0,
                    value,
                    ExprNode::Number {
                        packed_digits,
                        len,
                        decimal_places,
                    },
                );

                if work.mask == work.full_mask && work.found_solution {
                    return;
                }
            }
            return;
        }

        let mut bits_left = remaining;
        let mut used_digit_values = 0u16;
        while bits_left != 0 {
            let bit = bits_left & bits_left.wrapping_neg();
            bits_left ^= bit;

            let digit_index = bit.trailing_zeros() as usize;
            let digit = digits[digit_index];
            let digit_flag = 1u16 << digit;

            if used_digit_values & digit_flag != 0 {
                continue;
            }
            used_digit_values |= digit_flag;

            recurse(
                work,
                digits,
                mask,
                remaining ^ bit,
                (packed_digits << 4) | digit as u32,
                len + 1,
            );

            if work.mask == work.full_mask && work.found_solution {
                return;
            }
        }
    }

    recurse(work, digits, mask, mask, 0, 0);
}

fn remap_expr_node(node: ExprNode, old_base: usize, new_base: usize) -> ExprNode {
    #[inline]
    fn remap(id: usize, old_base: usize, new_base: usize) -> usize {
        if id >= old_base {
            new_base + (id - old_base)
        } else {
            id
        }
    }

    match node {
        ExprNode::Number { .. } | ExprNode::NumberRange { .. } => node,
        ExprNode::Unary(op, child) => ExprNode::Unary(op, remap(child, old_base, new_base)),
        ExprNode::Binary(op, left, right) => ExprNode::Binary(
            op,
            remap(left, old_base, new_base),
            remap(right, old_base, new_base),
        ),
        ExprNode::Ternary(op, first, second, third) => ExprNode::Ternary(
            op,
            remap(first, old_base, new_base),
            remap(second, old_base, new_base),
            remap(third, old_base, new_base),
        ),
        ExprNode::Quaternary(op, first, second, third, fourth) => ExprNode::Quaternary(
            op,
            remap(first, old_base, new_base),
            remap(second, old_base, new_base),
            remap(third, old_base, new_base),
            remap(fourth, old_base, new_base),
        ),
        ExprNode::Quinary(op, first, second, third, fourth, fifth) => ExprNode::Quinary(
            op,
            remap(first, old_base, new_base),
            remap(second, old_base, new_base),
            remap(third, old_base, new_base),
            remap(fourth, old_base, new_base),
            remap(fifth, old_base, new_base),
        ),
        ExprNode::ComplexUnary(op, child) => ExprNode::ComplexUnary(op, remap(child, old_base, new_base)),
        ExprNode::ComplexBinary(op, left, right) => ExprNode::ComplexBinary(
            op, remap(left, old_base, new_base), remap(right, old_base, new_base)
        ),
    }
}

fn merge_mask_work(search: &mut Search, mut work: MaskWork) {
    let new_base = search.nodes.len();

    for node in work.nodes.drain(..) {
        search.nodes.push(remap_expr_node(node, work.node_base, new_base));
    }

    for candidate in &mut work.cells[0].candidates {
        candidate.node_id = new_base + (candidate.node_id - work.node_base);
    }

    if work.found_solution {
        search.found_solution = true;
    }

    let cell = work.cells.pop().expect("mask work cell missing");
    let mask_cells = &mut search.dp[work.mask];
    if mask_cells.len() == work.cost {
        mask_cells.push(cell);
    } else if mask_cells.len() > work.cost {
        mask_cells[work.cost] = cell;
    } else {
        panic!("DP cost wave arrived out of order for mask {}: expected {}, got {}", work.mask, mask_cells.len(), work.cost);
    }
}

fn make_binary_partitions(mask: usize) -> Vec<(usize, usize)> {
    let mut partitions = Vec::new();
    let mut left_mask = (mask - 1) & mask;
    while left_mask > 0 {
        let right_mask = mask ^ left_mask;
        if right_mask > 0 && left_mask < right_mask {
            partitions.push((left_mask, right_mask));
        }
        left_mask = (left_mask - 1) & mask;
    }
    partitions
}

fn process_binary_partitions(
    work: &mut MaskWork,
    dp: &[Vec<Cell>],
    partitions: &[(usize, usize)],
    cost: usize,
    target: f64,
    worker_count: usize,
    gamma_cache: &mut HashMap<u64, Option<f64>>,
    partition_progress: &AtomicUsize,
) {
    if partitions.is_empty() {
        return;
    }

    if worker_count <= 1 {
        for &(left_mask, right_mask) in partitions {
            for left_cost in 0..cost {
                let right_cost = cost - 1 - left_cost;
                let left_candidates = &dp[left_mask][left_cost].candidates;
                let right_candidates = &dp[right_mask][right_cost].candidates;
                if left_candidates.is_empty() || right_candidates.is_empty() {
                    continue;
                }
                for &a in left_candidates {
                    for &b in right_candidates {
                        for_each_binary_pair_result(
                            a,
                            b,
                            cost == 1,
                            gamma_cache,
                            |value, node| work.insert(cost, value, node),
                        );
                        for_each_complex_binary_pair_result(a, b, |value, op| {
                            work.complex_binary(cost, value, a.node_id, b.node_id, op);
                        });
                        if work.found_solution {
                            return;
                        }
                    }
                }
            }
            partition_progress.fetch_add(1, Ordering::Relaxed);
        }
        return;
    }

    let mut pair_count = 0usize;
    for &(left_mask, right_mask) in partitions {
        for left_cost in 0..cost {
            let right_cost = cost - 1 - left_cost;
            pair_count = pair_count.saturating_add(
                dp[left_mask][left_cost].candidates.len()
                    .saturating_mul(dp[right_mask][right_cost].candidates.len()),
            );
        }
    }

    let parallel_threshold = worker_count.saturating_mul(256).max(1024);
    let use_parallel = worker_count > 1
        && partitions.len() >= worker_count * 2
        && pair_count >= parallel_threshold;

    if !use_parallel {
        for &(left_mask, right_mask) in partitions {
            for left_cost in 0..cost {
                let right_cost = cost - 1 - left_cost;
                let left_candidates = &dp[left_mask][left_cost].candidates;
                let right_candidates = &dp[right_mask][right_cost].candidates;
                for &a in left_candidates {
                    for &b in right_candidates {
                        for_each_binary_pair_result(
                            a,
                            b,
                            cost == 1,
                            gamma_cache,
                            |value, node| work.insert(cost, value, node),
                        );
                        for_each_complex_binary_pair_result(a, b, |value, op| {
                            work.complex_binary(cost, value, a.node_id, b.node_id, op);
                        });
                        if work.found_solution {
                            return;
                        }
                    }
                }
            }
            partition_progress.fetch_add(1, Ordering::Relaxed);
        }
        return;
    }

    let workers = worker_count.min(partitions.len()).max(1);
    let chunk_size = (partitions.len() + workers - 1) / workers;
    let cell_limit = work.cells[0].limit;
    let partials = std::thread::scope(|scope| {
        let mut handles = Vec::with_capacity(workers);
        for chunk in partitions.chunks(chunk_size) {
            let dp_ref = dp;
            let progress_ref = partition_progress;
            handles.push(scope.spawn(move || {
                let mut local_cell = Cell::new(cell_limit);
                let mut local_nodes = Vec::<ExprNode>::new();
                let mut local_gamma_cache = HashMap::<u64, Option<f64>>::new();

                for &(left_mask, right_mask) in chunk {
                    for left_cost in 0..cost {
                        let right_cost = cost - 1 - left_cost;
                        let left_candidates = &dp_ref[left_mask][left_cost].candidates;
                        let right_candidates = &dp_ref[right_mask][right_cost].candidates;
                        for &a in left_candidates {
                            for &b in right_candidates {
                                for_each_binary_pair_result(
                                    a,
                                    b,
                                    cost == 1,
                                    &mut local_gamma_cache,
                                    |value, node| {
                                        insert_into_cell(
                                            &mut local_cell,
                                            &mut local_nodes,
                                            target,
                                            value,
                                            node,
                                        );
                                    },
                                );
                                for_each_complex_binary_pair_result(a, b, |value, op| {
                                    insert_complex_into_cell(
                                        &mut local_cell,
                                        &mut local_nodes,
                                        target,
                                        value,
                                        ExprNode::ComplexBinary(op, a.node_id, b.node_id),
                                    );
                                });
                            }
                        }
                    }
                    progress_ref.fetch_add(1, Ordering::Relaxed);
                }

                (local_cell, local_nodes)
            }));
        }
        handles
            .into_iter()
            .map(|handle| handle.join().expect("binary partition worker panicked"))
            .collect::<Vec<_>>()
    });
    for (local_cell, local_nodes) in partials {
        for candidate in local_cell.candidates {
            let node = local_nodes[candidate.node_id];
            work.insert_complex(cost, Complex::new(candidate.value, candidate.imag), node);
        }
    }
}

fn process_mask_cost(
    dp: &[Vec<Cell>],
    digits: &[u8],
    mask: usize,
    cost: usize,
    binary_partitions: &[(usize, usize)],
    target: f64,
    tolerance: f64,
    candidate_limit: usize,
    full_mask: usize,
    node_base: usize,
    partition_workers: usize,
    gamma_cache: &mut HashMap<u64, Option<f64>>,
    integer_facts_cache: &mut HashMap<i64, IntegerFacts>,
    partition_progress: &AtomicUsize,
) -> MaskWork {
    let mut work = MaskWork::new(
        mask,
        full_mask,
        cost,
        target,
        tolerance,
        candidate_limit,
        node_base,
    );

    if cost == 0 {
        insert_number_permutations_local(&mut work, digits, mask);
        return work;
    }

    let digit_count = mask.count_ones();
    let mut previous_values = Vec::with_capacity(candidate_limit);
        previous_values.extend(dp[mask][cost - 1].candidates.iter().copied());
        for candidate in previous_values.iter().copied() {
            let value = candidate.value;
            let child = candidate.node_id;
            let complex_value = Complex::new(candidate.value, candidate.imag);

            for_each_complex_unary_result(complex_value, |result, op| {
                work.complex_unary(cost, result, child, op);
            });
            if candidate.imag != 0.0 {
                continue;
            }

            work.unary(cost, value.cbrt(), child, UnaryOp::Cbrt);
            work.unary(cost, value.ceil(), child, UnaryOp::Ceil);
            work.unary(cost, value.floor(), child, UnaryOp::Floor);
            work.unary(cost, value.round(), child, UnaryOp::Round);
            work.unary(cost, value.trunc(), child, UnaryOp::Trunc);
            work.unary(cost, value.fract(), child, UnaryOp::Fract);
            work.unary(cost, value.abs(), child, UnaryOp::Abs);
            work.unary(cost, -value, child, UnaryOp::Neg);
            work.unary(cost, value * value, child, UnaryOp::Square);
            work.unary(cost, value * value * value, child, UnaryOp::Cube);

            if value != 0.0 {
                work.unary(cost, 1.0 / value, child, UnaryOp::Reciprocal);
            }

            work.unary(cost, value.sin(), child, UnaryOp::Sin);
            work.unary(cost, value.cos(), child, UnaryOp::Cos);
            work.unary(cost, value.tan(), child, UnaryOp::Tan);
            work.unary(cost, value.atan(), child, UnaryOp::Atan);
            work.unary(cost, value.tanh(), child, UnaryOp::Tanh);
            work.unary(cost, value.asinh(), child, UnaryOp::Asinh);
            if value.abs() <= 28.0 {
                work.unary(cost, value.sinh(), child, UnaryOp::Sinh);
                work.unary(cost, value.cosh(), child, UnaryOp::Cosh);
            }

            if value.abs() <= 1.0 {
                work.unary(cost, value.asin(), child, UnaryOp::Asin);
                work.unary(cost, value.acos(), child, UnaryOp::Acos);
            }
            if value >= 1.0 {
                work.unary(cost, value.acosh(), child, UnaryOp::Acosh);
            }
            if value > -1.0 && value < 1.0 {
                work.unary(cost, value.atanh(), child, UnaryOp::Atanh);
            }
            if value <= 28.0 {
                work.unary(cost, value.exp(), child, UnaryOp::Exp);
                work.unary(cost, value.exp_m1(), child, UnaryOp::Expm1);
            }

            work.unary(cost, value.to_degrees(), child, UnaryOp::Degrees);
            work.unary(cost, value.to_radians(), child, UnaryOp::Radians);
            work.unary(cost, value / 100.0, child, UnaryOp::Percent);
            if value <= 12.0 && value >= -1074.0 {
                work.unary(cost, 10.0_f64.powf(value), child, UnaryOp::TenPow);
            }
            if value <= 39.0 && value >= -1074.0 {
                work.unary(cost, 2.0_f64.powf(value), child, UnaryOp::TwoPow);
            }

            if value > 0.0 {
                work.unary(cost, value.ln(), child, UnaryOp::Ln);
                work.unary(cost, value.log2(), child, UnaryOp::Log2);
                work.unary(cost, value.log10(), child, UnaryOp::Log10);
            }
            if value > -1.0 {
                work.unary(cost, value.ln_1p(), child, UnaryOp::Ln1p);
            }
            if value >= 0.0 {
                work.unary(cost, value.sqrt(), child, UnaryOp::Sqrt);
            }
            if let Some(result) = factorial(value) {
                work.unary(cost, result, child, UnaryOp::Factorial);
            }
            work.unary(
                cost,
                if value == 0.0 { 0.0 } else { value.signum() },
                child,
                UnaryOp::Signum,
            );
            work.unary(
                cost,
                if value < 0.0 { 0.0 } else if value == 0.0 { 0.5 } else { 1.0 },
                child,
                UnaryOp::Heaviside,
            );
            if value <= 39.0 && value >= -1074.0 {
                work.unary(cost, value.exp2(), child, UnaryOp::Exp2);
            }
            if let Some(result) = double_factorial(value) {
                work.unary(cost, result, child, UnaryOp::DoubleFactorial);
            }
            if let Some(result) = primorial(value) {
                work.unary(cost, result, child, UnaryOp::Primorial);
            }
            if let Some(result) = digit_sum(value) {
                work.unary(cost, result, child, UnaryOp::DigitSum);
            }
            if let Some(result) = digital_root(value) {
                work.unary(cost, result, child, UnaryOp::DigitalRoot);
            }
            let sinc = if value == 0.0 {
                1.0
            } else {
                let angle = std::f64::consts::PI * value;
                angle.sin() / angle
            };
            work.unary(cost, sinc, child, UnaryOp::Sinc);
            if (-5.0..=20.0).contains(&value) {
                if let Some(result) = gamma_cached(value, &mut *gamma_cache) {
                    work.unary(cost, result, child, UnaryOp::Gamma);
                }
            }
            work.unary(cost, logistic(value), child, UnaryOp::Logistic);
            work.unary(
                cost,
                value.max(0.0) + (-value.abs()).exp().ln_1p(),
                child,
                UnaryOp::Softplus,
            );

            let sin_value = value.sin();
            let cos_value = value.cos();
            if sin_value != 0.0 {
                work.unary(cost, 1.0 / sin_value, child, UnaryOp::Csc);
                work.unary(cost, cos_value / sin_value, child, UnaryOp::Cot);
            }
            if cos_value != 0.0 {
                work.unary(cost, 1.0 / cos_value, child, UnaryOp::Sec);
            }
            if value.abs() <= 28.0 {
                let sinh_value = value.sinh();
                let cosh_value = value.cosh();
                if cosh_value != 0.0 {
                    work.unary(cost, 1.0 / cosh_value, child, UnaryOp::Sech);
                }
                if sinh_value != 0.0 {
                    work.unary(cost, 1.0 / sinh_value, child, UnaryOp::Csch);
                    work.unary(cost, cosh_value / sinh_value, child, UnaryOp::Coth);
                }
            }
            if value > 0.0 && value < 1.0 {
                work.unary(cost, (value / (1.0 - value)).ln(), child, UnaryOp::Logit);
            }
            work.unary(cost, value / (1.0 + value.abs()), child, UnaryOp::Softsign);
            work.unary(cost, erf_approx(value), child, UnaryOp::Erf);
            work.unary(cost, erfc_approx(value), child, UnaryOp::Erfc);
            work.unary(
                cost,
                0.5 * (1.0 + erf_approx(value / std::f64::consts::SQRT_2)),
                child,
                UnaryOp::NormalCdf,
            );
            if let Some(result) = fibonacci_value(value) {
                work.unary(cost, result, child, UnaryOp::Fibonacci);
            }
            if let Some(result) = triangular_value(value) {
                work.unary(cost, result, child, UnaryOp::Triangular);
            }
            if let Some(result) = catalan_value(value) {
                work.unary(cost, result, child, UnaryOp::Catalan);
            }
            let facts = if value.fract() == 0.0 && (0.0..=10_000.0).contains(&value) {
                let integer = value as i64;
                Some(*integer_facts_cache.entry(integer).or_insert_with(|| {
                    integer_facts(value).expect("integer_facts domain checked")
                }))
            } else {
                None
            };
            if let Some(facts) = facts {
                work.unary(cost, facts.is_prime, child, UnaryOp::IsPrime);
                if facts.totient.is_finite() {
                    work.unary(cost, facts.totient, child, UnaryOp::Totient);
                    work.unary(cost, facts.divisor_count, child, UnaryOp::DivisorCount);
                    work.unary(cost, facts.prime_omega, child, UnaryOp::PrimeOmega);
                    work.unary(cost, facts.mobius, child, UnaryOp::Mobius);
                    work.unary(cost, facts.is_perfect, child, UnaryOp::IsPerfectNumber);
                    work.unary(cost, facts.radical, child, UnaryOp::Radical);
                    work.unary(cost, facts.divisor_sum, child, UnaryOp::DivisorSum);
                    work.unary(cost, facts.liouville, child, UnaryOp::Liouville);
                    work.unary(cost, facts.carmichael_lambda, child, UnaryOp::CarmichaelLambda);
                }
            }
            if value.fract() == 0.0 && value.abs() <= MAX_INTEGER_ARGUMENT {
                let n = value.abs() as i64;
                work.unary(cost, n.count_ones() as f64, child, UnaryOp::Popcount);
                work.unary(cost, (64 - (n as u64).leading_zeros()) as f64, child, UnaryOp::BitLength);
            }
            if let Some(result) = digit_product(value) {
                work.unary(cost, result, child, UnaryOp::DigitProduct);
            }
            if let Some(result) = alternating_digit_sum(value) {
                work.unary(cost, result, child, UnaryOp::AlternatingDigitSum);
            }
            for_each_extra_unary(value, |result, op| {
                work.unary(cost, result, child, op);
            }, sin_value, cos_value, Some(&mut *gamma_cache));
            if cost == 1 {
                for_each_added_unary_cached(value, |result, op| {
                    work.unary(cost, result, child, op);
                });
            }
            for_each_chained_added_unary(value, |result, op| {
                work.unary(cost, result, child, op);
            });
        }

    if work.found_solution {
        return work;
    }

    if digit_count >= 2 {
        process_binary_partitions(
            &mut work,
            dp,
            binary_partitions,
            cost,
            target,
            partition_workers,
            gamma_cache,
            partition_progress,
        );
        if work.found_solution {
            return work;
        }
    }

        if mask == full_mask && (3..=9).contains(&digit_count) && (cost == 1 || cost == 2) {
            let mut first_mask = (mask - 1) & mask;
            while first_mask != 0 {
                let remainder = mask ^ first_mask;
                let mut second_mask = (remainder - 1) & remainder;
                while second_mask != 0 {
                    let third_mask = remainder ^ second_mask;
                    if third_mask != 0 {
                        for first_cost in 0..cost {
                            for second_cost in 0..(cost - first_cost) {
                                let third_cost = cost - 1 - first_cost - second_cost;
                                let first_values = best_candidates::<3>(&dp[first_mask][first_cost]);
                                let second_values = best_candidates::<3>(&dp[second_mask][second_cost]);
                                let third_values = best_candidates::<3>(&dp[third_mask][third_cost]);
                                for a in first_values.iter().copied().flatten() {
                                    for b in second_values.iter().copied().flatten() {
                                        for c in third_values.iter().copied().flatten() {
                                            if a.imag != 0.0 || b.imag != 0.0 || c.imag != 0.0 {
                                                continue;
                                            }
                                            let av = a.value;
                                            let bv = b.value;
                                            let cv = c.value;
                                            if let Some(v) = clamp_value(av, bv, cv) {
                                                work.ternary(cost, v, a.node_id, b.node_id, c.node_id, TernaryOp::Clamp);
                                            }
                                            let v = lerp_value(av, bv, cv);
                                            work.ternary(cost, v, a.node_id, b.node_id, c.node_id, TernaryOp::Lerp);
                                            if let Some(v) = smoothstep_value(av, bv, cv) {
                                                work.ternary(cost, v, a.node_id, b.node_id, c.node_id, TernaryOp::Smoothstep);
                                            }
                                            if let Some(v) = modular_power_value(av, bv, cv) {
                                                work.ternary(cost, v, a.node_id, b.node_id, c.node_id, TernaryOp::ModularPower);
                                            }
                                            if let Some(v) = binomial_cdf_value(av, bv, cv) {
                                                work.ternary(cost, v, a.node_id, b.node_id, c.node_id, TernaryOp::BinomialCdf);
                                            }
                                            let fma = av.mul_add(bv, cv);
                                            work.ternary(cost, fma, a.node_id, b.node_id, c.node_id, TernaryOp::Fma);
                                            work.ternary(cost, fma, a.node_id, b.node_id, c.node_id, TernaryOp::MulAdd);
                                            let mut triple = [av, bv, cv];
                                            triple.sort_by(f64::total_cmp);
                                            work.ternary(cost, triple[1], a.node_id, b.node_id, c.node_id, TernaryOp::Median3);
                                            work.ternary(cost, av.hypot(bv).hypot(cv), a.node_id, b.node_id, c.node_id, TernaryOp::Hypot3);
                                            if av != bv {
                                                work.ternary(cost, (cv - av) / (bv - av), a.node_id, b.node_id, c.node_id, TernaryOp::InverseLerp);
                                            }
                                            work.ternary(cost, if cv >= av.min(bv) && cv <= av.max(bv) { 1.0 } else { 0.0 }, a.node_id, b.node_id, c.node_id, TernaryOp::IsBetween);
                                            if let Some(v) = binomial_pmf_value(av, bv, cv) {
                                                work.ternary(cost, v, a.node_id, b.node_id, c.node_id, TernaryOp::BinomialPmf);
                                            }
                                            if let Some(v) = regularized_beta_value(av, bv, cv) {
                                                work.ternary(cost, v, a.node_id, b.node_id, c.node_id, TernaryOp::RegularizedBeta);
                                                if let Some(beta) = beta_value(bv, cv) {
                                                    work.ternary(cost, v * beta, a.node_id, b.node_id, c.node_id, TernaryOp::BetaInc);
                                                }
                                            }
                                            if let (Some(ai), Some(bi), Some(mi)) = (integer_argument(av), integer_argument(bv), integer_argument(cv)) {
                                                if mi > 0 {
                                                    let m = mi as i128;
                                                    work.ternary(cost, (ai as i128 + bi as i128).rem_euclid(m) as f64, a.node_id, b.node_id, c.node_id, TernaryOp::ModularAdd);
                                                    work.ternary(cost, (ai as i128 - bi as i128).rem_euclid(m) as f64, a.node_id, b.node_id, c.node_id, TernaryOp::ModularSub);
                                                    work.ternary(cost, ((ai as i128 * bi as i128).rem_euclid(m)) as f64, a.node_id, b.node_id, c.node_id, TernaryOp::ModularMul);
                                                }
                                            }
                                            if let Some(v) = multinomial_value(av, bv, cv) {
                                                work.ternary(cost, v, a.node_id, b.node_id, c.node_id, TernaryOp::Multinomial);
                                            }
                                            if let Some(v) = gamma_pdf_value(av, bv, cv) { work.ternary(cost, v, a.node_id, b.node_id, c.node_id, TernaryOp::GammaPdf); }
                                            if let Some(v) = gamma_cdf_value(av, bv, cv) { work.ternary(cost, v, a.node_id, b.node_id, c.node_id, TernaryOp::GammaCdf); }
                                            if let Some(v) = beta_pdf_value(av, bv, cv) { work.ternary(cost, v, a.node_id, b.node_id, c.node_id, TernaryOp::BetaPdf); }
                                            if let Some(v) = beta_cdf_value(av, bv, cv) { work.ternary(cost, v, a.node_id, b.node_id, c.node_id, TernaryOp::BetaCdf); }
                                            if let Some(v) = uniform_pdf_value(av, bv, cv) { work.ternary(cost, v, a.node_id, b.node_id, c.node_id, TernaryOp::UniformPdf); }
                                            if let Some(v) = uniform_cdf_value(av, bv, cv) { work.ternary(cost, v, a.node_id, b.node_id, c.node_id, TernaryOp::UniformCdf); }
                                            if work.found_solution { return work; }
                                        }
                                        if work.found_solution { return work; }
                                    }
                                    if work.found_solution { return work; }
                                }
                            }
                        }
                    }
                    second_mask = (second_mask - 1) & remainder;
                }
                first_mask = (first_mask - 1) & mask;
            }
        }

        if mask == full_mask && (4..=7).contains(&digit_count) && cost == 1 {
            let mut first_mask = (mask - 1) & mask;
            while first_mask != 0 {
                let remainder1 = mask ^ first_mask;
                let mut second_mask = (remainder1 - 1) & remainder1;
                while second_mask != 0 {
                    let remainder2 = remainder1 ^ second_mask;
                    let mut third_mask = (remainder2 - 1) & remainder2;
                    while third_mask != 0 {
                        let fourth_mask = remainder2 ^ third_mask;
                        if fourth_mask != 0 {
                            for first_cost in 0..cost {
                                for second_cost in 0..(cost - first_cost) {
                                    for third_cost in 0..(cost - first_cost - second_cost) {
                                        let fourth_cost = cost - 1 - first_cost - second_cost - third_cost;
                                        let avs = best_candidates::<2>(&dp[first_mask][first_cost]);
                                        let bvs = best_candidates::<2>(&dp[second_mask][second_cost]);
                                        let cvs = best_candidates::<2>(&dp[third_mask][third_cost]);
                                        let dvs = best_candidates::<2>(&dp[fourth_mask][fourth_cost]);
                                        for a in avs.iter().copied().flatten() {
                                            for b in bvs.iter().copied().flatten() {
                                                for c in cvs.iter().copied().flatten() {
                                                    for d in dvs.iter().copied().flatten() {
                                                        if a.imag != 0.0 || b.imag != 0.0 || c.imag != 0.0 || d.imag != 0.0 {
                                                            continue;
                                                        }
                                                        if let Some(v) = chinese_remainder_value(a.value, b.value, c.value, d.value) {
                                                            work.quaternary(cost, v, a.node_id, b.node_id, c.node_id, d.node_id, QuaternaryOp::ChineseRemainder);
                                                        }
                                                        let midpoint = (a.value + b.value + c.value + d.value) / 4.0;
                                                        work.quaternary(cost, midpoint, a.node_id, b.node_id, c.node_id, d.node_id, QuaternaryOp::Bilinear);
                                                        work.quaternary(cost, midpoint, a.node_id, b.node_id, c.node_id, d.node_id, QuaternaryOp::Multilinear);
                                                        let cubic_midpoint = (-a.value + 9.0 * b.value + 9.0 * c.value - d.value) / 16.0;
                                                        work.quaternary(cost, cubic_midpoint, a.node_id, b.node_id, c.node_id, d.node_id, QuaternaryOp::CubicInterp);
                                                        if work.found_solution { return work; }
                                                    }
                                                    if work.found_solution { return work; }
                                                }
                                                if work.found_solution { return work; }
                                            }
                                            if work.found_solution { return work; }
                                        }
                                    }
                                }
                            }
                        }
                        third_mask = (third_mask - 1) & remainder2;
                    }
                    second_mask = (second_mask - 1) & remainder1;
                }
                first_mask = (first_mask - 1) & mask;
            }
        }

        if mask == full_mask && (5..=8).contains(&digit_count) && cost == 1 {
            let mut first_mask = (mask - 1) & mask;
            while first_mask != 0 {
                let remainder1 = mask ^ first_mask;
                let mut second_mask = (remainder1 - 1) & remainder1;
                while second_mask != 0 {
                    let remainder2 = remainder1 ^ second_mask;
                    let mut third_mask = (remainder2 - 1) & remainder2;
                    while third_mask != 0 {
                        let remainder3 = remainder2 ^ third_mask;
                        let mut fourth_mask = (remainder3 - 1) & remainder3;
                        while fourth_mask != 0 {
                            let fifth_mask = remainder3 ^ fourth_mask;
                            if fifth_mask != 0 {
                                let first_values = best_candidates::<2>(&dp[first_mask][0]);
                                let second_values = best_candidates::<2>(&dp[second_mask][0]);
                                let third_values = best_candidates::<2>(&dp[third_mask][0]);
                                let fourth_values = best_candidates::<2>(&dp[fourth_mask][0]);
                                let fifth_values = best_candidates::<2>(&dp[fifth_mask][0]);
                                for a in first_values.iter().copied().flatten() {
                                    for b in second_values.iter().copied().flatten() {
                                        for c in third_values.iter().copied().flatten() {
                                            for d in fourth_values.iter().copied().flatten() {
                                                for e in fifth_values.iter().copied().flatten() {
                                                    if a.imag != 0.0 || b.imag != 0.0 || c.imag != 0.0 || d.imag != 0.0 || e.imag != 0.0 {
                                                        continue;
                                                    }
                                                    if let Some(v) = remap_value(a.value, b.value, c.value, d.value, e.value) {
                                                        work.quinary(cost, v, a.node_id, b.node_id, c.node_id, d.node_id, e.node_id, QuinaryOp::Remap);
                                                    }
                                                    if work.found_solution { return work; }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            fourth_mask = (fourth_mask - 1) & remainder3;
                        }
                        third_mask = (third_mask - 1) & remainder2;
                    }
                    second_mask = (second_mask - 1) & remainder1;
                }
                first_mask = (first_mask - 1) & mask;
            }
        }

        if mask == full_mask
            && work.cells[0]
                .candidates
                .iter()
                .any(|candidate| candidate.score <= tolerance)
        {
            return work;
        }

    work
}

fn process_cost_wave(
    search: &mut Search,
    digits: &[u8],
    binary_partitions_by_mask: &[Vec<(usize, usize)>],
    cost: usize,
    candidate_limit: usize,
    worker_count: usize,
) {
    let full_mask = search.full_mask;
    if full_mask == 0 {
        return;
    }

    let node_base = search.nodes.len();
    let target = search.target;
    let tolerance = search.tolerance;
    let workers = worker_count.min(full_mask.saturating_sub(1).max(1)).max(1);
    let partition_progress = AtomicUsize::new(0);
    let non_full_count = full_mask.saturating_sub(1);
    let next = AtomicUsize::new(0);
    let completed = AtomicUsize::new(0);
    let mut results = if non_full_count == 0 {
        Vec::new()
    } else if workers == 1 {
        let mut local_results = Vec::with_capacity(non_full_count);
        let mut gamma_cache = HashMap::<u64, Option<f64>>::new();
        let mut integer_facts_cache = HashMap::<i64, IntegerFacts>::new();
        for (index, mask) in (1..full_mask).enumerate() {
            eprint!("\rProcessing operation-cost {cost}: {}/{}", index, full_mask);
            let _ = io::stderr().flush();
            let work = process_mask_cost(
                &search.dp,
                digits,
                mask,
                cost,
                &binary_partitions_by_mask[mask],
                target,
                tolerance,
                candidate_limit,
                full_mask,
                node_base,
                1,
                &mut gamma_cache,
                &mut integer_facts_cache,
                &partition_progress,
            );
            local_results.push((mask, work));
        }
        local_results
    } else {
        std::thread::scope(|scope| {
            let mut handles = Vec::with_capacity(workers);
            for _ in 0..workers {
                let dp = &search.dp;
                let partitions_ref = binary_partitions_by_mask;
                let next_ref = &next;
                let completed_ref = &completed;
                let partition_progress_ref = &partition_progress;
                handles.push(scope.spawn(move || {
                    let mut local_results = Vec::with_capacity((non_full_count / workers).max(1));
                    let mut gamma_cache = HashMap::<u64, Option<f64>>::new();
                    let mut integer_facts_cache = HashMap::<i64, IntegerFacts>::new();
                    loop {
                        let index = next_ref.fetch_add(1, Ordering::Relaxed);
                        if index >= non_full_count {
                            break;
                        }
                        let mask = index + 1;
                        let work = process_mask_cost(
                            dp,
                            digits,
                            mask,
                            cost,
                            &partitions_ref[mask],
                            target,
                            tolerance,
                            candidate_limit,
                            full_mask,
                            node_base,
                            1,
                            &mut gamma_cache,
                            &mut integer_facts_cache,
                            partition_progress_ref,
                        );
                        local_results.push((mask, work));
                        completed_ref.fetch_add(1, Ordering::Relaxed);
                    }
                    local_results
                }));
            }

            let mut last_reported = usize::MAX;
            while handles.iter().any(|handle| !handle.is_finished()) {
                let done = completed.load(Ordering::Relaxed);
                if done != last_reported {
                    eprint!("\rProcessing operation-cost {cost}: {done}/{full_mask}");
                    let _ = io::stderr().flush();
                    last_reported = done;
                }
                std::thread::sleep(std::time::Duration::from_millis(150));
            }
            let done = completed.load(Ordering::Relaxed);
            eprint!("\rProcessing operation-cost {cost}: {done}/{full_mask}");
            let _ = io::stderr().flush();

            handles
                .into_iter()
                .flat_map(|handle| handle.join().expect("subset worker panicked"))
                .collect::<Vec<_>>()
        })
    };

    results.sort_unstable_by_key(|(mask, _)| *mask);
    for (_, work) in results {
        merge_mask_work(search, work);
    }

    eprint!("\rProcessing operation-cost {cost}: {non_full_count}/{full_mask} | final subset");
    let _ = io::stderr().flush();
    let mut gamma_cache = HashMap::<u64, Option<f64>>::new();
    let mut integer_facts_cache = HashMap::<i64, IntegerFacts>::new();
    let final_work = process_mask_cost(
        &search.dp,
        digits,
        full_mask,
        cost,
        &binary_partitions_by_mask[full_mask],
        target,
        tolerance,
        candidate_limit,
        full_mask,
        node_base,
        worker_count,
        &mut gamma_cache,
        &mut integer_facts_cache,
        &partition_progress,
    );
    merge_mask_work(search, final_work);
    eprintln!("\rProcessing operation-cost {cost}: {full_mask}/{full_mask}");
}

fn best_candidates<const N: usize>(cell: &Cell) -> [Option<Candidate>; N] {
    let mut best: [Option<Candidate>; N] = [None; N];
    for candidate in cell.candidates.iter().copied() {
        for index in 0..N {
            if best[index].map_or(true, |current| candidate.score < current.score) {
                for shift in (index + 1..N).rev() {
                    best[shift] = best[shift - 1];
                }
                best[index] = Some(candidate);
                break;
            }
        }
    }
    best
}

fn read_line(prompt: &str) -> Option<String> {
    print!("{prompt}");
    io::stdout().flush().ok()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    Some(input.trim().to_owned())
}

fn main() {
    let number = match read_line("Number (digits only): ") {
        Some(value) if !value.is_empty() && value.bytes().all(|b| b.is_ascii_digit()) => value,
        _ => {
            println!("Enter a number made only of digits, such as 1234.");
            return;
        }
    };

    let digits: Vec<u8> = number.bytes().map(|b| b - b'0').collect();
    if digits.len() > MAX_DIGITS {
        println!("This version supports up to {MAX_DIGITS} digits.");
        return;
    }

    let target = match read_line("Target number: ").and_then(|s| s.parse::<f64>().ok()) {
        Some(value) if value.is_finite() && value.abs() <= MAX_ABS_VALUE => value,
        _ => {
            println!("Enter a finite real target with absolute value at most {MAX_ABS_VALUE}.");
            return;
        }
    };

    let depth = match read_line(&format!("Maximum depth (number of operations, max {MAX_DEPTH}): "))
        .and_then(|s| s.parse::<usize>().ok())
    {
        Some(value) if value <= MAX_DEPTH => value,
        _ => {
            println!("Enter a whole number from 0 to {MAX_DEPTH}.");
            return;
        }
    };

    let tolerance = EXACT_TOLERANCE * target.abs().max(1.0);

    let candidate_limit = std::env::var("MATHBRUTE_CANDIDATES")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .filter(|&n| (1..=MAX_CANDIDATES_PER_CELL).contains(&n))
        .unwrap_or(DEFAULT_CANDIDATES_PER_CELL);

    let worker_count = configured_worker_count();
    eprintln!("Parallel workers: {worker_count}");

    if digits.len() > MAX_EXACT_DIGITS {
        run_long_input_search(&digits, target, tolerance, depth, candidate_limit, worker_count);
        return;
    }

    let size = 1usize << digits.len();
    let full_mask = size - 1;
    let mut search = Search::new(size, depth, target, tolerance, full_mask, candidate_limit);
    let binary_partitions_by_mask: Vec<Vec<(usize, usize)>> = (0..size)
        .map(|mask| {
            if mask.count_ones() >= 2 {
                make_binary_partitions(mask)
            } else {
                Vec::new()
            }
        })
        .collect();

    for cost in 0..=depth {
        let wave_started = std::time::Instant::now();
        process_cost_wave(
            &mut search,
            &digits,
            &binary_partitions_by_mask,
            cost,
            candidate_limit,
            worker_count,
        );

        let wave_seconds = wave_started.elapsed().as_secs_f64().max(1.0e-9);
        eprintln!(
            "Completed operation-cost {cost} wave: {} subsets in {wave_seconds:.2}s ({:.2} subsets/s)",
            full_mask,
            full_mask as f64 / wave_seconds,
        );

        let found_at_this_cost = search.dp[full_mask][cost]
            .candidates
            .iter()
            .any(|candidate| candidate.score <= tolerance);
        if found_at_this_cost {
            search.found_solution = true;
            break;
        }
    }
    eprintln!();

    for cost in 0..=depth {
        let matches: Vec<Candidate> = search.dp[full_mask][cost]
            .candidates
            .iter()
            .copied()
            .filter(|c| c.score <= tolerance)
            .take(10)
            .collect();

        if !matches.is_empty() {
            println!("Found {} candidate(s) within tolerance using {cost} operation(s):", matches.len());
            println!("Configured relative tolerance: {EXACT_TOLERANCE:.3e} (absolute bound: {tolerance:.15})");
            for candidate in matches {
                println!(
                    "{} = {} (absolute error: {:.15})",
                    render_expr(candidate.node_id, &search.nodes, &digits),
                    complex_display(candidate.value, candidate.imag),
                    candidate.score,
                );
            }
            return;
        }
    }

    let best = search.dp[full_mask]
        .iter()
        .enumerate()
        .flat_map(|(cost, cell)| cell.candidates.iter().map(move |candidate| (candidate, cost)))
        .min_by(|a, b| a.0.score.total_cmp(&b.0.score));

    println!("No exact match found in the candidates explored.");
    println!("This is a capped search, so that does not prove no solution exists.");
    println!("Maximum candidates per cell: {candidate_limit}; higher operation depths use smaller caps.");
    println!("Set MATHBRUTE_CANDIDATES to raise the cap (up to {MAX_CANDIDATES_PER_CELL}) or lower it for more speed.");

    if let Some((candidate, cost)) = best {
        println!(
            "Closest result ({} operations): {} = {} (difference: {:.15})",
            cost,
            render_expr(candidate.node_id, &search.nodes, &digits),
            complex_display(candidate.value, candidate.imag),
            candidate.score
        );
    }
}
