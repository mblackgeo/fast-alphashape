extern crate ndarray;

use _fast_alphashape::*;
use ndarray::*;

fn main() {
    let points = array![
        [0., 0.],
        [0., 1.],
        [1., 1.],
        [1., 0.],
        [0.5, 0.25],
        [0.5, 0.75],
        [0.25, 0.5],
        [0.75, 0.5]
    ];
    let _alpha = 2.0;

    // extract the simplex triangles
    let simplexes: Vec<i32> = alpha_simplices(points.view());

    // get the circum radius for each simplex
    let mut radii: Vec<f64> = Vec::new();
    for tri in simplexes.chunks_exact(3) {
        let coords = stack![
            Axis(0),
            points.slice(s![tri[0], ..]),
            points.slice(s![tri[1], ..]),
            points.slice(s![tri[2], ..]),
        ];

        let radius = circumradius(coords.view());
        radii.push(radius);

        println!("{:?} - {:?}", tri, radius);
    }

    // TODO - implement the rest of this
    // # Create a set to hold unique edges of simplices that pass the radius
    // # filtering
    // edges = set()
    //
    // # Create a set to hold unique edges of perimeter simplices.
    // # Whenever a simplex is found that passes the radius filter, its edges
    // # will be inspected to see if they already exist in the `edges` set.  If an
    // # edge does not already exist there, it will be added to both the `edges`
    // # set and the `permimeter_edges` set.  If it does already exist there, it
    // # will be removed from the `perimeter_edges` set if found there.  This is
    // # taking advantage of the property of perimeter edges that each edge can
    // # only exist once.
    // perimeter_edges = set()
    //
    // if circumradius < 1.0 / resolved_alpha:
    // for edge in itertools.combinations(point_indices, r=coords.shape[-1]):
    //     if all([e not in edges for e in itertools.combinations(edge, r=len(edge))]):
    //         edges.add(edge)
    //         perimeter_edges.add(edge)
    //     else:
    //         perimeter_edges -= set(itertools.combinations(edge, r=len(edge)))
    //
    // # Create the resulting polygon from the edge points
    // m = MultiLineString([coords[np.array(edge)] for edge in perimeter_edges])
    // triangles = list(polygonize(m))
    // result = unary_union(triangles)
}
