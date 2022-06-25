from flask_restful import Api, Resource
from flask_sqlalchemy import SQLAlchemy
from website import create_app
from datetime import datetime
from methods import method
import psycopg2
import pytz
import os



app = create_app()
api = Api(app)

db = SQLAlchemy()
db.init_app(app=app)



class create_record(Resource):

    def post(self,url):
        print(url)

        # Ensure the order of the args passed to the URL

        DATABASE_URL = os.environ.get("DATABASE_URL")

        conn = psycopg2.connect(
            DATABASE_URL,sslmode="require"
        )

        curr = conn.cursor()

        reginaSK = pytz.timezone("America/Regina")
        now = datetime.now(reginaSK)
        dateStr = now.strftime("%-d.%-m.%y.%-H.%-M")

        parsedURL = method.parseURLForData(url)

        print(parsedURL)

        query = f"INSERT INTO telemetry(timestamp,accx,accy,accz,gyrx,gyry,gyrz) VALUES('{dateStr}','{parsedURL[0]}','{parsedURL[1]}','{parsedURL[2]}','{parsedURL[3]}','{parsedURL[4]}','{parsedURL[5]}');"

        curr.execute(query)

        conn.commit()

        conn.close()

        return {'inserted' : {'accX': accX,'accY': accY, 'accZ': accZ,'gyrX': gyrX,'gyrY': gyrY,'gyrZ':gyrZ}}



api.add_resource(create_record,"/api/create/<string:url>")


if __name__ == "__main__":
    print("[LOG] App __init__")
    app.run()
