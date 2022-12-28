from flask import Flask, render_template, flash, redirect, request
import uuid
from flask_bootstrap import Bootstrap
from dotenv import load_dotenv
from datetime import datetime
import os
from werkzeug.security import generate_password_hash, check_password_hash

# Load the environment variables, this module searches for .env first, but you can directly name the file if desired.
load_dotenv()
app = Flask(__name__)
# Bootstrap. Love it.
boostrap = Bootstrap(app)

# CSRF token
# app.config['SECRET_KEY'] = uuid.uuid1()

@app.route('/')
def main():
    title = "Cryo's Blockchain managed electronics e-retailer!"
    return render_template('main.html', title=title, active=True)

@app.route('/store')
def store():
    title = "Cryo's E-lectronics Superstore!"
    return render_template('main.html', title=title, active=True)

@app.route('/inventory/add', methods = ['GET'])
def add_item():
    title = "Add new item to inventory"
    return render_template('main.html', title=title, active=True)

@app.route('/inventory/manage', methods = ['GET'])
def manage_inventory():
    title = "Adjust quantity of item in inventory"
    return render_template('main.html', title=title, active=True)

if __name__ == '__main__':
    app.run(debug=True)