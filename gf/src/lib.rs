// Description: Simulate a class for Galois field

#[macro_use] extern crate impl_ops;
extern crate num_bigint;
extern crate num_integer;
extern crate num_traits;
extern crate num;
use num_bigint::BigInt;
use num_integer::Integer;
use std::ops;
use std::fmt;

// https://www.oreilly.com/library/view/programming-bitcoin/9781492031482/ch01.html
pub struct GF
{
	pub num:   BigInt,
	pub prime: BigInt
}

// Constructor
impl GF
{
	pub fn new<T:num_bigint::ToBigInt>(n:&T,p:&T) -> GF
	{
		let mut new_n:BigInt = n.to_bigint().unwrap();
		let     new_p:BigInt = p.to_bigint().unwrap();
		new_n = new_n.mod_floor(&new_p);
		GF{num:new_n,prime:new_p}
	}
}

// Copy
impl GF
{
	pub fn copy(&self) -> GF
	{
		GF::new(&self.num,&self.prime)
	}
}

// Implement use by print
impl fmt::Display for GF
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		// Show in Hex if bigher than 32 bits
		if self.prime > BigInt::parse_bytes(b"FFFFFFFF",16).unwrap()
		{
			let p = BigInt::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",16).unwrap();
			if self.prime == p
			{
				write!(f,"{:X} (mod P)",self.num)
			}
			else
			{
				write!(f,"{:X} (mod {:X})",self.num,self.prime)
			}
		}
		else
		// Show in dec if not
		{
			write!(f,"{} (mod {})",self.num,self.prime)
		}
	}
}

// Implement bool comparison for GF
impl PartialEq for GF
{
	fn eq(&self,other:&GF) -> bool
	{
		self.num == other.num && self.prime == other.prime
	}
}

// Implement addition for GF
impl_op_ex!(+ | a:&GF , b:&GF | -> GF
{
	if a.prime != b.prime
	{
		panic!("Cannot add two numbers in different Fields")
	}
	GF::new( &( (a.num.clone() + b.num.clone()).mod_floor(&a.prime) ) , &a.prime )
});

// Implement subtraction for GF
impl_op_ex!(- | a:&GF , b:&GF | -> GF
{
	if a.prime != b.prime
	{
		panic!("Cannot subtract two numbers in different Fields")
	}
	GF::new( &( (a.num.clone() - b.num.clone()).mod_floor(&a.prime) ) , &a.prime )
});

// Implement multiplication for GF
impl_op_ex!(* | a:&GF , b:&GF | -> GF
{
	if a.prime != b.prime
	{
		panic!("Cannot multiply two numbers in different Fields")
	}
	GF::new( &( (a.num.clone() * b.num.clone()).mod_floor(&a.prime) ) , &a.prime )
});

// Implement division for GF
impl_op_ex!(/ | a:&GF , b:&GF | -> GF
{
	if a.prime != b.prime
	{
		panic!("Cannot divide two numbers in different Fields")
	}
	let c = a.prime.clone() - BigInt::from(2);
	a * b.copy().pow(&c)
});

// Implement exponentiation for GF
impl GF
{
	// Exponent should be a integer, not a GF
	pub fn pow<T:num_bigint::ToBigInt>(self,exp:&T) -> GF
	{
		let big_exp:BigInt = exp.to_bigint().unwrap();
		let n1 = &self.prime - BigInt::from(1);
		let n = big_exp.mod_floor(&n1);
		let new_n = BigInt::modpow(&self.num,&n,&self.prime);
		GF::new(&new_n,&self.prime)
	}
}

// Implements the unary negation to GF
impl_op_ex!(- | a:&GF | -> GF
{
	GF::new(&-a.num.clone(),&a.prime)
});

