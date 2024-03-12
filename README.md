# Weighted Voronoi Stippling

This project is an implementation of Weighted Voronoi Stippling, a technique used to generate stipple illustrations from grayscale images. The technique is based on the use of Voronoi diagrams and centroidal Voronoi tessellations.

Currently, the project implements Lloyd's algorithm, which is a method for finding evenly spaced sets of points in subsets of Euclidean spaces.

## Current Implementation

The current implementation is written in Rust and uses the Macroquad game framework for rendering. The Voronoi diagrams are calculated using the `voronoi` function from the `voronoi` crate.

The main logic of the program is in the `main` function. It starts by generating a set of random points. Then, in a loop, it calculates the Voronoi diagram for these points, draws the Voronoi polygons, calculates the centroids of these polygons, and moves the points towards their corresponding centroids. This is the essence of Lloyd's algorithm.

The amount by which the points are moved towards their centroids (the "lerp amount") can be increased or decreased by pressing the Up or Down key, respectively. Changing the "lerp amount" can cause unexpected behaviour and is mainly for debugging purpouses. The Space key can be pressed to generate a new set of random points.

## Future Work

The next step for this project is to implement the Weighted Voronoi Stippling technique. This will involve modifying the Voronoi diagram calculation to take into account the grayscale values of the input image, and adjusting the movement of the points accordingly.

## Building and Running

To build and run the project, you will need to have Rust and Cargo installed. You can then use the following command to run the project:

```bash
cargo run --release
```

## Sources

- <https://youtu.be/Bxdt6T_1qgc>
- <https://en.wikipedia.org/wiki/Lloyd%27s_algorithm>
- <https://en.wikipedia.org/wiki/Delaunay_triangulation>
- <https://en.wikipedia.org/wiki/Voronoi_diagram>
- <https://en.wikipedia.org/wiki/Shoelace_formula>
- <https://www.cs.ubc.ca/labs/imager/tr/2002/secord2002b/secord.2002b.pdf>

## Contributing

Contributions are welcome! Please feel free to submit a pull request.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
