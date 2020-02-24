# Testing the Mimblewimble Concept

Here you find a test built on the main concept behind the Mimblewimble protocol.

You can compile it as following:

* Go to the folder **mimblewimble**
* Run **cargo build**

# Module GF

This module implements the Galois Field concept, which limits mathematical operations in range of integer numbers, flipping around if an overflow occurs. This is a basic concept used behind many cryptographic operations.

This module uses the BigNum library, which allows the use of 256 bits numbers or even larger. All the basic operations are covered:

* New
* Copy
* Display
* Boolean comparison
* Addition
* Subtraction
* Multiplication
* Division
* Exponentiation
* Negation

# Module Point

This module implements operations on Elliptic Curve points. This is also a basic activity used in cryptography. Here you have the following operations:

* Display
* Boolean comparison
* Addition
* Subtraction

# Module Curve

This module implements the elliptic curve Secp256k1, used in most cryptocurrencies. It has the following functions:

* Defines the numbers P and N along with the points G and H, used in Secp256k1.
* Test if a point in on the curve.
* Multiplies a point by an integer.

# Main Mimblewimble Program

This program uses all the modules above, which set the layers necessary to have elliptic curve operations working in cryptography. It basically does the following calculation:

<code>(ri1\*G + vi1\*H) + (ri2\*G + vi2\*H) = (ro3\*G + vo3\*H)</code>

This is the structure of a transaction, which proves that the sum of many inputs given as Pedersen Commitments is equal to the respective output. This is the first pillar of Mimblewimble: the arithmetic required to validate a transaction can be done without knowing any of the values.
