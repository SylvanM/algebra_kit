use rand::rngs;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

use crate::algebra::*;

// MARK: Groups

/// The additive group of the integers modulo an integer N
#[derive(Clone, Copy, Default, Debug)]
pub struct AdditiveGroupZM<const N: i64> {
	pub val: i64
}							

impl<const N: i64> Add for AdditiveGroupZM<N> {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		AdditiveGroupZM { val: (self.val + rhs.val).rem_euclid(N) }
	}
}

impl<const N: i64> AddAssign for AdditiveGroupZM<N> {
	fn add_assign(&mut self, rhs: Self) {
		self.val -= rhs.val;
		self.val = self.val.rem_euclid(N);
	}
}

impl<const N: i64> Neg for AdditiveGroupZM<N> {
	type Output = Self;

	fn neg(self) -> Self::Output {
		AdditiveGroupZM { val: (-self.val).rem_euclid(N) }
	}
}

impl<const N: i64> Sub for AdditiveGroupZM<N> {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		AdditiveGroupZM { val: (self.val - rhs.val).rem_euclid(N) }
	}
}

impl<const N: i64> SubAssign for AdditiveGroupZM<N> {
	fn sub_assign(&mut self, rhs: Self) {
		self.val -= rhs.val;
		self.val = self.val.rem_euclid(N)
	}
}

impl<const N: i64> Mul for AdditiveGroupZM<N> {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		self + rhs
	}
}

impl<const N: i64> MulAssign for AdditiveGroupZM<N> {
	fn mul_assign(&mut self, rhs: Self) {
		*self += rhs
	}
}

impl<const N: i64> Div for AdditiveGroupZM<N> {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		AdditiveGroupZM { val: self.val - rhs.val }
	}
}

impl<const N: i64> DivAssign for AdditiveGroupZM<N> {
	fn div_assign(&mut self, rhs: Self) {
		*self -= rhs
	}
}

impl<const N: i64> PartialEq for AdditiveGroupZM<N> {
	fn eq(&self, other: &Self) -> bool {
		self.val.rem_euclid(N) == other.val.rem_euclid(N)
	}
}

impl<const N: i64> Group for AdditiveGroupZM<N> {

	fn identity() -> Self {
		AdditiveGroupZM { val: 0 }
	}

	fn inverse(&self) -> Self {
		-(*self)
	}
}

impl<const N: i64> AdditiveGroupZM<N> {

	/// Creates a group element in Z/(N) from the integer x
	pub fn from_int(x: i64) -> AdditiveGroupZM<N> {
		AdditiveGroupZM { val: x.rem_euclid(N) }
	}

	/// Generates a random group element
	/// 
	/// NOT cryptographically secure!
	pub fn random() -> AdditiveGroupZM<N> {
		AdditiveGroupZM { val: rand::thread_rng().gen_range(0..N) }
	}

}

// MARK: Rings and Fields

impl Ring for f64 {
	fn one() -> Self {
		1.0
	}

	fn is_zero(&self) -> bool {
		*self == 0.0
	}

	fn zero() -> Self {
		0.0
	}

	fn power(&self, n: i64) -> Self {
		self.powf(n as f64)
	}
}

impl PoRing for f64 { /* f64 already satisfies this! */ }

impl Field for f64 {
	fn inverse(&self) -> Self {
		if self.is_zero() {
			panic!("Cannot divide by zero")
		} else {
			1.0 / self
		}
	}
}

impl PoField for f64 { }

impl Ring for f32 {

	fn one() -> Self {
		1.0
	}

	fn zero() -> Self {
		0.0
	}

	fn is_zero(&self) -> bool {
		*self == 0.0
	}

	fn power(&self, n: i64) -> Self {
		self.powf(n as f32)
	}
}

impl PoRing for f32 { /* f64 already satisfies this! */ }

impl Field for f32 {
	fn inverse(&self) -> Self {
		if self.is_zero() {
			panic!("Cannot divide by zero")
		} else {
			1.0 / self
		}
	}
}

impl PoField for f32 { /* f64 already satisfies this! */ }

impl Ring for i8 {
	
	fn one() -> Self {
		1
	}

	fn zero() -> Self {
		0
	}

	fn is_zero(&self) -> bool {
		*self == 0
	}

	fn power(&self, n: i64) -> Self {
		if n < 0 {
			panic!("Cannot invert ring element")
		}
		self.pow(n as u32)
	}
	
}

impl PoRing for i8 { }
impl OrderedRing for i8 { }

impl Ring for i16 {
	fn one() -> Self {
		1
	}

	fn zero() -> Self {
		0
	}

	fn power(&self, n: i64) -> Self {
		if n < 0 {
			panic!("Cannot invert ring element")
		}
		self.pow(n as u32)
	}
	
	fn is_zero(&self) -> bool {
		*self == 0
	}
}

impl PoRing for i16 { }
impl OrderedRing for i16 { }

impl Ring for i32 {
	fn one() -> Self {
		1
	}

	fn zero() -> Self {
		0
	}

	fn is_zero(&self) -> bool {
		*self == 0
	}

	fn power(&self, n: i64) -> Self {
		if n < 0 {
			panic!("Cannot invert ring element")
		}
		self.pow(n as u32)
	}
}

impl PoRing for i32 { }
impl OrderedRing for i32 { }

impl Ring for i64 {
	fn one() -> Self {
		1
	}

	fn zero() -> Self {
		0
	}

