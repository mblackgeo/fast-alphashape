import math
from typing import Union

import geopandas as gpd
import numpy as np
from shapely.geometry import MultiLineString, MultiPoint, Point, Polygon
from shapely.ops import polygonize, unary_union

from ._fast_alphashape import alphashape_wrapper


def alphashape(
    points: Union[np.array, MultiPoint, gpd.GeoDataFrame],
    alpha: float,
) -> Polygon:
    """Generate an alpha shape for a 2D array of points

    Parameters
    ----------
    points : Union[np.array, MultiPoint, gpd.GeoDataFrame]
        A numpy array of shape [n,2] containing points, or
        a Shapely MultiPoint object, or
        a GeoDataFrame containing geometry (coordinates will be extracted).
    alpha : float
        Alpha value. Determines the "smoothness" of the resulting concave hull.
        Higher values allow more concavity, lower values result in a "smoother"
        hull but may not contain all original input points. A value of 0 gives
        the convex hull.

    Returns
    -------
    Polygon
        Shapely Polygon of the alpha shape of the given points
    """
    # convert geometry in a GeoDataFrame to MultiPoint
    if isinstance(points, gpd.GeoDataFrame):
        points = MultiPoint(
            [Point(p) for g in points.geometry.values for p in g.coords]
        )

    # return convex hull if alpha is zero
    if math.isclose(alpha, 0):
        points = MultiPoint(points) if isinstance(points, np.ndarray) else points
        return points.convex_hull

    # cast to numpy array of coordinates for passing through to rust
    if isinstance(points, MultiPoint):
        points = np.array(points)

    # only [n,2] shape is supported
    if points.ndim != 2:
        raise RuntimeError("Input must be [n, 2]")

    # get the indices of that make up the edges of the alpha shape
    edge_idxs = alphashape_wrapper(points, alpha)

    # Create the resulting polygon from the edge points
    m = MultiLineString([points[np.array(edge)] for edge in edge_idxs])
    return unary_union(list(polygonize(m)))
