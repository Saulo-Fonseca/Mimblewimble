#[macro_use] extern crate impl_ops;
extern crate gf;
use gf::GF;
extern crate num_bigint;
use num_bigint::BigInt;
use std::fmt;
use std::ops;

// Structure for a point
pub struct Point
{
	pub x: GF,
	pub y: GF
}

// Implement use by print
impl fmt::Display for Point
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f,"x: {}\ny: {}",self.x,self.y)
	}
}

// Implement bool comparison for Point
impl PartialEq for Point
{
	fn eq(&self,other:&Point) -> bool
	{
		self.x == other.x && self.y == other.y
	}
}

// Implement addition for two points on a curve
impl_op_ex!(+ | p:&Point , q:&Point | -> Point
{
	let lam;
	if p.x == q.x && p.y == q.y
	{
		lam = (GF::new(&BigInt::from(3),&p.x.prime) * p.x.copy().pow(&2)) / (GF::new(&BigInt::from(2),&p.y.prime) * p.y.copy());
	}
	else
	{
		if q.x != p.x
		{
			lam = (q.y.copy() - p.y.copy()) / (q.x.copy() - p.x.copy());
		}
		else
		{
			panic!("Division by zero");
		}
	}
	let rx = lam.copy().pow(&2) - p.x.copy() - q.x.copy();
	let ry = lam.copy() * (p.x.copy() - rx.copy()) - p.y.copy();
	if (rx == p.x && ry == p.y) || (rx == q.x && ry == q.y)
	{
		panic!("Zero point found");
	}
	Point
	{
		x: rx.copy(),
		y: ry.copy()
	}
});

// Implement subtraction for two points on a curve
impl_op_ex!(- | p:&Point , q:&Point | -> Point
{
	let lam;
	let iq = Point{x:q.x.copy(),y:GF::new(&-q.y.num.clone(),&q.y.prime)}; // Inverse of q
	if p.x == iq.x && p.y == iq.y
	{
		lam = (GF::new(&BigInt::from(3),&p.x.prime) * p.x.copy().pow(&2)) / (GF::new(&BigInt::from(2),&p.y.prime) * p.y.copy());
	}
	else
	{
		if iq.x != p.x
		{
			lam = (iq.y.copy() - p.y.copy()) / (iq.x.copy() - p.x.copy());
		}
		else
		{
			panic!("Division by zero");
		}
	}
	let rx = lam.copy().pow(&2) - p.x.copy() - iq.x.copy();
	let ry = lam.copy() * (p.x.copy() - rx.copy()) - p.y.copy();
	if (rx == p.x && ry == p.y) || (rx == iq.x && ry == iq.y)
	{
		panic!("Zero point found");
	}
	Point
	{
		x: rx.copy(),
		y: ry.copy()
	}
});

