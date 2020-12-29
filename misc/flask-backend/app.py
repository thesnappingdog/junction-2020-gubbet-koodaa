from flask import Flask, send_from_directory, send_file
app = Flask(__name__)

@app.route('/')
def index():
    return send_file("index.html")

if __name__ == '__main__':
    app.run(ssl_context='adhoc', debug=True)
