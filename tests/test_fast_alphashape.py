import geopandas as gpd
from shapely import wkt
from shapely.geometry import Point

from fast_alphashape import __version__, alphashape


def test_convex_hull_from_np_array(points):
    res = alphashape(points, alpha=0)
    expect = wkt.loads("POLYGON ((0 0, 0 1, 1 1, 1 0, 0 0))")
    assert res.equals(expect)


def test_convex_hull_from_multipoint(points):
    res = alphashape(points, alpha=0)
    expect = wkt.loads("POLYGON ((0 0, 0 1, 1 1, 1 0, 0 0))")
    assert res.equals(expect)


def test_convex_hull_from_gpd(points):
    gdf = gpd.GeoDataFrame({"geometry": [Point(p) for p in points]})
    res = alphashape(points, alpha=0)
    expect = wkt.loads("POLYGON ((0 0, 0 1, 1 1, 1 0, 0 0))")
    assert res.equals(expect)


def test_basic_2d_shape(points):
    res = alphashape(points, alpha=2)
    expect = wkt.loads(
        "POLYGON ((0.75 0.5, 1 0, 0.5 0.25, 0 0, 0.25 0.5, 0 1, 0.5 0.75, 1 1, 0.75 0.5))"
    )
    assert res.equals(expect)


def test_from_geopandas_lines(resources):
    gdf = gpd.read_file(resources / "lines.geojson")
    res = alphashape(gdf, alpha=0.05)
    expect = wkt.loads(
        (
            "POLYGON (("
            "-0.11935515 51.52092008, -0.11071088 51.52280731, "
            "-0.09509052 51.51931587, -0.09084420999999999 51.51506917, "
            "-0.09008594 51.50147706, -0.09524217 51.49949453, "
            "-0.11146915 51.49628455, -0.12072004 51.49826722, "
            "-0.12496635 51.5004386, -0.13057755 51.50978383, "
            "-0.12496635 51.51884404, -0.11935515 51.52092008"
            "))"
        )
    )
    assert res.equals(expect)
