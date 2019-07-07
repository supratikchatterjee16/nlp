# Rust NLP

This is an experimental prototype to test the limits of programming in Rust.
It is meant to be an NLP module that lets people to analyze sentence structures, create their own custom logics and ultimately utilize this to create systems that can use Natural Language more freely than other systems.

First of all, this is being made for scripts that are closely related to the English script.
The concepts are going to be written below as this develops.

## Tag

Everything in NLP gets tagged. Each element contains of list tags that are created and managed by Annotators.
Annotators will be brought up later in the README, as it is a vast topic.
Each structure mentioned here, contain this element as an element in a linked list.

## Token

Tokens are the basic blocks that a person is going to deal with. Their classification is given below:

* Numeric
	* Normal	: Integers
	* Special	: Floats, Dates, etc.
> This is not done in the API at low level. This is done using Annotators at a higher level
* Alphabetic
	* Normal 	: Word
	* Special	: Names/Entities
> Again Seperation is made using annotators
* Alphanumeric	: These are special sequences that need to be accepted into the system for greater flexibility
* Punctuations
	* Endings	: Punctuations used to end sentences
	* Others	: Every other punctuation that can be used.

## Sentence

These are the next blocks of information that stores the semantics of the entered sentence in the 'Token' elements, as a list.

## Paragraph

These are a build on Literal to facilitate data structuring, based on the semantics stated by the different grammars.

## Context

Contains a list of paragaraphs.