	fn is_zero(&self) -> bool {
		*self == 0
	}

	fn power(&self, n: i64) -> Self {
		if n < 0 {
			panic!("Cannot invert ring element")
		}
		self.pow(n as u32)
	}
}

impl PoRing for i64 { }
impl OrderedRing for i64 { }

impl Ring for i128 {
	fn one() -> Self {
		1
	}

	fn zero() -> Self {
		0
	}

	fn is_zero(&self) -> bool {
		*self == 0
	}

	fn power(&self, n: i64) -> Self {
		if n < 0 {
			panic!("Cannot invert ring element")
		}
		self.pow(n as u32)
	}
}

impl PoRing for i128 { }
impl OrderedRing for i128 { }

/// The field of integers modulo a HUGE prime Q

/// The field of the integers modulo a prime Q
#[derive(Clone, Copy, Default)]
pub struct ZM<const Q: i64> {
	pub val: i64
}

impl<const Q: i64> ZM<Q> {
	pub fn rnd() -> ZM<Q> {
		ZM::<Q> { val: StdRng::from_entropy().gen::<i64>().rem_euclid(Q) }
	}
}

impl<const Q: i64> Debug for ZM<Q> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.val.fmt(f)
	}
}

impl<const Q: i64> PartialEq for ZM<Q> {
	fn eq(&self, other: &Self) -> bool {
		self.val == other.val
	}

	fn ne(&self, other: &Self) -> bool {
		self.val != other.val
	}
}

impl<const Q: i64> ZM<Q> {
	pub fn convert<const P: i64>(other: ZM<P>) -> ZM<Q> {
		other.val.into()
	}

	pub fn from_int(x: i64) -> ZM<Q> {
		x.into()
	}
}

impl<const Q: i64> From<i64> for ZM<Q> {
	fn from(value: i64) -> Self {
		ZM::<Q> { val: value.rem_euclid(Q) } 
	}
}

impl<const Q: i64> From<u8> for ZM<Q> {
	fn from(value: u8) -> Self {
		ZM::<Q> { val: (value as i64).rem_euclid(Q) } 
	}
}

impl<const Q: i64> From<i32> for ZM<Q> {
	fn from(value: i32) -> Self {
		ZM::<Q> { val: (value as i64).rem_euclid(Q) } 
	}
}

impl<const Q: i64> Add<ZM<Q>> for ZM<Q> {
	type Output = ZM<Q>;

	fn add(self, rhs: ZM<Q>) -> Self::Output {
		ZM::<Q> { val: (self.val + rhs.val) % Q }
	}
}

impl<const Q: i64> AddAssign<ZM<Q>> for ZM<Q> {
	fn add_assign(&mut self, rhs: ZM<Q>) {
		*self = *self + rhs
	}
}

impl<const Q: i64> Sub<ZM<Q>> for ZM<Q> {
	type Output = ZM<Q>;

	fn sub(self, rhs: ZM<Q>) -> Self::Output {
		ZM::<Q> { val: (self.val - rhs.val + Q) % Q }
	}
}

impl<const Q: i64> SubAssign<ZM<Q>> for ZM<Q> {
	fn sub_assign(&mut self, rhs: ZM<Q>) {
		*self = *self - rhs
	}
}

impl<const Q: i64> Mul<ZM<Q>> for ZM<Q> {
	type Output = ZM<Q>;

	fn mul(self, rhs: ZM<Q>) -> ZM<Q> {
		let product = self.val.rem_euclid(Q) * rhs.val.rem_euclid(Q);
		ZM::<Q> { val: product % Q }
	}
}

impl<const Q: i64> MulAssign<ZM<Q>> for ZM<Q> {
	fn mul_assign(&mut self, rhs: ZM<Q>) {
		*self = *self * rhs
	}
}

impl<const Q: i64> Neg for ZM<Q> {
	type Output = Self;

	fn neg(self) -> Self::Output {
		(Q - self.val).into()
	}
}

impl<const Q: i64> Ring for ZM<Q> {
	fn one() -> Self {
		ZM::<Q> { val: 1 }
	}

	fn zero() -> Self {
		ZM::<Q> { val: 0 }
	}

	fn is_zero(&self) -> bool {
		self.val == 0
	}

	fn power(&self, n: i64) -> Self {
		// TODO: Make this WAYY more efficient... Double and add, yeah?
		let mut power = ZM::<Q>::one();

		for _ in 1..=n {
			power *= *self
		}

		power
	}
}

impl EuclideanDomain for i64 {
	type SizeType = usize;

	fn euc_size(&self) -> usize {
		self.abs().try_into().unwrap()
	}

	fn quotient_and_remainder(&self, divisor: &Self) -> (Self, Self) {
		(self / divisor, self % divisor)
	}
}

pub fn mod_inv<R: EuclideanDomain>(x: R, m: R) -> R {
	match ext_gcd(x, m) { (_, i, _) => i }
}

impl<const Q: i64> Div<ZM<Q>> for ZM<Q> {
	type Output = ZM<Q>;

	fn div(self, rhs: ZM<Q>) -> Self::Output {
		self * mod_inv(rhs.val, Q).into()
	}
}

impl<const Q: i64> DivAssign<ZM<Q>> for ZM<Q> {
	fn div_assign(&mut self, rhs: ZM<Q>) {
		*self = *self / rhs
	}
}

impl<const Q: i64> Field for ZM<Q> {
	fn inverse(&self) -> Self {
		mod_inv(self.val, Q).into()
	}
}