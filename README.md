# Rust NLP

This project is an NLP tool made in Rust.
The concepts are going to be written below as this develops.

Targetted use cases are Communications and Literature.

Additionally, for relating language structures, if possible.

Current target : Coreference resolution, NER and Sentiment Tracking.

Precursor : Contextual classification of sentiment in a sentence.

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

Tokens will be classified by annotators.



## Sentence

This is a superblock of Token. 

This can be utilized for filtration, by understanding the type of sentence.

Types of Sentences(based on content) are:
1. Imperative
2. Assertive/Declarative
3. Interrogative
4. Exclamatory

Sentences however can be classified further based on structure.
They can be classified as :
1. Simple : Has 1 independent clause and is a whole sentence
2. Compound : Uses FANBOYS(coordinating conjunctions) to connect 2 connected clauses(1 independent, 1 dependent)
3. Complex : Has 1 independent and has atleast 1 dependent clause. Uses subordinate conjunctions.

They aren't easy to classify, as punctuation usage is not well defined.

## Paragraph

A super block of sentences to store various other information. 
Each paragraph speaks on either one topic, or gives opinions of a person.
They are of the following types:
1. Descriptive
2. Expository
3. Narrative
4. Persuasive

## Context

Contains a list of paragaraphs and metadata. 
Metadata information, will be used for confidence level classification in associated Contexts.

Contexts can be classified(and subclassified into the following):

1. Literary [1](https://mason.gmu.edu/~rnanian/305context.html)
	* Authorial
	* Socio-historical
	* Philosophical
	* Literary
	* Critical
2. Communicative [2](http://oregonstate.edu/instruct/theory/contexts.html)
	* Historical
	* Psychological
	* Cultural
	* Social
	* Physical

Classification of contexts are for improved sentiment analysis. It also helps in discrete sentiment analysis. 