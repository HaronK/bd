## Motivation

* Binary data format investigation.
* Data format verification.
* Extracting information from binary data.

## Overview

### Block

Block represents a slice of the data described by meta information, i.e. name, offset, children, etc.
Block occupies continuous area of the data but its children can be located either in the block area or in the different place.

#### Parameters

* Start address (offset):
  * Automatic - is taken from the current position.
  * Manual - user specify position manually.
* Array size - if bigger than 0 then block contains specified number of elements of the same type.
* Block element size:
  * Fixed manual - size is specified by user.
  * Fixed automatic - size is automatically  calculated as sum of fixed sizes subelements.
  * Dynamic - size is dynamically calculated. Can be different for different positions of this block.
* Attributes:
  * Name
  * Comment
  * Size - block element size.
  * Endian - little, big.
  * NumberSystem - hex, dec, oct, bin.
  * Visible - true, false.
  * ToString - string representation of the block.
  * User defined attributes.

### Template

Template contains an instructions how to build blocks hierarchy.

#### Types

* Array - array of elements of different types.
* Generic - has dynamic structure.
* Struct - has static structure (fixed number and count of the fields).
* Simple - basic type: integer, double, etc.
* Custom - templates that require specific treatment, i.e. c-strings.
* Container - block of data with a fixed size that can be initialized/parsed with other template(s). Container can be initialized with more than one template. In this case there will be several parallel block trees.

#### Template hierarchy visitors

* Lazy parser - walks over a tree and parses only elements with dynamic size.
* Element picker - walks to the specific element in the hierarchy and returns it or its value.
* Validator - walks over whole hierarchy and validates every element. Doesn't create constant hierarchy in the memory.
* Full parser - parses complete hierarchy and creates all necessary structures in the memory.
