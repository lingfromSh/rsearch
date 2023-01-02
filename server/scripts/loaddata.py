import importlib
import subprocess


def install_dependency(name, alias=None):
    """
    Check specified dependency whether is installed.
    if dependency is uninstalled, then use pip to install it.

    :param name: name for import
    :param alias: alias for pypi
    :return:
    """
    try:
        importlib.import_module(name)
    except ImportError:
        subprocess.run(f"pip install {alias or name}".split())


# Install motor as asyncio mongodb client
install_dependency("motor")
install_dependency("mongoengine")


async def load_data():
    from motor.motor_asyncio import AsyncIOMotorClient
    import csv
    import mongoengine

    class MovieDocument(mongoengine.Document):
        """
        Structure of movie
        """

        release_year = mongoengine.IntField()
        title = mongoengine.StringField()
        origin = mongoengine.StringField(null=True)
        director = mongoengine.ListField(mongoengine.StringField())
        cast = mongoengine.ListField(mongoengine.StringField())
        genre = mongoengine.ListField(mongoengine.StringField())
        wiki_page = mongoengine.URLField()
        plot = mongoengine.StringField()
        meta = {"collection": "movie"}

    m_client = AsyncIOMotorClient("mongodb://rsearch:password@database:27017")
    db = m_client.rsearch
    await db.movie.create_index([("plot", "text"), ("title", "text")])
    reader = csv.reader(open("../dataset/wiki_movie_plots_deduped.csv"))
    labels = ["release_year", "title", "origin", "director", "cast", "genre", "wiki_page", "plot"]
    rows = []
    for line_number, row in enumerate(reader):
        if line_number == 0:
            continue

        data = dict(zip(labels, row))
        data["genre"] = data.get("genre").split()
        data["cast"] = data.get("cast").split(",")
        data["director"] = data.get("director").split(",")
        movie = MovieDocument(**data)
        movie.validate()
        rows.append(movie.to_mongo())

        if len(rows) % 1000 == 0 and rows:
            await db.movie.insert_many(rows)
            rows.clear()


if __name__ == "__main__":
    import asyncio

    asyncio.run(load_data())
