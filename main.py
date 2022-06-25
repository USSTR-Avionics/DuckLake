from flask_restful import Api, Resource
from flask_sqlalchemy import SQLAlchemy
from website import create_app
import psycopg2


app = create_app()
api = Api(app)

db = SQLAlchemy()
db.init_app(app=app)

class createDB(Resource):

    def post(self):
        pass

    def get(self):
        pass


api.add_resource(createDB,"/api/create/<string:test>")


if __name__ == "__main__":
    print("[LOG] App __init__")
    app.run()
