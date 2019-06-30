# Rust NLP

This is an experimental prototype to test the limits of programming in Rust.
It is meant to be an NLP module that lets people to analyze sentence structures, create their own custom logics and ultimately utilize this to create systems that can use Natural Language more freely than other systems.

First of all, this is being made for scripts that are closely related to the English script.
The concepts are going to be written below as this develops.

## Literal

Literals are the basic blocks that a person is going to deal with. Their classification is given below:

* Numeric
	* Normal	: Integers
	* Special	: Floats, Dates, etc.
* Alphabetic
	* Normal 	: Word
	* Special	: Names/Entities

## Sentence

These are the next blocks of information that stores the semantics of the entered sentence in the 'Literal' structure.

## Annotation

These are a build on Literal to facilitate data structuring, based on the semantics stated by the different grammars.