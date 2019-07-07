/*
* TITLE  : NLP
* DESCRIPTION : NLP PACKAGE FOR MORE FASTER AND EFFICIENT ANALYSIS AND UNDERSTANDING OF TEXTS.
* AUTHOR : SUPRATIK CHATTERJEE
* YEAR OF CREATION   : 2019
* YEAR OF COMPLETION : ----
*
* MODULE DESCRIPTON
* Acts as a merge point for all submodules within the package.
* Contains only package management and expose logic.
*/

pub mod structure;//Contains structures for working with the NLP module

pub mod file;//Contains the file handlers to load and work with different files. 
/**
* Module 'file' needs to tackle multiple file formats necessary to work with the program.
* It should tackle XML, Excel sheets, CSVs, JSON and any other well known tabulation format.
* Software has it's own methods of memory management associated with this package.
*/

pub mod net;//Contains modules required to work with web scraping. Provides a REST API for network based processing

pub mod distributed;//Contains modules for distributed programming

pub mod core;//Contains threading logic, pipe creation logic, multiple layers of sanity check, and main execution program.

pub mod aux;//Contains utilities associated with the software