extern crate num_bigint;
use num_bigint::BigInt;
extern crate curve;
use curve::Curve;
extern crate gf;
use gf::GF;

fn main()
{
	// Initialize curve Secp256k1
	let c = Curve::secp256k1();

	// Operations
	let ri1 = GF::new(&BigInt::parse_bytes(b"8BCC628F650D6235ED8832BA615F34CC81BA0466A758D7A3D95EF7EAF6151CC2",16).unwrap(),&c.p);
	let vi1 = GF::new(&BigInt::from(40),&c.p);

	let ri2 = GF::new(&BigInt::parse_bytes(b"41DBBA6211CAF16AEEA766F847595BD69B1F96E8E52DD1D5DECDA9E5D99C5C2F",16).unwrap(),&c.p);
	let vi2 = GF::new(&BigInt::from(15),&c.p);

	// https://github.com/mimblewimble/grin/blob/master/doc/intro.md
	// (ri1*G + vi1*H) + (ri2*G + vi2*H) = (ro3*G + vo3*H)
	println!("{}",
		c.sk_to_pk(&ri1,&c.g)         + c.sk_to_pk(&vi1,&c.h) +
		c.sk_to_pk(&ri2,&c.g)         + c.sk_to_pk(&vi2,&c.h) ==
		c.sk_to_pk(&(&ri1+&ri2),&c.g) + c.sk_to_pk(&(&vi1+&vi2),&c.h));
}

