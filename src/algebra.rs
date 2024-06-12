//
// A collection of primitive algebraic structures
//

use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};

// MARK: Ring

/// An algebraic Ring
pub trait Ring: Debug + Clone + PartialEq + Sized + Add<Self> + AddAssign<Self> + Neg + Sub<Self> + SubAssign<Self> + Mul<Self> + MulAssign<Self> + Mul<Output = Self> + Add<Output = Self> + Neg<Output = Self> + Sub<Output = Self> {

	/// The multiplicative identity of this ring
	fn one() -> Self;

	/// The additive identity of this ring/
	fn zero() -> Self;

	/// Whether or not this ring element is zero
	fn is_zero(&self) -> bool;

	/// A ring element raised to a power/
   	fn power(&self, n: i64) -> Self;
}

// MARK: Field

/**
 * A field, which is a ring where every nonzero element has a multiplicative inverse
 */
pub trait Field: Ring + Div + DivAssign + Div<Output = Self> {
	fn inverse(&self) -> Self;
}

// MARK: Inner Product Space
pub trait InnerProductSpace<R: Ring> {
	fn inner_product(&self, other: Self) -> R;
}

pub trait NormSpace {

	type NormType: Ord;

	fn norm(&self) -> Self::NormType;
}

// MARK: Euclidean Domain

pub trait EuclideanDomain: Ring + Div + DivAssign + Rem + RemAssign {

	type SizeType: Ord;

	/// The Euclidean Size of this type. This is the size function 
	/// associated with a euclidean domain. I'm avoiding calling it "size"
	/// because often that function name is already used for other properties.
	fn euc_size(&self) -> Self::SizeType;

	/**
	 * Finds q and r such that 
	 *
	 * self = divisor * q + r
	 */
	fn quotient_and_remainder(&self, divisor: &Self) -> (Self, Self);
}

/// Returns (g, x, y) so that 
/// - g = gcd(a, b)
/// ax + by = gcd(a, b)
pub fn ext_gcd<R: EuclideanDomain>(a: R, b: R) -> (R, R, R) {

	if a == R::zero() {
		return (b, R::zero(), R::one())
	}

	let (q, r) = b.quotient_and_remainder(&a);

	let (g, x1, y1) = ext_gcd(r, a);

	let x = y1 - q * x1.clone();

	(g, x, x1)
}

/// The Euclidean Algorithm to find the GCD of two elements in a Euclidean Domain
pub fn gcd<R: EuclideanDomain>(a: &R, b: &R) -> R {
	if a.is_zero() {
		b.clone()
	} else if b.is_zero() {
		a.clone()
	} else if a.euc_size() < b.euc_size() {
		gcd(b, a)
	} else {
		let (_, r) = a.quotient_and_remainder(b);
		gcd(b, &r)
	}
}