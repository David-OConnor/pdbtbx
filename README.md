![Compile & Test](https://github.com/nonnominandus/rust-pdb/workflows/Compile%20&%20Test/badge.svg)

## Description
This is a Rust library helping to parse, edit and save crystallographic PDB files. It can read most crystallographic PDB lines, some are missing which we are hopeful will be added over time. Its high level goal is to create a stable, efficient and easy to use interface to PDB files. 

## Contributing
As this is a library in active development, feel free to share your thoughts, ideas, hopes, and criticisms. Every comment will be read and discussed so that this library is as useful as possible for all users. Of course we all like a civilised discussion so please follow the community guidelines, but over all please be a civilised human being.

## License
MIT, just use it if you can use it. If you use it for something cool I would like to hear, but no obligations!

## PDB format
PDB 3.30 as published by wwPDB in 2008.

Very basic mmCIF saving support, will likely be extended in the future.

## Why
Just for fun, to play with fancy abstractions. But at the same time I think that using Rust in scientific computing would be really cool and this library would be needed if I where to be doing my internship in Rust. So by creating it I hope to extend the usability of Rust a little bit more. Since Nature published an [article](https://www.nature.com/articles/d41586-020-03382-2) (technology feature) which laid out the benefits of using Rust and showed that Rust is used more and more, recently I am planning on working more with Rust in scientific projects. And I think that the best way to help Rust move forward (in the scientific community) is by creating more support for scientific projects in Rust.

## Contributors
* Douwe Schulte
* [Tianyi Shi](https://github.com/TianyiShi2001)

## Last update
### v0.3.3
_API additions_
* Added very basic exporting to mmCIF, it will only export the unit cell, symmetry and atomic data. 

Also see [changelog](https://github.com/nonnominandus/pdbtbx/blob/master/changelog.md).