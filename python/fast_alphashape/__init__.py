# flake8: noqa
import numpy as np
from shapely.geometry import MultiLineString, Polygon
from shapely.ops import polygonize, unary_union

from ._fast_alphashape import alpha_shape_wrapper

__version__ = "0.1.0"


def alpha_shape(points: np.array, alpha: float) -> Polygon:
    """Generate an alpha shape for a 2D array of points

    Parameters
    ----------
    points : np.array
        An array [n,2] containing points
    alpha : float
        Alpha value

    Returns
    -------
    Polygon
        Shapely Polygon of the alpha shape of the given points
    """
    # TODO error handling, array shape checking

    # get the indices of that make up the edges of the alpha shape
    edge_idxs = alpha_shape_wrapper(points, alpha)

    # Create the resulting polygon from the edge points
    m = MultiLineString([points[np.array(edge)] for edge in edge_idxs])
    return unary_union(list(polygonize(m)))
