from flask import Blueprint, render_template, request, redirect, url_for
from flask_cors import cross_origin

views = Blueprint('views',__name__)

@views.route('/')
def home():
    return {"message" : "Hello World! from home"}

@views.route("/api",methods=["GET","POST"])
@cross_origin()
def API():
    return {"message" : "Hello World! from the API"}

