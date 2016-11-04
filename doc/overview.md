## Overview

### Block

#### Parameters

* Start address:
  * Automatic - is taken from the current position.
  * Manual - user specify position manually.
* Array size - if bigger than 0 then block contains specified number of elements of the same type.
* Block element size:
  * Fixed manual - size is specified by user.
  * Fixed automatic - size is automatically  calculated as sum of fixed size subelements.
  * Dynamic - size is dynamically calculated. Can be different for different positions of this block.
* Attributes:
  * Name
  * Comment
  * Size - block element size.
  * Endian - little, big.
  * NumberSystem - hex, dec, oct, bin.
  * Visible - true, false.