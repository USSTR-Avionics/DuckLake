from flask_cors import CORS
from flask import Flask
import secrets
import os

def create_app():

    app = Flask(__name__)

    app.config['SECRET_KEY'] = secrets.token_hex(16)
    app.config['SQLALCHEMY_DATABASE_URI'] = os.environ.get("DATABASE_URL")
    app.config["SQLALCHEMY_TRACK_MODIFICATIONS"] = False

    CORS(app,resources={r'/*' : {'origins' : '*'}})

    from .views import views

    app.register_blueprint(views,url_prefix="/")

    return app
