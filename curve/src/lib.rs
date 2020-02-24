extern crate num_bigint;
use num_bigint::BigInt;
extern crate gf;
use gf::GF;
extern crate point;
use point::Point;

#[allow(dead_code)]
pub struct Curve
{
	pub p: BigInt,
	pub n: BigInt,
	pub g: Point,
	pub h: Point
}

#[allow(dead_code)]
impl Curve
{
	// Set parameters to Secp256k1
	pub fn secp256k1() -> Curve
	{
		let p = BigInt::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",16).unwrap();
		Curve
		{
			p: p.clone(),
			n: BigInt::parse_bytes(b"FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",16).unwrap(),
			g: Point
			{
				x: GF::new(&BigInt::parse_bytes(b"79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",16).unwrap(),&p),
				y: GF::new(&BigInt::parse_bytes(b"483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",16).unwrap(),&p)
			},
			h: Point
			{
				x: GF::new(&BigInt::parse_bytes(b"50929B74C1A04954B78B4B6035E97A5E078A5A0F28EC96D547BFEE9ACE803AC0",16).unwrap(),&p),
				y: GF::new(&BigInt::parse_bytes(b"31D3C6863973926E049E637CB1B5F40A36DAC28AF1766968C30C2313F3A38904",16).unwrap(),&p)
			}
		}
	}
}

// Check if a point is on the curve
#[allow(dead_code)]
impl Curve
{
	pub fn on_secp256k1(&self, p:&Point) -> bool
	{
		// Parameters of the curve Secp256k1: y^2 = x^3 + 0x + 7
		let a = GF::new(&BigInt::from(0),&p.x.prime);
		let b = GF::new(&BigInt::from(7),&p.x.prime);
		p.y.copy().pow(&2) - p.x.copy().pow(&3) - &a*p.x.copy() - b == GF::new(&a.num,&p.x.prime)
	}
}

// Compute sk*G with repeated addition
#[allow(dead_code)]
impl Curve
{
	pub fn sk_to_pk(&self, sk:&GF, g:&Point) -> Point
	{
		// Check if the key is valid
		let zero = BigInt::from(0);
		if !(zero < sk.num && sk.num < self.n)
		{
			panic!("The key is not in the curve range");
		}

		// Do the multiplication
		let one  = BigInt::from(1);
		let mut ret     = Point{x:GF::new(&zero,&g.x.prime),y:g.y.copy()};
		let mut local_g = Point{x:g.x.copy(),y:g.y.copy()};
		for i in 0..256
		{
			if &sk.num & (&one << i) != zero
			{
				if ret.x.num == zero
				{
					ret = Point{x:local_g.x.copy(),y:local_g.y.copy()};
				}
				else
				{
					ret = ret + &local_g;
				}
			}
			local_g = &local_g + &local_g;
		}
		ret
	}
}

